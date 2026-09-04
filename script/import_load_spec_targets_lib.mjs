// Helpers for importing nested slash-path loadSpec command specs.

export const DESCRIPTION_MAX_CHARS = 240;

const SUPPORTED_KEYS = [
  "name",
  "description",
  "subcommands",
  "options",
  "args",
  "suggestions",
  "generatorName",
  "template",
  "parserDirectives",
  "loadSpec",
  "aliasGenerator",
  "additionalSuggestions",
  "isDangerous",
  "priority",
  "hidden",
  "isPersistent",
  "isRequired",
  "requiresEquals",
  "isRepeatable",
  "exclusiveOn",
  "dependsOn",
  "skipGeneratorValidation",
  "filterTemplateName",
  "isVariadic",
  "isOptional",
  "isCommand",
  "default",
  "displayName",
  "type",
];

const BOILERPLATE_DESCRIPTIONS = new Set([
  "string",
  "list",
  "dict",
  "integer",
  "boolean",
  "number",
  "float",
]);

export function collapseWhitespace(value) {
  return String(value).replace(/\s+/g, " ").trim();
}

export function firstSentence(value) {
  const text = collapseWhitespace(value);
  const match = text.match(/^.*?[.!?](?=\s|$)/);
  return match ? match[0] : text;
}

export function leadingStatement(value) {
  const text = collapseWhitespace(value);
  const colonList = text.match(/^(.*?):(?:\s{2,}|\s+(?:Amazon|AWS|-|•))/);
  if (colonList && colonList[1].trim().length >= 12) {
    return colonList[1].trim();
  }
  return firstSentence(text);
}

export function truncateAtWordBoundary(value, max = DESCRIPTION_MAX_CHARS) {
  if (value.length <= max) {
    return value;
  }
  const budget = Math.max(1, max - 3);
  const slice = value.slice(0, budget);
  const space = slice.lastIndexOf(" ");
  const base = (space > 0 ? slice.slice(0, space) : slice).replace(
    /[.,;:]+$/,
    "",
  );
  return `${base}...`;
}

export function sanitizeDescription(value) {
  if (typeof value !== "string") {
    return undefined;
  }
  const statement = leadingStatement(value);
  if (!statement) {
    return undefined;
  }
  const truncated = truncateAtWordBoundary(statement);
  if (!truncated || BOILERPLATE_DESCRIPTIONS.has(truncated.toLowerCase())) {
    return undefined;
  }
  return truncated;
}

export function isDroppedValue(value) {
  return (
    typeof value === "function" ||
    (typeof value === "string" && value.startsWith("_NuFrRa_"))
  );
}

function isEmptyContainer(value) {
  return (
    (Array.isArray(value) && value.length === 0) ||
    (value != null &&
      typeof value === "object" &&
      !Array.isArray(value) &&
      Object.keys(value).length === 0)
  );
}

export function sanitizeSpec(value) {
  if (isDroppedValue(value)) {
    return undefined;
  }
  if (Array.isArray(value)) {
    return value
      .map((item) => sanitizeSpec(item))
      .filter((item) => item !== undefined && !isEmptyContainer(item));
  }
  if (value != null && typeof value === "object") {
    const out = {};
    for (const key of SUPPORTED_KEYS) {
      if (!Object.hasOwn(value, key)) {
        continue;
      }
      const raw = value[key];
      if (isDroppedValue(raw)) {
        continue;
      }
      if (key === "description") {
        const description = sanitizeDescription(raw);
        if (description) {
          out.description = description;
        }
        continue;
      }
      if (key === "loadSpec") {
        if (
          typeof raw === "string" &&
          raw.length > 0 &&
          !raw.startsWith("_NuFrRa_")
        ) {
          out.loadSpec = raw;
        }
        continue;
      }
      if (
        key === "parserDirectives" &&
        raw != null &&
        typeof raw === "object" &&
        !Array.isArray(raw)
      ) {
        const directives = {};
        for (const [directiveKey, directiveValue] of Object.entries(raw)) {
          if (!isDroppedValue(directiveValue)) {
            directives[directiveKey] = directiveValue;
          }
        }
        if (Object.keys(directives).length > 0) {
          out.parserDirectives = directives;
        }
        continue;
      }
      const sanitized = sanitizeSpec(raw);
      if (sanitized === undefined || isEmptyContainer(sanitized)) {
        continue;
      }
      out[key] = sanitized;
    }
    return out;
  }
  return value;
}

export function serializeSpec(spec) {
  return `${JSON.stringify(spec, null, 4)}\n`;
}

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
