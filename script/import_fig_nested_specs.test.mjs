import { test } from "node:test";
import assert from "node:assert/strict";
import {
  collectStaticSlashPathLoadSpecs,
  reachableSlashPathClosure,
  uniqueSorted,
} from "./import_fig_nested_specs_lib.mjs";

test("ignores function-style and non-slash loadSpec values", () => {
  const spec = {
    loadSpec: "flutter",
    subcommands: [
      { loadSpec: "_NuFrRa_() => 'aws/s3'" },
      { loadSpec: "aws/iam" },
      { loadSpec: { custom: true } },
      { loadSpec: "" },
    ],
  };
  assert.deepEqual(collectStaticSlashPathLoadSpecs(spec), ["aws/iam"]);
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
  assert.deepEqual(result.missing, []);
  assert.deepEqual(convertedOrder, ["aws/a", "aws/b", "aws/c"]);
  assert.deepEqual(uniqueSorted(convertedOrder), convertedOrder);
});

test("missing targets are reported and do not enqueue children", async () => {
  const result = await reachableSlashPathClosure(
    [{ loadSpec: "gcloud/alpha" }, { loadSpec: "gcloud/auth" }],
    (name) => (name === "gcloud/auth" ? { name: "auth" } : null),
  );
  assert.deepEqual(result.converted, ["gcloud/auth"]);
  assert.deepEqual(result.missing, ["gcloud/alpha"]);
});

test("newly discovered nested refs are converted after their parent", async () => {
  const catalog = {
    "aws/root": { subcommands: [{ loadSpec: "aws/child" }] },
    "aws/child": { subcommands: [{ loadSpec: "aws/leaf" }] },
    "aws/leaf": { name: "leaf" },
  };
  const result = await reachableSlashPathClosure(
    [{ subcommands: [{ loadSpec: "aws/root" }] }],
    (name) => catalog[name] ?? null,
  );
  assert.deepEqual(result.converted, ["aws/root", "aws/child", "aws/leaf"]);
});

test("sibling refs are converted in sorted order", async () => {
  const catalog = {
    "aws/root": {
      subcommands: [{ loadSpec: "aws/zebra" }, { loadSpec: "aws/apple" }],
    },
    "aws/apple": { name: "apple" },
    "aws/zebra": { name: "zebra" },
  };
  const result = await reachableSlashPathClosure(
    [{ subcommands: [{ loadSpec: "aws/root" }] }],
    (name) => catalog[name] ?? null,
  );
  assert.deepEqual(result.converted, ["aws/root", "aws/apple", "aws/zebra"]);
});
