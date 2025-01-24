import { defineConfig } from "drizzle-kit";

export default defineConfig({
  dialect: "sqlite",
  schema: "./lib/db/sqlite/schema.ts",
  out: "./src-tauri/migrations",
  dbCredentials: { url: "/root/.config/com.lightning.pos/minnal.db" },
  verbose: false,
  strict: true,
});
