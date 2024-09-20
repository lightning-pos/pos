'use client';

import { drizzle } from 'drizzle-orm/pglite';
import { PGlite } from '@electric-sql/pglite';
// import { migrate } from 'drizzle-orm/pglite/migrator';
import { AppSchema, Database } from '@/lib/powersync/app_schema';
import { Connector } from '@/lib/powersync/connector';
import { PowerSyncContext } from '@powersync/react';
import { PowerSyncDatabase } from '@powersync/web';
import { Loading } from '@carbon/react';
import Logger from 'js-logger';
import React, { Suspense, useEffect } from 'react';
import { wrapPowerSyncWithKysely } from '@powersync/kysely-driver';

// eslint-disable-next-line react-hooks/rules-of-hooks
Logger.useDefaults();
Logger.setLevel(Logger.DEBUG);

export const powerSyncDb = new PowerSyncDatabase({
  database: { dbFilename: 'powersync2.db' },
  schema: AppSchema,
  flags: { disableSSRWarning: false }
});

export const db = wrapPowerSyncWithKysely<Database>(powerSyncDb);

export const pgliteDb = new PGlite('idb://minnal');
export const drizzleDb = drizzle(pgliteDb);

export const SystemProvider = ({ children }: { children: React.ReactNode }) => {
  // Run migrations on the PGLite database
  useEffect(() => {
    const initializeDatabases = async () => {
      try {
        Logger.info('Running migrations');
        // await migrate(drizzleDb, { migrationsFolder: './lib/pglite/migrations' });
        Logger.info('Migrations completed');
      } catch (error) {
        Logger.error('Migration failed:', error);
      }
    };

    initializeDatabases();
  }, []);


  return (
    <Suspense fallback={<Loading />}>
      <PowerSyncContext.Provider value={powerSyncDb}>{children}</PowerSyncContext.Provider>
    </Suspense>
  );
};

export default SystemProvider;
