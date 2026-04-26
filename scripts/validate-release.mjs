import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import path from 'node:path';
import process from 'node:process';

const rootDir = path.dirname(fileURLToPath(new URL('../package.json', import.meta.url)));

function readJson(relativePath) {
   const filePath = path.join(rootDir, relativePath),
      content = readFileSync(filePath, 'utf8');

   return JSON.parse(content);
}

function readText(relativePath) {
   const filePath = path.join(rootDir, relativePath);

   return readFileSync(filePath, 'utf8');
}

function extractCargoVersion(relativePath) {
   const filePath = path.join(rootDir, relativePath),
      content = readText(relativePath),
      match = content.match(/^version = "([^"]+)"/m);

   if (!match) {
      throw new Error(`Could not find a package version in ${filePath}.`);
   }

   return match[1];
}

function extractDeviceAiDependencyVersion() {
   const filePath = path.join(rootDir, 'Cargo.toml'),
      content = readText('Cargo.toml'),
      dependencyLine = content.split('\n').find((line) => {
         return line.startsWith('device-ai = {');
      });

   if (!dependencyLine) {
      throw new Error(`Could not find the device-ai dependency in ${filePath}.`);
   }

   if (!dependencyLine.includes('path = "crates/device-ai"')) {
      throw new Error(`The device-ai dependency in ${filePath} must point at crates/device-ai.`);
   }

   const versionMatch = dependencyLine.match(/version = "([^"]+)"/);

   if (!versionMatch) {
      throw new Error(`The device-ai dependency in ${filePath} must declare a version.`);
   }

   return versionMatch[1];
}

function normalizeTagVersion(tagValue) {
   if (!tagValue) {
      return null;
   }

   const normalizedTag = tagValue.replace(/^refs\/tags\//u, '');

   if (!/^v\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?$/u.test(normalizedTag)) {
      throw new Error(`Release tags must look like vX.Y.Z. Received: ${tagValue}`);
   }

   return normalizedTag.slice(1);
}

function main() {
   const packageVersion = readJson('package.json').version,
      pluginCargoVersion = extractCargoVersion('Cargo.toml'),
      deviceAiVersion = extractCargoVersion(path.join('crates', 'device-ai', 'Cargo.toml')),
      deviceAiDependencyVersion = extractDeviceAiDependencyVersion(),
      tagVersion = normalizeTagVersion(process.argv[2]),
      mismatches = [];

   if (packageVersion !== pluginCargoVersion) {
      mismatches.push(
         `package.json version ${packageVersion} does not match Cargo.toml version ${pluginCargoVersion}.`,
      );
   }

   if (packageVersion !== deviceAiVersion) {
      mismatches.push(
         `package.json version ${packageVersion} does not match crates/device-ai/Cargo.toml version ${deviceAiVersion}.`,
      );
   }

   if (deviceAiDependencyVersion !== deviceAiVersion) {
      mismatches.push(
         `Cargo.toml depends on device-ai ${deviceAiDependencyVersion}, but crates/device-ai/Cargo.toml is ${deviceAiVersion}.`,
      );
   }

   if (tagVersion && tagVersion !== packageVersion) {
      mismatches.push(`Release tag version ${tagVersion} does not match repository version ${packageVersion}.`);
   }

   if (mismatches.length > 0) {
      console.error('Release validation failed:\n');
      mismatches.forEach((mismatch) => {
         console.error(`- ${mismatch}`);
      });
      process.exit(1);
   }

   console.log(`Release validation passed for version ${packageVersion}.`);
}

main();
