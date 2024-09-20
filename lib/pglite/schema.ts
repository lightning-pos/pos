import { pgTable, integer, text, timestamp, pgEnum } from "drizzle-orm/pg-core";

export const customers = pgTable('customers', {
  id: text('id').primaryKey(),
  name: text('name').notNull(),
  email: text('email'),
  phoneNumber: text('phone_number'),
  countryCode: text('country_code'),
  createdAt: timestamp('created_at').defaultNow(),
  updatedAt: timestamp('updated_at').defaultNow(),
});

export const itemCategories = pgTable('item_categories', {
  id: text('id').primaryKey(),
  name: text('name').notNull(),
  description: text('description'),
  state: text('state'),
  createdAt: timestamp('created_at').defaultNow(),
  updatedAt: timestamp('updated_at').defaultNow(),
});


export const items = pgTable('items', {
  id: text('id').primaryKey(),
  categoryId: text('category_id').references(() => itemCategories.id, { onDelete: 'restrict' }),
  name: text('name').notNull(),
  description: text('description'),
  price: integer('price'),
  createdAt: timestamp('created_at').defaultNow(),
  updatedAt: timestamp('updated_at').defaultNow(),
});


export const orderState = pgEnum('order_state', ['open', 'closed', 'cancelled']);
export const orders = pgTable('orders', {
  id: text('id').primaryKey(),
  customerId: text('customer_id').references(() => customers.id, { onDelete: 'restrict' }),
  customerName: text('customer_name'),
  customerPhoneNumber: text('customer_phone_number'),
  orderDate: timestamp('order_date').defaultNow(),
  netAmount: integer('net_amount'),
  discAmount: integer('disc_amount'),
  taxAmount: integer('tax_amount'),
  state: orderState('order_state'),
  createdAt: timestamp('created_at').defaultNow(),
  updatedAt: timestamp('updated_at').defaultNow(),
});

export const orderItems = pgTable('order_items', {
  id: text('id').primaryKey(),
  orderId: text('order_id').references(() => orders.id, { onDelete: 'cascade' }),
  itemId: text('item_id').references(() => items.id, { onDelete: 'restrict' }),
  itemName: text('item_name'),
  quantity: integer('quantity'),
  priceAmount: integer('price_amount'),
  taxAmount: integer('tax_amount'),
  createdAt: timestamp('created_at').defaultNow(),
  updatedAt: timestamp('updated_at').defaultNow(),
});
