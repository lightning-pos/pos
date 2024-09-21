import migrations from '@/lib/pglite/migrations/migrations.json' assert { type: 'json' };
import { column } from '@powersync/common';
import { sql } from 'drizzle-orm';
import { PgliteDatabase } from 'drizzle-orm/pglite';

export const migrate = async (drizzle: PgliteDatabase) => {
  const migrationsTable = '__drizzle_migrations';

  // Create migrations table if it doesn't exist
  await drizzle.execute(sql`
    CREATE TABLE IF NOT EXISTS ${sql.identifier(migrationsTable)} (
        id SERIAL PRIMARY KEY,
        hash TEXT NOT NULL,
        tag TEXT NOT NULL,
        created_at NUMERIC
      )`);

  // Fetch the last migration
  const dbMigrations = await drizzle.execute(sql`
    SELECT id, hash, created_at
    FROM ${sql.identifier(migrationsTable)}
    ORDER BY created_at DESC
    LIMIT 1
  `);

  console.log('dbMigrations', dbMigrations);

  const lastDbMigration = dbMigrations.rows[0];

  console.log('lastDbMigration', lastDbMigration);
  for (const migration of migrations) {
    if (!lastDbMigration || Number(lastDbMigration.created_at) < migration.when) {
      for (const stmt of migration.sql) {
        await drizzle.execute(sql.raw(stmt));
      }

      await drizzle.execute(sql`
        INSERT INTO ${sql.identifier(migrationsTable)} (hash, created_at, tag)
        VALUES (${migration.hash}, ${migration.when}, ${migration.tag})
      `);
    }
  }
};
