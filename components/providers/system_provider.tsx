'use client';

import { drizzle } from 'drizzle-orm/pglite';
import { PGlite } from '@electric-sql/pglite';
import { AppSchema, Database } from '@/lib/powersync/app_schema';
import { Connector } from '@/lib/powersync/connector';
import { PowerSyncContext } from '@powersync/react';
import { PowerSyncDatabase } from '@powersync/web';
import { Loading } from '@carbon/react';
import Logger from 'js-logger';
import React, { Suspense, useEffect } from 'react';
import { wrapPowerSyncWithKysely } from '@powersync/kysely-driver';
import { migrate } from '@/lib/pglite/migrator';

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
  useEffect(() => {
    const initializeDatabases = async () => {
      try {
        await migrate(drizzleDb);
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
