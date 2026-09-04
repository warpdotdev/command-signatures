#!/usr/bin/env node
// Convert withfig/autocomplete nested TypeScript specs into JSON assets that
// rust-embed can look up by slash path (`aws/s3.json`, `gcloud/compute.json`).
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

const require = createRequire(import.meta.url);
const esbuild = require("esbuild");

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const jsonRoot = path.join(repoRoot, "command-signatures/json");

function collectLoadSpecs(value, out = []) {
  if (Array.isArray(value)) {
    for (const item of value) {
      collectLoadSpecs(item, out);
    }
    return out;
  }
  if (value && typeof value === "object") {
    if (typeof value.loadSpec === "string" && value.loadSpec.includes("/")) {
      out.push(value.loadSpec);
    }
    for (const nested of Object.values(value)) {
      collectLoadSpecs(nested, out);
    }
  }
  return out;
}

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
    return serializeSpec(spec);
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
  const pointerFiles = ["aws.json", "gcloud.json"].map((name) =>
    path.join(jsonRoot, name),
  );
  const pointers = [
    ...new Set(
      pointerFiles.flatMap((file) =>
        collectLoadSpecs(JSON.parse(fs.readFileSync(file, "utf8"))),
      ),
    ),
  ].sort();

  const converted = [];
  const missing = [];
  const failed = [];

  for (const pointer of pointers) {
    const tsPath = path.join(srcRoot, `${pointer}.ts`);
    const outPath = path.join(jsonRoot, `${pointer}.json`);
    if (!fs.existsSync(tsPath)) {
      missing.push(pointer);
      continue;
    }
    try {
      const json = await convertSpec(tsPath);
      fs.mkdirSync(path.dirname(outPath), { recursive: true });
      fs.writeFileSync(outPath, json);
      converted.push(pointer);
      console.log(`wrote ${path.relative(repoRoot, outPath)}`);
    } catch (error) {
      failed.push(`${pointer}: ${error.message ?? error}`);
      console.error(`failed ${pointer}:`, error);
    }
  }

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
