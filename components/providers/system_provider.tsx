'use client';

import { AppSchema, Database } from '@/lib/powersync/app_schema';
import { Connector } from '@/lib/powersync/connector';
import { PowerSyncContext } from '@powersync/react';
import { PowerSyncDatabase } from '@powersync/web';
import { Loading } from '@carbon/react';
import Logger from 'js-logger';
import React, { Suspense } from 'react';
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

// TODO: Enable once the backend for connector is ready
// const connector = new Connector();
// powerSync.connect(connector);

export const SystemProvider = ({ children }: { children: React.ReactNode }) => {
  return (
    <Suspense fallback={<Loading />}>
      <PowerSyncContext.Provider value={powerSyncDb}>{children}</PowerSyncContext.Provider>
    </Suspense>
  );
};

export default SystemProvider;
