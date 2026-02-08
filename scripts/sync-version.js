/**
 * Sync version between package.json and Cargo.toml
 * Usage: node scripts/sync-version.js [version]
 * Example: node scripts/sync-version.js 1.0.0
 */

import { readFileSync, writeFileSync } from "fs";
import { resolve, dirname } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));
const rootDir = resolve(__dirname, "..");

const packageJsonPath = resolve(rootDir, "package.json");
const cargoTomlPath = resolve(rootDir, "src-tauri", "Cargo.toml");

// Get new version from args or read from package.json
let newVersion = process.argv[2];

if (!newVersion) {
  const pkg = JSON.parse(readFileSync(packageJsonPath, "utf-8"));
  console.log(`Current version: ${pkg.version}`);
  console.log("Usage: npm run version:sync <new-version>");
  console.log("Example: npm run version:sync 1.0.0");
  process.exit(0);
}

// Validate version format
if (!/^\d+\.\d+\.\d+$/.test(newVersion)) {
  console.error("Error: Version must be in format X.Y.Z (e.g., 1.0.0)");
  process.exit(1);
}

// Update package.json
const packageJson = JSON.parse(readFileSync(packageJsonPath, "utf-8"));
const oldVersion = packageJson.version;
packageJson.version = newVersion;
writeFileSync(packageJsonPath, JSON.stringify(packageJson, null, 2) + "\n");
console.log(`✓ package.json: ${oldVersion} → ${newVersion}`);

// Update Cargo.toml
let cargoToml = readFileSync(cargoTomlPath, "utf-8");
cargoToml = cargoToml.replace(
  /^version = "[\d.]+"$/m,
  `version = "${newVersion}"`,
);
writeFileSync(cargoTomlPath, cargoToml);
console.log(`✓ Cargo.toml: ${oldVersion} → ${newVersion}`);

console.log(`\n* Version synced to ${newVersion}`);
