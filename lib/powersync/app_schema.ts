import { column, Schema, TableV2 } from "@powersync/web";

const categories = new TableV2({
  name: column.text,
  description: column.text,
});

export const AppSchema = new Schema({
  categories
});

export type Database = (typeof AppSchema)["types"];
export type Categories = Database["categories"];
