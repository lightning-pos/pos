import { relations, sql } from "drizzle-orm";
import { sqliteTable, integer, text, primaryKey } from "drizzle-orm/sqlite-core";

export const customersTable = sqliteTable('customers', {
  id: text('id').primaryKey(),
  name: text('name').notNull(),
  email: text('email'),
  countryCode: text('country_code'),
  phoneNumber: text('phone_number'),
  createdAt: integer('created_at', { mode: 'timestamp' }).notNull().default(sql`(unixepoch())`),
  updatedAt: integer('updated_at', { mode: 'timestamp' }).notNull().default(sql`(unixepoch())`),
});
export type Customer = typeof customersTable.$inferSelect;
export type NewCustomer = typeof customersTable.$inferInsert;

export const itemCategoriesTable = sqliteTable('item_categories', {
  id: text('id').primaryKey(),
  name: text('name').notNull(),
  description: text('description'),
  state: text('state', { enum: ['active', 'inactive'] }).notNull().default('active'),
  createdAt: integer('created_at', { mode: 'timestamp' }).notNull().default(sql`(unixepoch())`),
  updatedAt: integer('updated_at', { mode: 'timestamp' }).notNull().default(sql`(unixepoch())`),
});
export type ItemCategory = typeof itemCategoriesTable.$inferSelect;
export type NewItemCategory = typeof itemCategoriesTable.$inferInsert;

export const itemCategoryRelations = relations(itemCategoriesTable, ({ many }) => ({
  items: many(itemsTable),
}));

export const itemsTable = sqliteTable('items', {
  id: text('id').primaryKey(),
  categoryId: text('category_id').notNull().references(() => itemCategoriesTable.id),
  name: text('name').notNull(),
  description: text('description'),
  price: integer('price').notNull().default(0),
  createdAt: integer('created_at', { mode: 'timestamp' }).notNull().default(sql`(unixepoch())`),
  updatedAt: integer('updated_at', { mode: 'timestamp' }).notNull().default(sql`(unixepoch())`),
});
export type Item = typeof itemsTable.$inferSelect;
export type NewItem = typeof itemsTable.$inferInsert;

export const itemRelations = relations(itemsTable, ({ one, many }) => ({
  category: one(itemCategoriesTable, {
    fields: [itemsTable.categoryId],
    references: [itemCategoriesTable.id],
  }),
  taxes: many(itemTaxesTable),
}));

export const itemTaxesTable = sqliteTable('item_taxes', {
  itemId: text('item_id').notNull().references(() => itemsTable.id, { onDelete: 'cascade' }),
  taxId: text('tax_id').notNull().references(() => taxesTable.id, { onDelete: 'restrict' }),
}, (t) => ({
  pk: primaryKey({ columns: [t.itemId, t.taxId] }),
}));
export type ItemTax = typeof itemTaxesTable.$inferSelect;
export type NewItemTax = typeof itemTaxesTable.$inferInsert;

export const itemTaxRelations = relations(itemTaxesTable, ({ one }) => ({
  item: one(itemsTable, {
    fields: [itemTaxesTable.itemId],
    references: [itemsTable.id],
  }),
  tax: one(taxesTable, {
    fields: [itemTaxesTable.taxId],
    references: [taxesTable.id],
  }),
}));

export const ordersTable = sqliteTable('orders', {
  id: text('id').primaryKey(),
  customerId: text('customer_id').references(() => customersTable.id),
  customerName: text('customer_name').notNull(),
  customerPhoneNumber: text('customer_phone_number').notNull(),
  orderDate: integer('order_date', { mode: 'timestamp' }).notNull().default(sql`(unixepoch())`),
  netAmount: integer('net_amount').notNull(),
  discAmount: integer('disc_amount').notNull(),
  taxableAmount: integer('taxable_amount').notNull(),
  taxAmount: integer('tax_amount').notNull(),
  totalAmount: integer('total_amount').notNull(),
  state: text('order_state', { enum: ['open', 'closed', 'cancelled'] }).notNull(),
  createdAt: integer('created_at', { mode: 'timestamp' }).notNull().default(sql`(unixepoch())`),
  updatedAt: integer('updated_at', { mode: 'timestamp' }).notNull().default(sql`(unixepoch())`),
});
export type Order = typeof ordersTable.$inferSelect;
export type NewOrder = typeof ordersTable.$inferInsert;

export const orderItemsTable = sqliteTable('order_items', {
  id: text('id').primaryKey(),
  orderId: text('order_id').references(() => ordersTable.id),
  itemId: text('item_id').references(() => itemsTable.id),
  itemName: text('item_name').notNull(),
  quantity: integer('quantity').notNull(),
  priceAmount: integer('price_amount').notNull(),
  taxAmount: integer('tax_amount').notNull(),
  createdAt: integer('created_at', { mode: 'timestamp' }).notNull().default(sql`(unixepoch())`),
  updatedAt: integer('updated_at', { mode: 'timestamp' }).notNull().default(sql`(unixepoch())`),
});
export type OrderItem = typeof orderItemsTable.$inferSelect;
export type NewOrderItem = typeof orderItemsTable.$inferInsert;

export const taxesTable = sqliteTable('taxes', {
  id: text('id').primaryKey(),
  name: text('name').notNull(),
  rate: integer('rate').notNull(),
  description: text('description'),
  createdAt: integer('created_at', { mode: 'timestamp' }).notNull().default(sql`(unixepoch())`),
  updatedAt: integer('updated_at', { mode: 'timestamp' }).notNull().default(sql`(unixepoch())`),
});
export type Tax = typeof taxesTable.$inferSelect;
export type NewTax = typeof taxesTable.$inferInsert;

export const taxRelations = relations(taxesTable, ({ many }) => ({
  items: many(itemTaxesTable),
}));

export const suppliersTable = sqliteTable('suppliers', {
  id: text('id').primaryKey(),
  name: text('name').notNull(),
  email: text('email'),
  countryCode: text('country_code'),
  phoneNumber: text('phone_number'),
  address: text('address'),
  tinNumber: text('tin_number'),
  createdAt: integer('created_at', { mode: 'timestamp' }).notNull().default(sql`(unixepoch())`),
  updatedAt: integer('updated_at', { mode: 'timestamp' }).notNull().default(sql`(unixepoch())`),
});
export type Supplier = typeof suppliersTable.$inferSelect;
export type NewSupplier = typeof suppliersTable.$inferInsert;

export const purchaseOrdersTable = sqliteTable('purchase_orders', {
  id: text('id').primaryKey(),
  supplierId: text('supplier_id').references(() => suppliersTable.id),
  supplierName: text('supplier_name').notNull(),
  orderDate: integer('order_date', { mode: 'timestamp' }).notNull().default(sql`(unixepoch())`),
  netAmount: integer('net_amount').notNull(),
  discAmount: integer('disc_amount').notNull(),
  taxableAmount: integer('taxable_amount').notNull(),
  taxAmount: integer('tax_amount').notNull(),
  totalAmount: integer('total_amount').notNull(),
  state: text('order_state', { enum: ['open', 'closed', 'cancelled'] }).notNull(),
  createdAt: integer('created_at', { mode: 'timestamp' }).notNull().default(sql`(unixepoch())`),
  updatedAt: integer('updated_at', { mode: 'timestamp' }).notNull().default(sql`(unixepoch())`),
});
export type PurchaseOrder = typeof purchaseOrdersTable.$inferSelect;
export type NewPurchaseOrder = typeof purchaseOrdersTable.$inferInsert;

export const purchaseOrderItemsTable = sqliteTable('purchase_order_items', {
  id: text('id').primaryKey(),
  purchaseOrderId: text('purchase_order_id').references(() => purchaseOrdersTable.id),
  itemId: text('item_id').references(() => itemsTable.id),
  itemName: text('item_name').notNull(),
  quantity: integer('quantity').notNull(),
  priceAmount: integer('price_amount').notNull(),
  taxAmount: integer('tax_amount').notNull(),
  createdAt: integer('created_at', { mode: 'timestamp' }).notNull().default(sql`(unixepoch())`),
  updatedAt: integer('updated_at', { mode: 'timestamp' }).notNull().default(sql`(unixepoch())`),
});
export type PurchaseOrderItem = typeof purchaseOrderItemsTable.$inferSelect;
export type NewPurchaseOrderItem = typeof purchaseOrderItemsTable.$inferInsert;

export const purchaseOrderRelations = relations(purchaseOrdersTable, ({ one, many }) => ({
  supplier: one(suppliersTable, {
    fields: [purchaseOrdersTable.supplierId],
    references: [suppliersTable.id],
  }),
  items: many(purchaseOrderItemsTable),
}));

export const purchaseOrderItemRelations = relations(purchaseOrderItemsTable, ({ one }) => ({
  purchaseOrder: one(purchaseOrdersTable, {
    fields: [purchaseOrderItemsTable.purchaseOrderId],
    references: [purchaseOrdersTable.id],
  }),
  item: one(itemsTable, {
    fields: [purchaseOrderItemsTable.itemId],
    references: [itemsTable.id],
  }),
}));

