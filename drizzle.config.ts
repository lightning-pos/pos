import { defineConfig } from "drizzle-kit";

export default defineConfig({
  dialect: "sqlite",
  schema: "./lib/db/sqlite/schema.ts",
  out: "./src-tauri/migrations",
  verbose: false,
  strict: true,
});
