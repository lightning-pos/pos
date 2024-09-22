import { pgTable, integer, text, timestamp, pgEnum } from "drizzle-orm/pg-core";

export const customersTable = pgTable('customers', {
  id: text('id').primaryKey(),
  name: text('name').notNull(),
  email: text('email'),
  phoneNumber: text('phone_number'),
  countryCode: text('country_code'),
  createdAt: timestamp('created_at').defaultNow(),
  updatedAt: timestamp('updated_at').defaultNow(),
});

export const itemCategoriesTable = pgTable('item_categories', {
  id: text('id').primaryKey(),
  name: text('name').notNull(),
  description: text('description'),
  state: text('state'),
  createdAt: timestamp('created_at').defaultNow(),
  updatedAt: timestamp('updated_at').defaultNow(),
});

export const itemsTable = pgTable('items', {
  id: text('id').primaryKey(),
  categoryId: text('category_id').references(() => itemCategoriesTable.id, { onDelete: 'restrict' }),
  name: text('name').notNull(),
  description: text('description'),
  price: integer('price'),
  createdAt: timestamp('created_at').defaultNow(),
  updatedAt: timestamp('updated_at').defaultNow(),
});

export const itemTaxesTable = pgTable('item_taxes', {
  id: text('id').primaryKey(),
  itemId: text('item_id').references(() => itemsTable.id, { onDelete: 'cascade' }),
  taxId: text('tax_id').references(() => taxesTable.id, { onDelete: 'restrict' }),
  createdAt: timestamp('created_at').defaultNow(),
  updatedAt: timestamp('updated_at').defaultNow(),
});

export const orderStateEnum = pgEnum('order_state', ['open', 'closed', 'cancelled']);
export const ordersTable = pgTable('orders', {
  id: text('id').primaryKey(),
  customerId: text('customer_id').references(() => customersTable.id, { onDelete: 'restrict' }),
  customerName: text('customer_name'),
  customerPhoneNumber: text('customer_phone_number'),
  orderDate: timestamp('order_date').defaultNow(),
  netAmount: integer('net_amount'),
  discAmount: integer('disc_amount'),
  taxAmount: integer('tax_amount'),
  totalAmount: integer('total_amount'),
  state: orderStateEnum('order_state'),
  createdAt: timestamp('created_at').defaultNow(),
  updatedAt: timestamp('updated_at').defaultNow(),
});

export const orderItemsTable = pgTable('order_items', {
  id: text('id').primaryKey(),
  orderId: text('order_id').references(() => ordersTable.id, { onDelete: 'cascade' }),
  itemId: text('item_id').references(() => itemsTable.id, { onDelete: 'restrict' }),
  itemName: text('item_name'),
  quantity: integer('quantity'),
  priceAmount: integer('price_amount'),
  taxAmount: integer('tax_amount'),
  createdAt: timestamp('created_at').defaultNow(),
  updatedAt: timestamp('updated_at').defaultNow(),
});

export const taxesTable = pgTable('taxes', {
  id: text('id').primaryKey(),
  name: text('name').notNull(),
  rate: integer('rate').notNull(),
  description: text('description'),
  createdAt: timestamp('created_at').defaultNow(),
  updatedAt: timestamp('updated_at').defaultNow(),
});

// Add these type definitions at the end of the file
export type Customer = typeof customersTable.$inferSelect;
export type NewCustomer = typeof customersTable.$inferInsert;

export type ItemCategory = typeof itemCategoriesTable.$inferSelect;
export type NewItemCategory = typeof itemCategoriesTable.$inferInsert;

export type Item = typeof itemsTable.$inferSelect;
export type NewItem = typeof itemsTable.$inferInsert;

export type Order = typeof ordersTable.$inferSelect;
export type NewOrder = typeof ordersTable.$inferInsert;

export type OrderItem = typeof orderItemsTable.$inferSelect;
export type NewOrderItem = typeof orderItemsTable.$inferInsert;

export type Tax = typeof taxesTable.$inferSelect;
export type NewTax = typeof taxesTable.$inferInsert;

export type ItemTax = typeof itemTaxesTable.$inferSelect;
export type NewItemTax = typeof itemTaxesTable.$inferInsert;
