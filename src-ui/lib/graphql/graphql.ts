/* eslint-disable */
import { DocumentTypeDecoration } from '@graphql-typed-document-node/core';
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
export type MakeEmpty<T extends { [key: string]: unknown }, K extends keyof T> = { [_ in K]?: never };
export type Incremental<T> = T | { [P in keyof T]?: P extends ' $fragmentName' | '__typename' ? T[P] : never };
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: { input: string; output: string; }
  String: { input: string; output: string; }
  Boolean: { input: boolean; output: boolean; }
  Int: { input: number; output: number; }
  Float: { input: number; output: number; }
  /** UUID stored as text */
  DbUuid: { input: string; output: string; }
  /**
   * Combined date and time (without time zone) in `yyyy-MM-dd HH:mm:ss` format.
   *
   * See also [`chrono::NaiveDateTime`][1] for details.
   *
   * [1]: https://docs.rs/chrono/latest/chrono/naive/struct.NaiveDateTime.html
   */
  LocalDateTime: { input: string; output: string; }
  Money: { input: string; output: string; }
  Percentage: { input: string; output: string; }
};

export type AnalyticsOverview = {
  __typename?: 'AnalyticsOverview';
  totalCustomers: Scalars['Int']['output'];
  totalOrders: Scalars['Int']['output'];
  totalProducts: Scalars['Int']['output'];
  totalSales: Scalars['Money']['output'];
};

export type Cart = {
  __typename?: 'Cart';
  cartData: Scalars['String']['output'];
  createdAt: Scalars['LocalDateTime']['output'];
  customer?: Maybe<Customer>;
  customerId?: Maybe<Scalars['DbUuid']['output']>;
  id: Scalars['DbUuid']['output'];
  updatedAt: Scalars['LocalDateTime']['output'];
};

export type CartNewInput = {
  cartData: Scalars['String']['input'];
  customerId?: InputMaybe<Scalars['DbUuid']['input']>;
};

export type CartUpdateInput = {
  cartData?: InputMaybe<Scalars['String']['input']>;
  id: Scalars['DbUuid']['input'];
};

export type Customer = {
  __typename?: 'Customer';
  address?: Maybe<Scalars['String']['output']>;
  createdAt: Scalars['LocalDateTime']['output'];
  email?: Maybe<Scalars['String']['output']>;
  fullName: Scalars['String']['output'];
  id: Scalars['DbUuid']['output'];
  phone?: Maybe<Scalars['String']['output']>;
  updatedAt: Scalars['LocalDateTime']['output'];
};

export type CustomerNewInput = {
  address?: InputMaybe<Scalars['String']['input']>;
  email?: InputMaybe<Scalars['String']['input']>;
  fullName: Scalars['String']['input'];
  phone?: InputMaybe<Scalars['String']['input']>;
};

export type CustomerUpdateInput = {
  address?: InputMaybe<Scalars['String']['input']>;
  email?: InputMaybe<Scalars['String']['input']>;
  fullName?: InputMaybe<Scalars['String']['input']>;
  id: Scalars['DbUuid']['input'];
  phone?: InputMaybe<Scalars['String']['input']>;
};

export type Expense = {
  __typename?: 'Expense';
  amount: Scalars['Money']['output'];
  category: PurchaseCategory;
  categoryId: Scalars['DbUuid']['output'];
  createdAt: Scalars['LocalDateTime']['output'];
  description?: Maybe<Scalars['String']['output']>;
  expenseDate: Scalars['LocalDateTime']['output'];
  id: Scalars['DbUuid']['output'];
  title: Scalars['String']['output'];
  updatedAt: Scalars['LocalDateTime']['output'];
};

export type ExpenseNewInput = {
  amount: Scalars['Money']['input'];
  categoryId: Scalars['DbUuid']['input'];
  description?: InputMaybe<Scalars['String']['input']>;
  expenseDate: Scalars['LocalDateTime']['input'];
  title: Scalars['String']['input'];
};

export type ExpenseUpdateInput = {
  amount?: InputMaybe<Scalars['Money']['input']>;
  categoryId?: InputMaybe<Scalars['DbUuid']['input']>;
  description?: InputMaybe<Scalars['String']['input']>;
  expenseDate?: InputMaybe<Scalars['LocalDateTime']['input']>;
  id: Scalars['DbUuid']['input'];
  title?: InputMaybe<Scalars['String']['input']>;
};

export type Item = {
  __typename?: 'Item';
  category: ItemGroup;
  createdAt: Scalars['LocalDateTime']['output'];
  description?: Maybe<Scalars['String']['output']>;
  id: Scalars['DbUuid']['output'];
  name: Scalars['String']['output'];
  nature: ItemNature;
  price: Scalars['Money']['output'];
  state: ItemState;
  taxes: Array<Tax>;
  updatedAt: Scalars['LocalDateTime']['output'];
};

export type ItemGroup = {
  __typename?: 'ItemGroup';
  createdAt: Scalars['LocalDateTime']['output'];
  description?: Maybe<Scalars['String']['output']>;
  id: Scalars['DbUuid']['output'];
  name: Scalars['String']['output'];
  state: ItemGroupState;
  updatedAt: Scalars['LocalDateTime']['output'];
};

export type ItemGroupNew = {
  description?: InputMaybe<Scalars['String']['input']>;
  name: Scalars['String']['input'];
};

export enum ItemGroupState {
  Active = 'ACTIVE',
  Deleted = 'DELETED',
  Inactive = 'INACTIVE'
}

export type ItemGroupUpdate = {
  description?: InputMaybe<Scalars['String']['input']>;
  id: Scalars['DbUuid']['input'];
  name?: InputMaybe<Scalars['String']['input']>;
  state?: InputMaybe<ItemGroupState>;
  updatedAt?: InputMaybe<Scalars['LocalDateTime']['input']>;
};

export enum ItemNature {
  Goods = 'GOODS',
  Service = 'SERVICE'
}

export enum ItemState {
  Active = 'ACTIVE',
  Deleted = 'DELETED',
  Inactive = 'INACTIVE'
}

export type ItemTaxNewInput = {
  itemId: Scalars['DbUuid']['input'];
  taxId: Scalars['DbUuid']['input'];
};

export type Mutation = {
  __typename?: 'Mutation';
  addUser: User;
  assignTaxToItem: Scalars['Int']['output'];
  createCart: Cart;
  createCustomer: Customer;
  createExpense: Expense;
  createItem: Item;
  createItemCategory: ItemGroup;
  createPurchaseCategory: PurchaseCategory;
  createSalesOrder: SalesOrder;
  createSupplier: Supplier;
  createTax: Tax;
  deleteCart: Scalars['Int']['output'];
  deleteCustomer: Scalars['Int']['output'];
  deleteExpense: Scalars['Int']['output'];
  deleteItem: Scalars['Int']['output'];
  deleteItemCategory: Scalars['Int']['output'];
  deletePurchaseCategory: Scalars['DbUuid']['output'];
  deleteSupplier: Scalars['Int']['output'];
  deleteTax: Scalars['Int']['output'];
  deleteUser: Scalars['Int']['output'];
  login: Scalars['Boolean']['output'];
  logout: Scalars['Boolean']['output'];
  removeTaxFromItem: Scalars['Int']['output'];
  updateCart: Cart;
  updateCustomer: Customer;
  updateExpense: Expense;
  updateItem: Item;
  updateItemCategory: ItemGroup;
  updatePurchaseCategory: PurchaseCategory;
  updateSupplier: Supplier;
  updateTax: Tax;
  updateUser: User;
  voidSalesOrder: SalesOrder;
};


export type MutationAddUserArgs = {
  user: UserNewInput;
};


export type MutationAssignTaxToItemArgs = {
  input: ItemTaxNewInput;
};


export type MutationCreateCartArgs = {
  cart: CartNewInput;
};


export type MutationCreateCustomerArgs = {
  customer: CustomerNewInput;
};


export type MutationCreateExpenseArgs = {
  expense: ExpenseNewInput;
};


export type MutationCreateItemArgs = {
  item: NewItem;
};


export type MutationCreateItemCategoryArgs = {
  newCategory: ItemGroupNew;
};


export type MutationCreatePurchaseCategoryArgs = {
  description?: InputMaybe<Scalars['String']['input']>;
  name: Scalars['String']['input'];
};


export type MutationCreateSalesOrderArgs = {
  salesOrder: SalesOrderNewInput;
};


export type MutationCreateSupplierArgs = {
  supplier: SupplierNewInput;
};


export type MutationCreateTaxArgs = {
  input: TaxNewInput;
};


export type MutationDeleteCartArgs = {
  id: Scalars['DbUuid']['input'];
};


export type MutationDeleteCustomerArgs = {
  id: Scalars['DbUuid']['input'];
};


export type MutationDeleteExpenseArgs = {
  id: Scalars['DbUuid']['input'];
};


export type MutationDeleteItemArgs = {
  id: Scalars['DbUuid']['input'];
};


export type MutationDeleteItemCategoryArgs = {
  id: Scalars['DbUuid']['input'];
};


export type MutationDeletePurchaseCategoryArgs = {
  id: Scalars['DbUuid']['input'];
};


export type MutationDeleteSupplierArgs = {
  id: Scalars['DbUuid']['input'];
};


export type MutationDeleteTaxArgs = {
  id: Scalars['DbUuid']['input'];
};


export type MutationDeleteUserArgs = {
  id: Scalars['DbUuid']['input'];
};


export type MutationLoginArgs = {
  password: Scalars['String']['input'];
  username: Scalars['String']['input'];
};


export type MutationRemoveTaxFromItemArgs = {
  itemId: Scalars['DbUuid']['input'];
  taxId: Scalars['DbUuid']['input'];
};


export type MutationUpdateCartArgs = {
  cart: CartUpdateInput;
};


export type MutationUpdateCustomerArgs = {
  customer: CustomerUpdateInput;
};


export type MutationUpdateExpenseArgs = {
  expense: ExpenseUpdateInput;
};


export type MutationUpdateItemArgs = {
  item: UpdateItem;
};


export type MutationUpdateItemCategoryArgs = {
  category: ItemGroupUpdate;
};


export type MutationUpdatePurchaseCategoryArgs = {
  description?: InputMaybe<Scalars['String']['input']>;
  id: Scalars['DbUuid']['input'];
  name?: InputMaybe<Scalars['String']['input']>;
};


export type MutationUpdateSupplierArgs = {
  supplier: SupplierUpdateInput;
};


export type MutationUpdateTaxArgs = {
  input: TaxUpdateInput;
};


export type MutationUpdateUserArgs = {
  user: UserUpdateInput;
};


export type MutationVoidSalesOrderArgs = {
  id: Scalars['DbUuid']['input'];
};

export type NewItem = {
  categoryId: Scalars['DbUuid']['input'];
  description?: InputMaybe<Scalars['String']['input']>;
  name: Scalars['String']['input'];
  nature: ItemNature;
  price: Scalars['Money']['input'];
  state: ItemState;
  taxIds?: InputMaybe<Array<Scalars['DbUuid']['input']>>;
};

/** Purchase Category */
export type PurchaseCategory = {
  __typename?: 'PurchaseCategory';
  createdAt: Scalars['LocalDateTime']['output'];
  description?: Maybe<Scalars['String']['output']>;
  id: Scalars['DbUuid']['output'];
  name: Scalars['String']['output'];
  state: PurchaseCategoryState;
  updatedAt: Scalars['LocalDateTime']['output'];
};

export enum PurchaseCategoryState {
  Active = 'ACTIVE',
  Deleted = 'DELETED',
  Inactive = 'INACTIVE'
}

export type Query = {
  __typename?: 'Query';
  allPurchaseCategories: Array<PurchaseCategory>;
  analyticsOverview: AnalyticsOverview;
  apiVersion: Scalars['String']['output'];
  cart: Cart;
  carts: Array<Cart>;
  customer: Customer;
  customerByPhone: Customer;
  customers: Array<Customer>;
  expense: Expense;
  expenses: Array<Expense>;
  expensesByCategory: Array<Expense>;
  item: Item;
  itemCategories: Array<ItemGroup>;
  items: Array<Item>;
  itemsCategory: ItemGroup;
  purchaseCategories: Array<PurchaseCategory>;
  purchaseCategory: PurchaseCategory;
  salesOrder: SalesOrder;
  salesOrders: Array<SalesOrder>;
  supplier: Supplier;
  suppliers: Array<Supplier>;
  tax: Tax;
  taxes: Array<Tax>;
  totalCarts: Scalars['Int']['output'];
  totalCustomers: Scalars['Int']['output'];
  totalExpenses: Scalars['Int']['output'];
  totalSalesOrders: Scalars['Int']['output'];
  totalSuppliers: Scalars['Int']['output'];
  totalTaxes: Scalars['Int']['output'];
  user: User;
  users: Array<User>;
};


export type QueryAnalyticsOverviewArgs = {
  days?: InputMaybe<Scalars['Int']['input']>;
};


export type QueryCartArgs = {
  id: Scalars['DbUuid']['input'];
};


export type QueryCartsArgs = {
  first?: InputMaybe<Scalars['Int']['input']>;
  offset?: InputMaybe<Scalars['Int']['input']>;
};


export type QueryCustomerArgs = {
  id: Scalars['DbUuid']['input'];
};


export type QueryCustomerByPhoneArgs = {
  phone: Scalars['String']['input'];
};


export type QueryCustomersArgs = {
  first?: InputMaybe<Scalars['Int']['input']>;
  offset?: InputMaybe<Scalars['Int']['input']>;
};


export type QueryExpenseArgs = {
  id: Scalars['DbUuid']['input'];
};


export type QueryExpensesArgs = {
  first?: InputMaybe<Scalars['Int']['input']>;
  offset?: InputMaybe<Scalars['Int']['input']>;
};


export type QueryExpensesByCategoryArgs = {
  categoryId: Scalars['DbUuid']['input'];
  first?: InputMaybe<Scalars['Int']['input']>;
  offset?: InputMaybe<Scalars['Int']['input']>;
};


export type QueryItemArgs = {
  id: Scalars['DbUuid']['input'];
};


export type QueryItemCategoriesArgs = {
  first?: InputMaybe<Scalars['Int']['input']>;
  offset?: InputMaybe<Scalars['Int']['input']>;
};


export type QueryItemsArgs = {
  first?: InputMaybe<Scalars['Int']['input']>;
  offset?: InputMaybe<Scalars['Int']['input']>;
};


export type QueryItemsCategoryArgs = {
  id: Scalars['DbUuid']['input'];
};


export type QueryPurchaseCategoriesArgs = {
  first?: InputMaybe<Scalars['Int']['input']>;
  offset?: InputMaybe<Scalars['Int']['input']>;
};


export type QueryPurchaseCategoryArgs = {
  id: Scalars['DbUuid']['input'];
};


export type QuerySalesOrderArgs = {
  id: Scalars['DbUuid']['input'];
};


export type QuerySalesOrdersArgs = {
  first?: InputMaybe<Scalars['Int']['input']>;
  offset?: InputMaybe<Scalars['Int']['input']>;
};


export type QuerySupplierArgs = {
  id: Scalars['DbUuid']['input'];
};


export type QuerySuppliersArgs = {
  first?: InputMaybe<Scalars['Int']['input']>;
  offset?: InputMaybe<Scalars['Int']['input']>;
};


export type QueryTaxArgs = {
  id: Scalars['DbUuid']['input'];
};


export type QueryTaxesArgs = {
  first?: InputMaybe<Scalars['Int']['input']>;
  offset?: InputMaybe<Scalars['Int']['input']>;
};


export type QueryUserArgs = {
  id: Scalars['DbUuid']['input'];
};


export type QueryUsersArgs = {
  first?: InputMaybe<Scalars['Int']['input']>;
  offset?: InputMaybe<Scalars['Int']['input']>;
};

export type SalesOrder = {
  __typename?: 'SalesOrder';
  createdAt: Scalars['LocalDateTime']['output'];
  customer: Customer;
  customerId: Scalars['DbUuid']['output'];
  customerName: Scalars['String']['output'];
  customerPhoneNumber: Scalars['String']['output'];
  discAmount: Scalars['Money']['output'];
  id: Scalars['DbUuid']['output'];
  items: Array<SalesOrderItem>;
  netAmount: Scalars['Money']['output'];
  orderDate: Scalars['LocalDateTime']['output'];
  state: SalesOrderState;
  taxAmount: Scalars['Money']['output'];
  taxableAmount: Scalars['Money']['output'];
  totalAmount: Scalars['Money']['output'];
  updatedAt: Scalars['LocalDateTime']['output'];
};

export type SalesOrderItem = {
  __typename?: 'SalesOrderItem';
  createdAt: Scalars['LocalDateTime']['output'];
  id: Scalars['DbUuid']['output'];
  itemId: Scalars['DbUuid']['output'];
  itemName: Scalars['String']['output'];
  orderId: Scalars['DbUuid']['output'];
  priceAmount: Scalars['Money']['output'];
  quantity: Scalars['Int']['output'];
  taxAmount: Scalars['Money']['output'];
  totalAmount: Scalars['Money']['output'];
  updatedAt: Scalars['LocalDateTime']['output'];
};

export type SalesOrderItemInput = {
  itemId: Scalars['DbUuid']['input'];
  itemName: Scalars['String']['input'];
  priceAmount: Scalars['Money']['input'];
  quantity: Scalars['Int']['input'];
  taxAmount: Scalars['Money']['input'];
  totalAmount: Scalars['Money']['input'];
};

export type SalesOrderNewInput = {
  customerId: Scalars['DbUuid']['input'];
  customerName: Scalars['String']['input'];
  customerPhoneNumber: Scalars['String']['input'];
  discAmount: Scalars['Money']['input'];
  items: Array<SalesOrderItemInput>;
  netAmount: Scalars['Money']['input'];
  orderDate: Scalars['LocalDateTime']['input'];
  state: SalesOrderState;
  taxAmount: Scalars['Money']['input'];
  taxableAmount: Scalars['Money']['input'];
  totalAmount: Scalars['Money']['input'];
};

export enum SalesOrderState {
  Cancelled = 'CANCELLED',
  Completed = 'COMPLETED',
  Draft = 'DRAFT'
}

export type Supplier = {
  __typename?: 'Supplier';
  address?: Maybe<Scalars['String']['output']>;
  createdAt: Scalars['LocalDateTime']['output'];
  id: Scalars['DbUuid']['output'];
  name: Scalars['String']['output'];
  phone?: Maybe<Scalars['String']['output']>;
  updatedAt: Scalars['LocalDateTime']['output'];
};

export type SupplierNewInput = {
  address?: InputMaybe<Scalars['String']['input']>;
  name: Scalars['String']['input'];
  phone?: InputMaybe<Scalars['String']['input']>;
};

export type SupplierUpdateInput = {
  address?: InputMaybe<Scalars['String']['input']>;
  id: Scalars['DbUuid']['input'];
  name?: InputMaybe<Scalars['String']['input']>;
  phone?: InputMaybe<Scalars['String']['input']>;
};

export type Tax = {
  __typename?: 'Tax';
  createdAt: Scalars['LocalDateTime']['output'];
  description?: Maybe<Scalars['String']['output']>;
  id: Scalars['DbUuid']['output'];
  name: Scalars['String']['output'];
  rate: Scalars['Percentage']['output'];
  updatedAt: Scalars['LocalDateTime']['output'];
};

export type TaxNewInput = {
  description?: InputMaybe<Scalars['String']['input']>;
  itemIds?: InputMaybe<Array<Scalars['DbUuid']['input']>>;
  name: Scalars['String']['input'];
  rate: Scalars['Percentage']['input'];
};

export type TaxUpdateInput = {
  description?: InputMaybe<Scalars['String']['input']>;
  id: Scalars['DbUuid']['input'];
  name?: InputMaybe<Scalars['String']['input']>;
  rate?: InputMaybe<Scalars['Percentage']['input']>;
};

export type UpdateItem = {
  categoryId?: InputMaybe<Scalars['DbUuid']['input']>;
  description?: InputMaybe<Scalars['String']['input']>;
  id: Scalars['DbUuid']['input'];
  name?: InputMaybe<Scalars['String']['input']>;
  nature?: InputMaybe<ItemNature>;
  price?: InputMaybe<Scalars['Money']['input']>;
  state?: InputMaybe<ItemState>;
  updatedAt?: InputMaybe<Scalars['LocalDateTime']['input']>;
};

export type User = {
  __typename?: 'User';
  createdAt: Scalars['LocalDateTime']['output'];
  fullName: Scalars['String']['output'];
  id: Scalars['DbUuid']['output'];
  lastLoginAt?: Maybe<Scalars['LocalDateTime']['output']>;
  state: UserState;
  updatedAt: Scalars['LocalDateTime']['output'];
  username: Scalars['String']['output'];
};

export type UserNewInput = {
  fullName: Scalars['String']['input'];
  pin: Scalars['String']['input'];
  username: Scalars['String']['input'];
};

export enum UserState {
  Active = 'ACTIVE',
  Inactive = 'INACTIVE',
  Locked = 'LOCKED'
}

export type UserUpdateInput = {
  fullName?: InputMaybe<Scalars['String']['input']>;
  id: Scalars['DbUuid']['input'];
  pin?: InputMaybe<Scalars['String']['input']>;
  state?: InputMaybe<UserState>;
  username?: InputMaybe<Scalars['String']['input']>;
};

export type GetAnalyticsOverviewQueryVariables = Exact<{
  days: Scalars['Int']['input'];
}>;


export type GetAnalyticsOverviewQuery = { __typename?: 'Query', analyticsOverview: { __typename?: 'AnalyticsOverview', totalSales: string, totalOrders: number, totalCustomers: number, totalProducts: number } };

export type GetCategoriesQueryVariables = Exact<{
  first: Scalars['Int']['input'];
  offset: Scalars['Int']['input'];
}>;


export type GetCategoriesQuery = { __typename?: 'Query', itemCategories: Array<{ __typename?: 'ItemGroup', id: string, name: string, description?: string | null, state: ItemGroupState, createdAt: string, updatedAt: string }> };

export type GetCategoryQueryVariables = Exact<{
  id: Scalars['DbUuid']['input'];
}>;


export type GetCategoryQuery = { __typename?: 'Query', itemsCategory: { __typename?: 'ItemGroup', id: string, name: string, description?: string | null, state: ItemGroupState, createdAt: string, updatedAt: string } };

export type CreateCategoryMutationVariables = Exact<{
  input: ItemGroupNew;
}>;


export type CreateCategoryMutation = { __typename?: 'Mutation', createItemCategory: { __typename?: 'ItemGroup', id: string, name: string, description?: string | null, state: ItemGroupState, createdAt: string, updatedAt: string } };

export type UpdateCategoryMutationVariables = Exact<{
  input: ItemGroupUpdate;
}>;


export type UpdateCategoryMutation = { __typename?: 'Mutation', updateItemCategory: { __typename?: 'ItemGroup', id: string, name: string, description?: string | null, state: ItemGroupState, createdAt: string, updatedAt: string } };

export type DeleteCategoryMutationVariables = Exact<{
  id: Scalars['DbUuid']['input'];
}>;


export type DeleteCategoryMutation = { __typename?: 'Mutation', deleteItemCategory: number };

export type GetItemsQueryVariables = Exact<{
  first: Scalars['Int']['input'];
  offset: Scalars['Int']['input'];
}>;


export type GetItemsQuery = { __typename?: 'Query', items: Array<{ __typename?: 'Item', id: string, name: string, description?: string | null, nature: ItemNature, state: ItemState, price: string, createdAt: string, updatedAt: string, category: { __typename?: 'ItemGroup', id: string, name: string, description?: string | null, state: ItemGroupState, createdAt: string, updatedAt: string }, taxes: Array<{ __typename?: 'Tax', id: string, name: string, rate: string, description?: string | null, createdAt: string, updatedAt: string }> }> };

export type GetItemCategoriesQueryVariables = Exact<{ [key: string]: never; }>;


export type GetItemCategoriesQuery = { __typename?: 'Query', itemCategories: Array<{ __typename?: 'ItemGroup', id: string, name: string, description?: string | null, state: ItemGroupState, createdAt: string, updatedAt: string }> };

export type GetItemTaxesQueryVariables = Exact<{ [key: string]: never; }>;


export type GetItemTaxesQuery = { __typename?: 'Query', taxes: Array<{ __typename?: 'Tax', id: string, name: string, rate: string, description?: string | null, createdAt: string, updatedAt: string }> };

export type CreateItemMutationVariables = Exact<{
  input: NewItem;
}>;


export type CreateItemMutation = { __typename?: 'Mutation', createItem: { __typename?: 'Item', id: string, name: string, description?: string | null, nature: ItemNature, state: ItemState, price: string, createdAt: string, updatedAt: string, category: { __typename?: 'ItemGroup', id: string, name: string, description?: string | null, state: ItemGroupState, createdAt: string, updatedAt: string }, taxes: Array<{ __typename?: 'Tax', id: string, name: string, rate: string, description?: string | null, createdAt: string, updatedAt: string }> } };

export type UpdateItemMutationVariables = Exact<{
  input: UpdateItem;
}>;


export type UpdateItemMutation = { __typename?: 'Mutation', updateItem: { __typename?: 'Item', id: string, name: string, description?: string | null, nature: ItemNature, state: ItemState, price: string, createdAt: string, updatedAt: string, category: { __typename?: 'ItemGroup', id: string, name: string, description?: string | null, state: ItemGroupState, createdAt: string, updatedAt: string }, taxes: Array<{ __typename?: 'Tax', id: string, name: string, rate: string, description?: string | null, createdAt: string, updatedAt: string }> } };

export type DeleteItemMutationVariables = Exact<{
  id: Scalars['DbUuid']['input'];
}>;


export type DeleteItemMutation = { __typename?: 'Mutation', deleteItem: number };

export type GetCustomersQueryVariables = Exact<{
  first: Scalars['Int']['input'];
  offset: Scalars['Int']['input'];
}>;


export type GetCustomersQuery = { __typename?: 'Query', totalCustomers: number, customers: Array<{ __typename?: 'Customer', id: string, fullName: string, email?: string | null, phone?: string | null, address?: string | null, createdAt: string, updatedAt: string }> };

export type CreateCustomerMutationVariables = Exact<{
  input: CustomerNewInput;
}>;


export type CreateCustomerMutation = { __typename?: 'Mutation', createCustomer: { __typename?: 'Customer', id: string, fullName: string, email?: string | null, phone?: string | null, address?: string | null, createdAt: string, updatedAt: string } };

export type UpdateCustomerMutationVariables = Exact<{
  input: CustomerUpdateInput;
}>;


export type UpdateCustomerMutation = { __typename?: 'Mutation', updateCustomer: { __typename?: 'Customer', id: string, fullName: string, email?: string | null, phone?: string | null, address?: string | null, createdAt: string, updatedAt: string } };

export type DeleteCustomerMutationVariables = Exact<{
  id: Scalars['DbUuid']['input'];
}>;


export type DeleteCustomerMutation = { __typename?: 'Mutation', deleteCustomer: number };

export type GetSalesOrdersQueryVariables = Exact<{
  first: Scalars['Int']['input'];
  offset: Scalars['Int']['input'];
}>;


export type GetSalesOrdersQuery = { __typename?: 'Query', totalSalesOrders: number, salesOrders: Array<{ __typename?: 'SalesOrder', id: string, customerId: string, customerName: string, customerPhoneNumber: string, orderDate: string, netAmount: string, discAmount: string, taxableAmount: string, taxAmount: string, totalAmount: string, state: SalesOrderState, createdAt: string, updatedAt: string, customer: { __typename?: 'Customer', id: string, fullName: string, phone?: string | null, createdAt: string, updatedAt: string }, items: Array<{ __typename?: 'SalesOrderItem', id: string, orderId: string, itemId: string, itemName: string, quantity: number, priceAmount: string, taxAmount: string, totalAmount: string, createdAt: string, updatedAt: string }> }> };

export type GetPosCategoriesQueryVariables = Exact<{
  first: Scalars['Int']['input'];
}>;


export type GetPosCategoriesQuery = { __typename?: 'Query', itemCategories: Array<{ __typename?: 'ItemGroup', id: string, name: string, description?: string | null, state: ItemGroupState, createdAt: string, updatedAt: string }> };

export type GetPosItemsQueryVariables = Exact<{
  first: Scalars['Int']['input'];
  offset: Scalars['Int']['input'];
}>;


export type GetPosItemsQuery = { __typename?: 'Query', items: Array<{ __typename?: 'Item', id: string, name: string, description?: string | null, nature: ItemNature, state: ItemState, price: string, createdAt: string, updatedAt: string, category: { __typename?: 'ItemGroup', id: string, name: string, description?: string | null, state: ItemGroupState, createdAt: string, updatedAt: string }, taxes: Array<{ __typename?: 'Tax', id: string, name: string, rate: string, description?: string | null, createdAt: string, updatedAt: string }> }> };

export type GetPosTaxesQueryVariables = Exact<{ [key: string]: never; }>;


export type GetPosTaxesQuery = { __typename?: 'Query', taxes: Array<{ __typename?: 'Tax', id: string, name: string, rate: string, description?: string | null, createdAt: string, updatedAt: string }> };

export type GetPosCustomerByPhoneQueryVariables = Exact<{
  phone: Scalars['String']['input'];
}>;


export type GetPosCustomerByPhoneQuery = { __typename?: 'Query', customerByPhone: { __typename?: 'Customer', id: string, fullName: string, phone?: string | null, email?: string | null, address?: string | null, createdAt: string, updatedAt: string } };

export type CreatePosCustomerMutationVariables = Exact<{
  fullName: Scalars['String']['input'];
  phone: Scalars['String']['input'];
}>;


export type CreatePosCustomerMutation = { __typename?: 'Mutation', createCustomer: { __typename?: 'Customer', id: string, fullName: string, phone?: string | null, email?: string | null, address?: string | null, createdAt: string, updatedAt: string } };

export type CreateSalesOrderMutationVariables = Exact<{
  salesOrder: SalesOrderNewInput;
}>;


export type CreateSalesOrderMutation = { __typename?: 'Mutation', createSalesOrder: { __typename?: 'SalesOrder', id: string, customerName: string, orderDate: string, netAmount: string, taxAmount: string, totalAmount: string, state: SalesOrderState } };

export type GetPurchaseCategoriesQueryVariables = Exact<{
  first: Scalars['Int']['input'];
  offset: Scalars['Int']['input'];
}>;


export type GetPurchaseCategoriesQuery = { __typename?: 'Query', purchaseCategories: Array<{ __typename?: 'PurchaseCategory', id: string, name: string, description?: string | null, state: PurchaseCategoryState, createdAt: string, updatedAt: string }> };

export type GetPurchaseCategoryQueryVariables = Exact<{
  id: Scalars['DbUuid']['input'];
}>;


export type GetPurchaseCategoryQuery = { __typename?: 'Query', purchaseCategory: { __typename?: 'PurchaseCategory', id: string, name: string, description?: string | null, state: PurchaseCategoryState, createdAt: string, updatedAt: string } };

export type CreatePurchaseCategoryMutationVariables = Exact<{
  name: Scalars['String']['input'];
  description?: InputMaybe<Scalars['String']['input']>;
}>;


export type CreatePurchaseCategoryMutation = { __typename?: 'Mutation', createPurchaseCategory: { __typename?: 'PurchaseCategory', id: string, name: string, description?: string | null, state: PurchaseCategoryState, createdAt: string, updatedAt: string } };

export type UpdatePurchaseCategoryMutationVariables = Exact<{
  id: Scalars['DbUuid']['input'];
  name?: InputMaybe<Scalars['String']['input']>;
  description?: InputMaybe<Scalars['String']['input']>;
}>;


export type UpdatePurchaseCategoryMutation = { __typename?: 'Mutation', updatePurchaseCategory: { __typename?: 'PurchaseCategory', id: string, name: string, description?: string | null, state: PurchaseCategoryState, createdAt: string, updatedAt: string } };

export type DeletePurchaseCategoryMutationVariables = Exact<{
  id: Scalars['DbUuid']['input'];
}>;


export type DeletePurchaseCategoryMutation = { __typename?: 'Mutation', deletePurchaseCategory: string };

export type GetExpensesQueryVariables = Exact<{
  first: Scalars['Int']['input'];
  offset: Scalars['Int']['input'];
}>;


export type GetExpensesQuery = { __typename?: 'Query', totalExpenses: number, expenses: Array<{ __typename?: 'Expense', id: string, title: string, amount: string, expenseDate: string, categoryId: string, description?: string | null, createdAt: string, updatedAt: string, category: { __typename?: 'PurchaseCategory', id: string, name: string } }> };

export type GetPurchaseCategoriesForExpensesQueryVariables = Exact<{ [key: string]: never; }>;


export type GetPurchaseCategoriesForExpensesQuery = { __typename?: 'Query', allPurchaseCategories: Array<{ __typename?: 'PurchaseCategory', id: string, name: string }> };

export type GetExpensesByCategoryQueryVariables = Exact<{
  categoryId: Scalars['DbUuid']['input'];
  first: Scalars['Int']['input'];
  offset: Scalars['Int']['input'];
}>;


export type GetExpensesByCategoryQuery = { __typename?: 'Query', expensesByCategory: Array<{ __typename?: 'Expense', id: string, title: string, amount: string, expenseDate: string, categoryId: string, description?: string | null, createdAt: string, updatedAt: string, category: { __typename?: 'PurchaseCategory', id: string, name: string } }> };

export type CreateExpenseMutationVariables = Exact<{
  input: ExpenseNewInput;
}>;


export type CreateExpenseMutation = { __typename?: 'Mutation', createExpense: { __typename?: 'Expense', id: string, title: string, amount: string, expenseDate: string, categoryId: string, description?: string | null, createdAt: string, updatedAt: string, category: { __typename?: 'PurchaseCategory', id: string, name: string } } };

export type UpdateExpenseMutationVariables = Exact<{
  input: ExpenseUpdateInput;
}>;


export type UpdateExpenseMutation = { __typename?: 'Mutation', updateExpense: { __typename?: 'Expense', id: string, title: string, amount: string, expenseDate: string, categoryId: string, description?: string | null, createdAt: string, updatedAt: string, category: { __typename?: 'PurchaseCategory', id: string, name: string } } };

export type DeleteExpenseMutationVariables = Exact<{
  id: Scalars['DbUuid']['input'];
}>;


export type DeleteExpenseMutation = { __typename?: 'Mutation', deleteExpense: number };

export type GetTaxesQueryVariables = Exact<{
  first: Scalars['Int']['input'];
  offset: Scalars['Int']['input'];
}>;


export type GetTaxesQuery = { __typename?: 'Query', totalTaxes: number, taxes: Array<{ __typename?: 'Tax', id: string, name: string, rate: string, description?: string | null, createdAt: string, updatedAt: string }> };

export type CreateTaxMutationVariables = Exact<{
  input: TaxNewInput;
}>;


export type CreateTaxMutation = { __typename?: 'Mutation', createTax: { __typename?: 'Tax', id: string, name: string, rate: string, description?: string | null, createdAt: string, updatedAt: string } };

export type UpdateTaxMutationVariables = Exact<{
  input: TaxUpdateInput;
}>;


export type UpdateTaxMutation = { __typename?: 'Mutation', updateTax: { __typename?: 'Tax', id: string, name: string, rate: string, description?: string | null, createdAt: string, updatedAt: string } };

export type DeleteTaxMutationVariables = Exact<{
  id: Scalars['DbUuid']['input'];
}>;


export type DeleteTaxMutation = { __typename?: 'Mutation', deleteTax: number };

export type GetSuppliersQueryVariables = Exact<{
  first: Scalars['Int']['input'];
  offset: Scalars['Int']['input'];
}>;


export type GetSuppliersQuery = { __typename?: 'Query', totalSuppliers: number, suppliers: Array<{ __typename?: 'Supplier', id: string, name: string, address?: string | null, phone?: string | null, createdAt: string, updatedAt: string }> };

export type CreateSupplierMutationVariables = Exact<{
  input: SupplierNewInput;
}>;


export type CreateSupplierMutation = { __typename?: 'Mutation', createSupplier: { __typename?: 'Supplier', id: string, name: string, address?: string | null, phone?: string | null, createdAt: string, updatedAt: string } };

export type UpdateSupplierMutationVariables = Exact<{
  input: SupplierUpdateInput;
}>;


export type UpdateSupplierMutation = { __typename?: 'Mutation', updateSupplier: { __typename?: 'Supplier', id: string, name: string, address?: string | null, phone?: string | null, createdAt: string, updatedAt: string } };

export type DeleteSupplierMutationVariables = Exact<{
  id: Scalars['DbUuid']['input'];
}>;


export type DeleteSupplierMutation = { __typename?: 'Mutation', deleteSupplier: number };

export class TypedDocumentString<TResult, TVariables>
  extends String
  implements DocumentTypeDecoration<TResult, TVariables>
{
  __apiType?: DocumentTypeDecoration<TResult, TVariables>['__apiType'];

  constructor(private value: string, public __meta__?: Record<string, any> | undefined) {
    super(value);
  }

  toString(): string & DocumentTypeDecoration<TResult, TVariables> {
    return this.value;
  }
}

export const GetAnalyticsOverviewDocument = new TypedDocumentString(`
    query GetAnalyticsOverview($days: Int!) {
  analyticsOverview(days: $days) {
    totalSales
    totalOrders
    totalCustomers
    totalProducts
  }
}
    `) as unknown as TypedDocumentString<GetAnalyticsOverviewQuery, GetAnalyticsOverviewQueryVariables>;
export const GetCategoriesDocument = new TypedDocumentString(`
    query getCategories($first: Int!, $offset: Int!) {
  itemCategories(first: $first, offset: $offset) {
    id
    name
    description
    state
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<GetCategoriesQuery, GetCategoriesQueryVariables>;
export const GetCategoryDocument = new TypedDocumentString(`
    query getCategory($id: DbUuid!) {
  itemsCategory(id: $id) {
    id
    name
    description
    state
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<GetCategoryQuery, GetCategoryQueryVariables>;
export const CreateCategoryDocument = new TypedDocumentString(`
    mutation createCategory($input: ItemGroupNew!) {
  createItemCategory(newCategory: $input) {
    id
    name
    description
    state
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<CreateCategoryMutation, CreateCategoryMutationVariables>;
export const UpdateCategoryDocument = new TypedDocumentString(`
    mutation updateCategory($input: ItemGroupUpdate!) {
  updateItemCategory(category: $input) {
    id
    name
    description
    state
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<UpdateCategoryMutation, UpdateCategoryMutationVariables>;
export const DeleteCategoryDocument = new TypedDocumentString(`
    mutation deleteCategory($id: DbUuid!) {
  deleteItemCategory(id: $id)
}
    `) as unknown as TypedDocumentString<DeleteCategoryMutation, DeleteCategoryMutationVariables>;
export const GetItemsDocument = new TypedDocumentString(`
    query getItems($first: Int!, $offset: Int!) {
  items(first: $first, offset: $offset) {
    id
    name
    description
    nature
    state
    price
    createdAt
    updatedAt
    category {
      id
      name
      description
      state
      createdAt
      updatedAt
    }
    taxes {
      id
      name
      rate
      description
      createdAt
      updatedAt
    }
  }
}
    `) as unknown as TypedDocumentString<GetItemsQuery, GetItemsQueryVariables>;
export const GetItemCategoriesDocument = new TypedDocumentString(`
    query getItemCategories {
  itemCategories {
    id
    name
    description
    state
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<GetItemCategoriesQuery, GetItemCategoriesQueryVariables>;
export const GetItemTaxesDocument = new TypedDocumentString(`
    query getItemTaxes {
  taxes {
    id
    name
    rate
    description
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<GetItemTaxesQuery, GetItemTaxesQueryVariables>;
export const CreateItemDocument = new TypedDocumentString(`
    mutation createItem($input: NewItem!) {
  createItem(item: $input) {
    id
    name
    description
    nature
    state
    price
    createdAt
    updatedAt
    category {
      id
      name
      description
      state
      createdAt
      updatedAt
    }
    taxes {
      id
      name
      rate
      description
      createdAt
      updatedAt
    }
  }
}
    `) as unknown as TypedDocumentString<CreateItemMutation, CreateItemMutationVariables>;
export const UpdateItemDocument = new TypedDocumentString(`
    mutation updateItem($input: UpdateItem!) {
  updateItem(item: $input) {
    id
    name
    description
    nature
    state
    price
    createdAt
    updatedAt
    category {
      id
      name
      description
      state
      createdAt
      updatedAt
    }
    taxes {
      id
      name
      rate
      description
      createdAt
      updatedAt
    }
  }
}
    `) as unknown as TypedDocumentString<UpdateItemMutation, UpdateItemMutationVariables>;
export const DeleteItemDocument = new TypedDocumentString(`
    mutation deleteItem($id: DbUuid!) {
  deleteItem(id: $id)
}
    `) as unknown as TypedDocumentString<DeleteItemMutation, DeleteItemMutationVariables>;
export const GetCustomersDocument = new TypedDocumentString(`
    query GetCustomers($first: Int!, $offset: Int!) {
  customers(first: $first, offset: $offset) {
    id
    fullName
    email
    phone
    address
    createdAt
    updatedAt
  }
  totalCustomers
}
    `) as unknown as TypedDocumentString<GetCustomersQuery, GetCustomersQueryVariables>;
export const CreateCustomerDocument = new TypedDocumentString(`
    mutation CreateCustomer($input: CustomerNewInput!) {
  createCustomer(customer: $input) {
    id
    fullName
    email
    phone
    address
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<CreateCustomerMutation, CreateCustomerMutationVariables>;
export const UpdateCustomerDocument = new TypedDocumentString(`
    mutation UpdateCustomer($input: CustomerUpdateInput!) {
  updateCustomer(customer: $input) {
    id
    fullName
    email
    phone
    address
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<UpdateCustomerMutation, UpdateCustomerMutationVariables>;
export const DeleteCustomerDocument = new TypedDocumentString(`
    mutation DeleteCustomer($id: DbUuid!) {
  deleteCustomer(id: $id)
}
    `) as unknown as TypedDocumentString<DeleteCustomerMutation, DeleteCustomerMutationVariables>;
export const GetSalesOrdersDocument = new TypedDocumentString(`
    query GetSalesOrders($first: Int!, $offset: Int!) {
  salesOrders(first: $first, offset: $offset) {
    id
    customerId
    customerName
    customerPhoneNumber
    orderDate
    netAmount
    discAmount
    taxableAmount
    taxAmount
    totalAmount
    state
    createdAt
    updatedAt
    customer {
      id
      fullName
      phone
      createdAt
      updatedAt
    }
    items {
      id
      orderId
      itemId
      itemName
      quantity
      priceAmount
      taxAmount
      totalAmount
      createdAt
      updatedAt
    }
  }
  totalSalesOrders
}
    `) as unknown as TypedDocumentString<GetSalesOrdersQuery, GetSalesOrdersQueryVariables>;
export const GetPosCategoriesDocument = new TypedDocumentString(`
    query getPosCategories($first: Int!) {
  itemCategories(first: $first) {
    id
    name
    description
    state
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<GetPosCategoriesQuery, GetPosCategoriesQueryVariables>;
export const GetPosItemsDocument = new TypedDocumentString(`
    query getPosItems($first: Int!, $offset: Int!) {
  items(first: $first, offset: $offset) {
    id
    name
    description
    nature
    state
    price
    createdAt
    updatedAt
    category {
      id
      name
      description
      state
      createdAt
      updatedAt
    }
    taxes {
      id
      name
      rate
      description
      createdAt
      updatedAt
    }
  }
}
    `) as unknown as TypedDocumentString<GetPosItemsQuery, GetPosItemsQueryVariables>;
export const GetPosTaxesDocument = new TypedDocumentString(`
    query getPosTaxes {
  taxes {
    id
    name
    rate
    description
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<GetPosTaxesQuery, GetPosTaxesQueryVariables>;
export const GetPosCustomerByPhoneDocument = new TypedDocumentString(`
    query getPosCustomerByPhone($phone: String!) {
  customerByPhone(phone: $phone) {
    id
    fullName
    phone
    email
    address
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<GetPosCustomerByPhoneQuery, GetPosCustomerByPhoneQueryVariables>;
export const CreatePosCustomerDocument = new TypedDocumentString(`
    mutation createPosCustomer($fullName: String!, $phone: String!) {
  createCustomer(customer: {fullName: $fullName, phone: $phone}) {
    id
    fullName
    phone
    email
    address
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<CreatePosCustomerMutation, CreatePosCustomerMutationVariables>;
export const CreateSalesOrderDocument = new TypedDocumentString(`
    mutation createSalesOrder($salesOrder: SalesOrderNewInput!) {
  createSalesOrder(salesOrder: $salesOrder) {
    id
    customerName
    orderDate
    netAmount
    taxAmount
    totalAmount
    state
  }
}
    `) as unknown as TypedDocumentString<CreateSalesOrderMutation, CreateSalesOrderMutationVariables>;
export const GetPurchaseCategoriesDocument = new TypedDocumentString(`
    query getPurchaseCategories($first: Int!, $offset: Int!) {
  purchaseCategories(first: $first, offset: $offset) {
    id
    name
    description
    state
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<GetPurchaseCategoriesQuery, GetPurchaseCategoriesQueryVariables>;
export const GetPurchaseCategoryDocument = new TypedDocumentString(`
    query getPurchaseCategory($id: DbUuid!) {
  purchaseCategory(id: $id) {
    id
    name
    description
    state
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<GetPurchaseCategoryQuery, GetPurchaseCategoryQueryVariables>;
export const CreatePurchaseCategoryDocument = new TypedDocumentString(`
    mutation createPurchaseCategory($name: String!, $description: String) {
  createPurchaseCategory(name: $name, description: $description) {
    id
    name
    description
    state
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<CreatePurchaseCategoryMutation, CreatePurchaseCategoryMutationVariables>;
export const UpdatePurchaseCategoryDocument = new TypedDocumentString(`
    mutation updatePurchaseCategory($id: DbUuid!, $name: String, $description: String) {
  updatePurchaseCategory(id: $id, name: $name, description: $description) {
    id
    name
    description
    state
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<UpdatePurchaseCategoryMutation, UpdatePurchaseCategoryMutationVariables>;
export const DeletePurchaseCategoryDocument = new TypedDocumentString(`
    mutation deletePurchaseCategory($id: DbUuid!) {
  deletePurchaseCategory(id: $id)
}
    `) as unknown as TypedDocumentString<DeletePurchaseCategoryMutation, DeletePurchaseCategoryMutationVariables>;
export const GetExpensesDocument = new TypedDocumentString(`
    query GetExpenses($first: Int!, $offset: Int!) {
  expenses(first: $first, offset: $offset) {
    id
    title
    amount
    expenseDate
    categoryId
    description
    createdAt
    updatedAt
    category {
      id
      name
    }
  }
  totalExpenses
}
    `) as unknown as TypedDocumentString<GetExpensesQuery, GetExpensesQueryVariables>;
export const GetPurchaseCategoriesForExpensesDocument = new TypedDocumentString(`
    query GetPurchaseCategoriesForExpenses {
  allPurchaseCategories {
    id
    name
  }
}
    `) as unknown as TypedDocumentString<GetPurchaseCategoriesForExpensesQuery, GetPurchaseCategoriesForExpensesQueryVariables>;
export const GetExpensesByCategoryDocument = new TypedDocumentString(`
    query GetExpensesByCategory($categoryId: DbUuid!, $first: Int!, $offset: Int!) {
  expensesByCategory(categoryId: $categoryId, first: $first, offset: $offset) {
    id
    title
    amount
    expenseDate
    categoryId
    description
    createdAt
    updatedAt
    category {
      id
      name
    }
  }
}
    `) as unknown as TypedDocumentString<GetExpensesByCategoryQuery, GetExpensesByCategoryQueryVariables>;
export const CreateExpenseDocument = new TypedDocumentString(`
    mutation CreateExpense($input: ExpenseNewInput!) {
  createExpense(expense: $input) {
    id
    title
    amount
    expenseDate
    categoryId
    description
    createdAt
    updatedAt
    category {
      id
      name
    }
  }
}
    `) as unknown as TypedDocumentString<CreateExpenseMutation, CreateExpenseMutationVariables>;
export const UpdateExpenseDocument = new TypedDocumentString(`
    mutation UpdateExpense($input: ExpenseUpdateInput!) {
  updateExpense(expense: $input) {
    id
    title
    amount
    expenseDate
    categoryId
    description
    createdAt
    updatedAt
    category {
      id
      name
    }
  }
}
    `) as unknown as TypedDocumentString<UpdateExpenseMutation, UpdateExpenseMutationVariables>;
export const DeleteExpenseDocument = new TypedDocumentString(`
    mutation DeleteExpense($id: DbUuid!) {
  deleteExpense(id: $id)
}
    `) as unknown as TypedDocumentString<DeleteExpenseMutation, DeleteExpenseMutationVariables>;
export const GetTaxesDocument = new TypedDocumentString(`
    query GetTaxes($first: Int!, $offset: Int!) {
  taxes(first: $first, offset: $offset) {
    id
    name
    rate
    description
    createdAt
    updatedAt
  }
  totalTaxes
}
    `) as unknown as TypedDocumentString<GetTaxesQuery, GetTaxesQueryVariables>;
export const CreateTaxDocument = new TypedDocumentString(`
    mutation CreateTax($input: TaxNewInput!) {
  createTax(input: $input) {
    id
    name
    rate
    description
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<CreateTaxMutation, CreateTaxMutationVariables>;
export const UpdateTaxDocument = new TypedDocumentString(`
    mutation UpdateTax($input: TaxUpdateInput!) {
  updateTax(input: $input) {
    id
    name
    rate
    description
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<UpdateTaxMutation, UpdateTaxMutationVariables>;
export const DeleteTaxDocument = new TypedDocumentString(`
    mutation DeleteTax($id: DbUuid!) {
  deleteTax(id: $id)
}
    `) as unknown as TypedDocumentString<DeleteTaxMutation, DeleteTaxMutationVariables>;
export const GetSuppliersDocument = new TypedDocumentString(`
    query GetSuppliers($first: Int!, $offset: Int!) {
  suppliers(first: $first, offset: $offset) {
    id
    name
    address
    phone
    createdAt
    updatedAt
  }
  totalSuppliers
}
    `) as unknown as TypedDocumentString<GetSuppliersQuery, GetSuppliersQueryVariables>;
export const CreateSupplierDocument = new TypedDocumentString(`
    mutation CreateSupplier($input: SupplierNewInput!) {
  createSupplier(supplier: $input) {
    id
    name
    address
    phone
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<CreateSupplierMutation, CreateSupplierMutationVariables>;
export const UpdateSupplierDocument = new TypedDocumentString(`
    mutation UpdateSupplier($input: SupplierUpdateInput!) {
  updateSupplier(supplier: $input) {
    id
    name
    address
    phone
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<UpdateSupplierMutation, UpdateSupplierMutationVariables>;
export const DeleteSupplierDocument = new TypedDocumentString(`
    mutation DeleteSupplier($id: DbUuid!) {
  deleteSupplier(id: $id)
}
    `) as unknown as TypedDocumentString<DeleteSupplierMutation, DeleteSupplierMutationVariables>;