import { column, Schema, TableV2 } from "@powersync/web";

const item_categories = new TableV2({
  name: column.text,
  description: column.text,
  status: column.text,
});

const items = new TableV2({
  name: column.text,
  description: column.text,
  code: column.text,
  sku: column.text,
  nature: column.text,
  uom: column.text,
  mrp: column.integer,
  price_includes_tax: column.integer,
  item_category_id: column.text
});

export const AppSchema = new Schema({
  item_categories,
  items
});

export type Database = (typeof AppSchema)["types"];
export type Category = Database["item_categories"];
export type Item = Database["items"];
