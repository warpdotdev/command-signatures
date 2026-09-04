#!/usr/bin/env node
// Convert withfig/autocomplete nested TypeScript specs into JSON assets that
// rust-embed can look up by slash path (`aws/s3.json`, `gcloud/compute.json`).
// Starts from aws.json/gcloud.json and follows every static slash-path loadSpec
// discovered in imported output.
//
// Usage:
//   node script/import_fig_nested_specs.mjs /path/to/withfig/autocomplete
//
// Requires the `esbuild` package to be importable (see script/import_fig_nested_specs).

import { createRequire } from "node:module";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { reachableSlashPathClosure } from "./import_fig_nested_specs_lib.mjs";

const require = createRequire(import.meta.url);
const esbuild = require("esbuild");

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const jsonRoot = path.join(repoRoot, "command-signatures/json");

function specFromModule(mod) {
  if (mod && typeof mod === "object" && "default" in mod && mod.default) {
    return mod.default;
  }
  return mod;
}

function serializeSpec(spec) {
  return `${JSON.stringify(
    spec,
    (_key, value) =>
      typeof value === "function"
        ? `_NuFrRa_${Function.prototype.toString.call(value)}`
        : value,
    4,
  )}\n`;
}

async function convertSpec(tsPath) {
  const result = await esbuild.build({
    absWorkingDir: path.dirname(tsPath),
    entryPoints: [tsPath],
    bundle: true,
    format: "cjs",
    platform: "node",
    write: false,
    logLevel: "silent",
  });
  const tmp = path.join(
    os.tmpdir(),
    `fig-spec-${process.pid}-${Math.random().toString(16).slice(2)}.cjs`,
  );
  fs.writeFileSync(tmp, result.outputFiles[0].text);
  try {
    const spec = specFromModule(require(tmp));
    delete require.cache[require.resolve(tmp)];
    return spec;
  } finally {
    fs.unlinkSync(tmp);
  }
}

async function main() {
  const autocompleteRoot = process.argv[2];
  if (!autocompleteRoot) {
    console.error(
      "usage: node script/import_fig_nested_specs.mjs /path/to/withfig/autocomplete",
    );
    process.exit(2);
  }

  const srcRoot = path.join(path.resolve(autocompleteRoot), "src");
  const seedSpecs = ["aws.json", "gcloud.json"].map((name) =>
    JSON.parse(fs.readFileSync(path.join(jsonRoot, name), "utf8")),
  );

  const converted = [];
  const failed = [];
  const { missing } = await reachableSlashPathClosure(seedSpecs, async (pointer) => {
    const tsPath = path.join(srcRoot, `${pointer}.ts`);
    const outPath = path.join(jsonRoot, `${pointer}.json`);
    if (!fs.existsSync(tsPath)) {
      return null;
    }
    try {
      const spec = await convertSpec(tsPath);
      fs.mkdirSync(path.dirname(outPath), { recursive: true });
      fs.writeFileSync(outPath, serializeSpec(spec));
      converted.push(pointer);
      console.log(`wrote ${path.relative(repoRoot, outPath)}`);
      return spec;
    } catch (error) {
      failed.push(`${pointer}: ${error.message ?? error}`);
      console.error(`failed ${pointer}:`, error);
      return null;
    }
  });

  console.log(
    `converted ${converted.length}; missing ${missing.length}; failed ${failed.length}`,
  );
  if (missing.length) {
    console.log("missing:", missing.join(", "));
  }
  if (failed.length) {
    console.error(failed.join("\n"));
    process.exit(1);
  }
}

await main();
