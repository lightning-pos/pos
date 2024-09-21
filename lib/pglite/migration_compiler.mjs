/**
 * This script compiles the Drizzle migrations into a format that can be processed by a webworker
 * This allows for migrations to be shipped and ran within the application
 */

import crypto from 'node:crypto';
import fs from 'node:fs';
import journal from './migrations/meta/_journal.json' with { type: 'json' };

const migrate = [];

for (let index = 0; index < journal.entries.length; index++) {
  const { when, idx, tag } = journal.entries[index];

  console.log(`parsing ${tag}`);
  const migrationFile = fs
    .readFileSync(`./lib/pglite/migrations/${tag}.sql`)
    .toString();

  migrate.push({
    idx,
    when,
    tag,
    hash: crypto.createHash('sha256').update(migrationFile).digest('hex'),
    sql: migrationFile
      .replace(/\n\t?/g, '')
      .split('--> statement-breakpoint')
      .map((x) => x.trim()),
  });
}

fs.writeFileSync(
  './lib/pglite/migrations/migrations.json',
  JSON.stringify(migrate, null, 2)
);
