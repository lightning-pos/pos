'use client';
import Database from "@tauri-apps/plugin-sql";
import { drizzle, SqliteRemoteDatabase } from "drizzle-orm/sqlite-proxy";
import * as schema from "@/lib/db/sqlite/schema";
import React, { createContext, useContext, useEffect, useState } from "react";

/**
 * Represents the result of a SELECT query.
 */
export type SelectQueryResult = {
  [key: string]: any;
};

/**
 * Checks if the given SQL query is a SELECT query.
 * @param sql The SQL query to check.
 * @returns True if the query is a SELECT query, false otherwise.
 */
function isSelectQuery(sql: string): boolean {
  const selectRegex = /^\s*SELECT\b/i;
  return selectRegex.test(sql);
}

export const DrizzleContext = createContext<SqliteRemoteDatabase<typeof schema> | null>(null);

export const DrizzleProvider = ({ children }: { children: React.ReactNode }) => {
  const [sqlite, setSqlite] = useState<Database>()
  const [db, setDb] = useState<SqliteRemoteDatabase<typeof schema>>()
  useEffect(() => {
    console.log("Loading DB")
    const loadDb = async () => {
      // Load sqlite using tauri proxy
      const sqlite = Database.get("sqlite:minnal.db");
      setSqlite(sqlite)
    }
    loadDb()
  }, [])

  useEffect(() => {
    const loadDrizzle = async () => {
      if (!sqlite) { return }
      let db = drizzle<typeof schema>(
        async (sql, params, method) => {
          let rows: any = [];
          let results = [];

          // If the query is a SELECT, use the select method
          if (isSelectQuery(sql)) {
            rows = await sqlite.select(sql, params).catch((e: any) => {
              console.error("SQL Error:", e);
              return [];
            });
          } else {
            // Otherwise, use the execute method
            rows = await sqlite.execute(sql, params).catch((e: any) => {
              console.error("SQL Error:", e);
              return [];
            });
            return { rows: [] };
          }

          rows = rows.map((row: any) => {
            return Object.values(row);
          });

          // If the method is "all", return all rows
          results = method === "all" ? rows : rows[0];

          return { rows: results };
        },
        // Pass the schema to the drizzle instance
        { schema: schema, logger: true }
      )
      setDb(db)
    }
    loadDrizzle()
  }, [sqlite])

  return (
    <>
      {db ? (
        <DrizzleContext.Provider value={db}>{children}</DrizzleContext.Provider>
      ) : (
        <div>DB connection failed...</div>
      )}
    </>
  );
};

export const useDb = () => {
  const db = useContext(DrizzleContext)

  if (!db) {
    throw new Error('useDb must be used within a DrizzleProvider')
  }

  return db
}

export default DrizzleProvider;
