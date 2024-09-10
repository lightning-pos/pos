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
  price: column.integer,
  uom: column.text,
  mrp: column.integer,
  price_includes_tax: column.integer,
  item_category_id: column.text,
  tax_ids: column.text, // Changed from json to text
});

const orders = new TableV2({
  id: column.text,
  total_amount: column.integer,
  payment_method: column.text,
  created_at: column.integer,
  status: column.text,
  subtotal: column.integer,
  tax: column.integer,
});

const order_items = new TableV2({
  id: column.text,
  order_id: column.text,
  item_id: column.text,
  item_name: column.text,
  quantity: column.integer,
  price: column.integer,
  tax: column.integer,
});

const taxes = new TableV2({
  id: column.text,
  name: column.text,
  rate: column.integer,
  description: column.text,
});

const customers = new TableV2({
  name: column.text,
  email: column.text,
  phone_number: column.text,
  country_code: column.text,
});

export const AppSchema = new Schema({
  item_categories,
  items,
  orders,
  order_items, // Add this line
  taxes,
  customers,
});

export type Database = (typeof AppSchema)["types"];
export type Category = Database["item_categories"];
export type Item = Database["items"];
export type Order = Database["orders"];
export type OrderItem = Database["order_items"]; // Add this line
export type Tax = Database["taxes"];
export type Customer = Database["customers"];
