// Pure helpers for importing nested Fig loadSpec targets.
// Slash-path string loadSpecs are followed; function-style `_NuFrRa_` values are ignored.

export function collectStaticSlashPathLoadSpecs(value, out = []) {
  if (Array.isArray(value)) {
    for (const item of value) {
      collectStaticSlashPathLoadSpecs(item, out);
    }
    return out;
  }
  if (value && typeof value === "object") {
    if (typeof value.loadSpec === "string") {
      const target = value.loadSpec;
      if (
        target.includes("/") &&
        target.length > 0 &&
        !target.startsWith("_NuFrRa_")
      ) {
        out.push(target);
      }
    }
    for (const nested of Object.values(value)) {
      collectStaticSlashPathLoadSpecs(nested, out);
    }
  }
  return out;
}

export function uniqueSorted(values) {
  return [...new Set(values)].sort();
}

// Walk seed specs, converting each slash-path target and enqueueing static
// slash-path loadSpecs found in converted output. `getSpec(name)` may be async
// and should return a parsed spec object or null if the target cannot be produced.
export async function reachableSlashPathClosure(seedSpecs, getSpec) {
  const queue = uniqueSorted(
    seedSpecs.flatMap((spec) => collectStaticSlashPathLoadSpecs(spec)),
  );
  const seen = new Set();
  const converted = [];
  const missing = [];

  while (queue.length > 0) {
    const name = queue.shift();
    if (seen.has(name)) {
      continue;
    }
    seen.add(name);

    const spec = await getSpec(name);
    if (spec == null) {
      missing.push(name);
      continue;
    }

    converted.push(name);
    for (const ref of uniqueSorted(collectStaticSlashPathLoadSpecs(spec))) {
      if (!seen.has(ref)) {
        queue.push(ref);
      }
    }
    queue.sort();
  }

  return { converted, missing };
}
