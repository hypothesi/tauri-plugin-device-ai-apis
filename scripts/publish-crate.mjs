import { readFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import path from 'node:path';
import process from 'node:process';

const rootDir = path.dirname(fileURLToPath(new URL('../package.json', import.meta.url))),
   CRATE_MANIFESTS = Object.freeze({
      'device-ai': path.join('crates', 'device-ai', 'Cargo.toml'),
      'tauri-plugin-device-ai-apis': 'Cargo.toml',
   }),
   PUBLISH_ATTEMPTS = 10,
   RETRY_DELAY_MS = 30000,
   NOT_PUBLISHED_STATUS = 404,
   USER_AGENT = 'tauri-plugin-device-ai-apis-release';

function extractCargoVersion(relativePath) {
   const filePath = path.join(rootDir, relativePath),
      content = readFileSync(filePath, 'utf8'),
      match = content.match(/^version = "([^"]+)"/mu);

   if (!match) {
      throw new Error(`Could not find a package version in ${filePath}.`);
   }

   return match[1];
}

async function isPublished(crateName, version) {
   const response = await fetch(`https://crates.io/api/v1/crates/${crateName}/${version}`, {
      headers: { 'user-agent': USER_AGENT },
   });

   if (response.status === NOT_PUBLISHED_STATUS) {
      return false;
   }

   if (!response.ok) {
      throw new Error(`crates.io returned ${response.status} while checking ${crateName} ${version}.`);
   }

   return true;
}

function runCargoPublish(crateName) {
   const result = spawnSync('cargo', [ 'publish', '--locked', '-p', crateName ], { stdio: 'inherit' });

   return result.status === 0;
}

function wait(durationMS) {
   return new Promise((resolve) => {
      setTimeout(resolve, durationMS);
   });
}

async function main() {
   const crateName = process.argv[2],
      manifestPath = CRATE_MANIFESTS[crateName];

   if (!manifestPath) {
      const knownCrates = Object.keys(CRATE_MANIFESTS).join(', ');

      throw new Error(`Unknown crate ${crateName}. Expected one of: ${knownCrates}.`);
   }

   const version = extractCargoVersion(manifestPath);

   for (let attempt = 1; attempt <= PUBLISH_ATTEMPTS; attempt++) {
      if (await isPublished(crateName, version)) {
         console.log(`${crateName} ${version} is already on crates.io. Skipping publish.`);
         return;
      }

      if (runCargoPublish(crateName)) {
         console.log(`Published ${crateName} ${version}.`);
         return;
      }

      if (attempt < PUBLISH_ATTEMPTS) {
         console.log(`Publish attempt ${attempt} for ${crateName} failed. A dependency may still be`
            + ' propagating through the crates.io index. Retrying in 30 seconds.');
         await wait(RETRY_DELAY_MS);
      }
   }

   throw new Error(`Could not publish ${crateName} ${version} after ${PUBLISH_ATTEMPTS} attempts.`);
}

main().catch((error) => {
   console.error(error.message);
   process.exit(1);
});
