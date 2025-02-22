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
  DbUuid: { input: string; output: string; }
  /**
   * Combined date and time (without time zone) in `yyyy-MM-dd HH:mm:ss` format.
   *
   * See also [`chrono::NaiveDateTime`][1] for details.
   *
   * [1]: https://docs.rs/chrono/latest/chrono/naive/struct.NaiveDateTime.html
   */
  LocalDateTime: { input: string; output: string; }
  Money: { input: string; output: number; }
  Percentage: { input: string; output: number; }
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
  createItem: Item;
  createItemCategory: ItemGroup;
  createSalesOrder: SalesOrder;
  createTax: Tax;
  deleteCart: Scalars['Int']['output'];
  deleteCustomer: Scalars['Int']['output'];
  deleteItem: Scalars['Int']['output'];
  deleteItemCategory: Scalars['Int']['output'];
  deleteTax: Scalars['Int']['output'];
  deleteUser: Scalars['Int']['output'];
  login: Scalars['Boolean']['output'];
  logout: Scalars['Boolean']['output'];
  removeTaxFromItem: Scalars['Int']['output'];
  updateCart: Cart;
  updateCustomer: Customer;
  updateItem: Item;
  updateItemCategory: ItemGroup;
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


export type MutationCreateItemArgs = {
  item: NewItem;
};


export type MutationCreateItemCategoryArgs = {
  newCategory: ItemGroupNew;
};


export type MutationCreateSalesOrderArgs = {
  salesOrder: SalesOrderNewInput;
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


export type MutationDeleteItemArgs = {
  id: Scalars['DbUuid']['input'];
};


export type MutationDeleteItemCategoryArgs = {
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


export type MutationUpdateItemArgs = {
  item: UpdateItem;
};


export type MutationUpdateItemCategoryArgs = {
  category: ItemGroupUpdate;
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

export type Query = {
  __typename?: 'Query';
  analyticsOverview: AnalyticsOverview;
  apiVersion: Scalars['String']['output'];
  cart: Cart;
  carts: Array<Cart>;
  customer: Customer;
  customerByPhone: Customer;
  customers: Array<Customer>;
  item: Item;
  itemCategories: Array<ItemGroup>;
  items: Array<Item>;
  itemsCategory: ItemGroup;
  salesOrder: SalesOrder;
  salesOrders: Array<SalesOrder>;
  tax: Tax;
  taxes: Array<Tax>;
  totalCarts: Scalars['Int']['output'];
  totalCustomers: Scalars['Int']['output'];
  totalSalesOrders: Scalars['Int']['output'];
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


export type QuerySalesOrderArgs = {
  id: Scalars['DbUuid']['input'];
};


export type QuerySalesOrdersArgs = {
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
  updatedAt: Scalars['LocalDateTime']['output'];
};

export type SalesOrderItemInput = {
  itemId: Scalars['DbUuid']['input'];
  itemName: Scalars['String']['input'];
  priceAmount: Scalars['Money']['input'];
  quantity: Scalars['Int']['input'];
  taxAmount: Scalars['Money']['input'];
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


export type GetAnalyticsOverviewQuery = { __typename?: 'Query', analyticsOverview: { __typename?: 'AnalyticsOverview', totalSales: number, totalOrders: number, totalCustomers: number, totalProducts: number } };

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


export type GetItemsQuery = { __typename?: 'Query', items: Array<{ __typename?: 'Item', id: string, name: string, description?: string | null, nature: ItemNature, state: ItemState, price: number, createdAt: string, updatedAt: string, category: { __typename?: 'ItemGroup', id: string, name: string, description?: string | null, state: ItemGroupState, createdAt: string, updatedAt: string }, taxes: Array<{ __typename?: 'Tax', id: string, name: string, rate: number, description?: string | null, createdAt: string, updatedAt: string }> }> };

export type GetItemCategoriesQueryVariables = Exact<{ [key: string]: never; }>;


export type GetItemCategoriesQuery = { __typename?: 'Query', itemCategories: Array<{ __typename?: 'ItemGroup', id: string, name: string, description?: string | null, state: ItemGroupState, createdAt: string, updatedAt: string }> };

export type GetItemTaxesQueryVariables = Exact<{ [key: string]: never; }>;


export type GetItemTaxesQuery = { __typename?: 'Query', taxes: Array<{ __typename?: 'Tax', id: string, name: string, rate: number, description?: string | null, createdAt: string, updatedAt: string }> };

export type CreateItemMutationVariables = Exact<{
  input: NewItem;
}>;


export type CreateItemMutation = { __typename?: 'Mutation', createItem: { __typename?: 'Item', id: string, name: string, description?: string | null, nature: ItemNature, state: ItemState, price: number, createdAt: string, updatedAt: string, category: { __typename?: 'ItemGroup', id: string, name: string, description?: string | null, state: ItemGroupState, createdAt: string, updatedAt: string }, taxes: Array<{ __typename?: 'Tax', id: string, name: string, rate: number, description?: string | null, createdAt: string, updatedAt: string }> } };

export type UpdateItemMutationVariables = Exact<{
  input: UpdateItem;
}>;


export type UpdateItemMutation = { __typename?: 'Mutation', updateItem: { __typename?: 'Item', id: string, name: string, description?: string | null, nature: ItemNature, state: ItemState, price: number, createdAt: string, updatedAt: string, category: { __typename?: 'ItemGroup', id: string, name: string, description?: string | null, state: ItemGroupState, createdAt: string, updatedAt: string }, taxes: Array<{ __typename?: 'Tax', id: string, name: string, rate: number, description?: string | null, createdAt: string, updatedAt: string }> } };

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


export type GetSalesOrdersQuery = { __typename?: 'Query', totalSalesOrders: number, salesOrders: Array<{ __typename?: 'SalesOrder', id: string, customerId: string, customerName: string, customerPhoneNumber: string, orderDate: string, netAmount: number, discAmount: number, taxableAmount: number, taxAmount: number, totalAmount: number, state: SalesOrderState, createdAt: string, updatedAt: string, customer: { __typename?: 'Customer', id: string, fullName: string, phone?: string | null, createdAt: string, updatedAt: string }, items: Array<{ __typename?: 'SalesOrderItem', id: string, orderId: string, itemId: string, itemName: string, quantity: number, priceAmount: number, taxAmount: number, createdAt: string, updatedAt: string }> }> };

export type GetPosCategoriesQueryVariables = Exact<{
  first: Scalars['Int']['input'];
}>;


export type GetPosCategoriesQuery = { __typename?: 'Query', itemCategories: Array<{ __typename?: 'ItemGroup', id: string, name: string, description?: string | null, state: ItemGroupState, createdAt: string, updatedAt: string }> };

export type GetPosItemsQueryVariables = Exact<{
  first: Scalars['Int']['input'];
  offset: Scalars['Int']['input'];
}>;


export type GetPosItemsQuery = { __typename?: 'Query', items: Array<{ __typename?: 'Item', id: string, name: string, description?: string | null, nature: ItemNature, state: ItemState, price: number, createdAt: string, updatedAt: string, category: { __typename?: 'ItemGroup', id: string, name: string, description?: string | null, state: ItemGroupState, createdAt: string, updatedAt: string }, taxes: Array<{ __typename?: 'Tax', id: string, name: string, rate: number, description?: string | null, createdAt: string, updatedAt: string }> }> };

export type GetPosTaxesQueryVariables = Exact<{ [key: string]: never; }>;


export type GetPosTaxesQuery = { __typename?: 'Query', taxes: Array<{ __typename?: 'Tax', id: string, name: string, rate: number, description?: string | null, createdAt: string, updatedAt: string }> };

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


export type CreateSalesOrderMutation = { __typename?: 'Mutation', createSalesOrder: { __typename?: 'SalesOrder', id: string, customerName: string, orderDate: string, netAmount: number, taxAmount: number, totalAmount: number, state: SalesOrderState } };

export type GetTaxesQueryVariables = Exact<{
  first: Scalars['Int']['input'];
  offset: Scalars['Int']['input'];
}>;


export type GetTaxesQuery = { __typename?: 'Query', totalTaxes: number, taxes: Array<{ __typename?: 'Tax', id: string, name: string, rate: number, description?: string | null, createdAt: string, updatedAt: string }> };

export type CreateTaxMutationVariables = Exact<{
  input: TaxNewInput;
}>;


export type CreateTaxMutation = { __typename?: 'Mutation', createTax: { __typename?: 'Tax', id: string, name: string, rate: number, description?: string | null, createdAt: string, updatedAt: string } };

export type UpdateTaxMutationVariables = Exact<{
  input: TaxUpdateInput;
}>;


export type UpdateTaxMutation = { __typename?: 'Mutation', updateTax: { __typename?: 'Tax', id: string, name: string, rate: number, description?: string | null, createdAt: string, updatedAt: string } };

export type DeleteTaxMutationVariables = Exact<{
  id: Scalars['DbUuid']['input'];
}>;


export type DeleteTaxMutation = { __typename?: 'Mutation', deleteTax: number };

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