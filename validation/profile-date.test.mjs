import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import test from "node:test";

const repoRoot = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const schema = JSON.parse(
  await readFile(resolve(repoRoot, "schema/profile.schema.json"), "utf8"),
);
const profile = JSON.parse(await readFile(resolve(repoRoot, "profile.json"), "utf8"));
const datePattern = new RegExp(schema.$defs.date.pattern);

test("date schema accepts valid Gregorian dates", () => {
  for (const value of [
    "01-31-2026",
    "02-28-2025",
    "02-29-2024",
    "02-29-0400",
    "02-29-2000",
    "02-29-0004",
    "04-30-2026",
    "12-31-9999",
  ]) {
    assert.match(value, datePattern, `expected valid date: ${value}`);
  }
});

test("date schema rejects impossible or malformed dates", () => {
  for (const value of [
    "02-29-2025",
    "02-31-2026",
    "04-31-2026",
    "13-01-2026",
    "00-01-2026",
    "01-00-2026",
    "01-32-2026",
    "02-29-1900",
    "02-29-2100",
    "01-01-0000",
    "2026-01-31",
    " 02-29-2024",
  ]) {
    assert.doesNotMatch(value, datePattern, `expected invalid date: ${value}`);
  }
});

test("every non-null profile date uses a valid calendar date", () => {
  const dateFields = new Set(["reviewed_on", "source_date", "last_supported_on"]);
  let checked = 0;

  function visit(value, path = "$") {
    if (Array.isArray(value)) {
      value.forEach((item, index) => visit(item, `${path}[${index}]`));
      return;
    }
    if (value === null || typeof value !== "object") return;

    for (const [key, child] of Object.entries(value)) {
      const childPath = `${path}.${key}`;
      if (dateFields.has(key) && child !== null) {
        assert.equal(typeof child, "string", `${childPath} must be a string or null`);
        assert.match(child, datePattern, `${childPath} must be a valid calendar date`);
        checked += 1;
      }
      visit(child, childPath);
    }
  }

  visit(profile);
  assert.ok(checked > 0, "expected to validate dates in profile.json");
});
