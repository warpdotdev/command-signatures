import { test } from "node:test";
import assert from "node:assert/strict";
import fs from "node:fs";
import { fileURLToPath } from "node:url";
import {
  DESCRIPTION_MAX_CHARS,
  reachableSlashPathClosure,
  sanitizeDescription,
  sanitizeSpec,
  serializeSpec,
  uniqueSorted,
} from "./import_load_spec_targets_lib.mjs";

const longResourceIdDescription = fs.readFileSync(
  fileURLToPath(
    new URL(
      "./testdata/application-autoscaling-resource-id-description.txt",
      import.meta.url,
    ),
  ),
  "utf8",
);

test("application-autoscaling delete-scaling-policy --resource-id keeps only that option's leading statement", () => {
  assert.ok(longResourceIdDescription.length > DESCRIPTION_MAX_CHARS);
  assert.equal(
    sanitizeDescription(longResourceIdDescription),
    "The identifier of the resource associated with the scalable target.",
  );
});

test("top-level descriptions stop before a colon-introduced enumeration", () => {
  const raw =
    "With Application Auto Scaling, you can configure automatic scaling for the following resources:   Amazon AppStream 2.0 fleets   Amazon Aurora Replicas";
  assert.equal(
    sanitizeDescription(raw),
    "With Application Auto Scaling, you can configure automatic scaling for the following resources",
  );
});

test("subcommand descriptions stay on that subcommand and are not copied to children", () => {
  const sanitized = sanitizeSpec({
    name: "application-autoscaling",
    description: "Parent service overview that children must not inherit.",
    subcommands: [
      {
        name: "delete-scaling-policy",
        description: "Deletes the specified scaling policy for an Application Auto Scaling scalable target.",
        options: [
          {
            name: "--resource-id",
            description: longResourceIdDescription,
          },
          { name: "--quiet" },
        ],
      },
    ],
  });
  assert.equal(
    sanitized.description,
    "Parent service overview that children must not inherit.",
  );
  assert.equal(
    sanitized.subcommands[0].description,
    "Deletes the specified scaling policy for an Application Auto Scaling scalable target.",
  );
  assert.equal(
    sanitized.subcommands[0].options[0].description,
    "The identifier of the resource associated with the scalable target.",
  );
  assert.equal(sanitized.subcommands[0].options[1].description, undefined);
});

test("argument descriptions stay on that argument", () => {
  const sanitized = sanitizeSpec({
    name: "root",
    description: "Root command.",
    args: {
      name: "PATH",
      description: "Path to the config file. Additional unused documentation follows.",
    },
  });
  assert.equal(sanitized.args.description, "Path to the config file.");
  assert.equal(sanitized.description, "Root command.");
});

test("boilerplate type names are omitted rather than invented", () => {
  assert.equal(sanitizeDescription("String"), undefined);
  assert.equal(sanitizeDescription("  List  "), undefined);
});

test("first sentences longer than the maximum are truncated at a word boundary", () => {
  const sentence = `${"word ".repeat(80).trim()}.`;
  const sanitized = sanitizeDescription(sentence);
  assert.ok(sanitized.endsWith("..."));
  assert.ok(sanitized.length <= DESCRIPTION_MAX_CHARS);
});

test("function-valued and serialized function properties are dropped", () => {
  const sanitized = sanitizeSpec({
    name: "demo",
    generateSpec: "_NuFrRa_() => ({})",
    generators: { script: () => "echo" },
    loadSpec: "_NuFrRa_custom",
    options: [
      {
        name: "--help",
        description: "Show help.",
        insertValue: "_NuFrRa_unused",
      },
    ],
  });
  assert.deepEqual(sanitized, {
    name: "demo",
    options: [{ name: "--help", description: "Show help." }],
  });
});

test("supported static fields are preserved in deterministic key order", () => {
  const sanitized = sanitizeSpec({
    name: "root",
    loadSpec: "aws/child",
    extra: "drop-me",
    parserDirectives: { flagsArePosixNoncompliant: true },
    args: {
      name: "PATH",
      template: "filepaths",
      generatorName: "files",
      filterTemplateName: "only_dirs",
      isOptional: true,
    },
    options: [{ name: "--profile", args: { name: "PROFILE" } }],
    subcommands: [{ name: "inner", suggestions: ["a"] }],
  });
  assert.deepEqual(Object.keys(sanitized), [
    "name",
    "subcommands",
    "options",
    "args",
    "parserDirectives",
    "loadSpec",
  ]);
  assert.equal(sanitized.loadSpec, "aws/child");
  assert.equal(sanitized.args.template, "filepaths");
  assert.equal(sanitized.args.generatorName, "files");
  assert.equal(sanitized.args.filterTemplateName, "only_dirs");
});

test("closure is the sorted reachable set and visits each target once", async () => {
  const catalog = {
    "aws/a": {
      subcommands: [{ loadSpec: "aws/c" }, { loadSpec: "aws/b" }],
    },
    "aws/b": { subcommands: [{ loadSpec: "aws/c" }, { loadSpec: "aws/a" }] },
    "aws/c": { name: "c" },
  };
  const convertedOrder = [];
  const result = await reachableSlashPathClosure(
    [{ loadSpec: "aws/b" }, { loadSpec: "aws/a" }],
    (name) => {
      convertedOrder.push(name);
      return catalog[name] ?? null;
    },
  );
  assert.deepEqual(result.converted, ["aws/a", "aws/b", "aws/c"]);
  assert.deepEqual(convertedOrder, ["aws/a", "aws/b", "aws/c"]);
  assert.deepEqual(uniqueSorted(convertedOrder), convertedOrder);
});

test("sanitized serialization is byte-stable across regeneration", () => {
  const spec = {
    name: "application-autoscaling",
    description: longResourceIdDescription,
    generateSpec: "_NuFrRa_() => ({})",
    options: [
      {
        name: "--resource-id",
        description: longResourceIdDescription,
      },
    ],
  };
  const first = serializeSpec(sanitizeSpec(spec));
  const second = serializeSpec(sanitizeSpec(JSON.parse(first)));
  assert.equal(first, second);
  assert.ok(!first.includes("_NuFrRa_"));
  assert.ok(!first.includes("ECS service - The resource type is service"));
});
