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

export type Brand = {
  __typename?: 'Brand';
  createdAt: Scalars['LocalDateTime']['output'];
  description?: Maybe<Scalars['String']['output']>;
  id: Scalars['DbUuid']['output'];
  isActive: Scalars['Boolean']['output'];
  name: Scalars['String']['output'];
  updatedAt: Scalars['LocalDateTime']['output'];
};

export type BrandNewInput = {
  description?: InputMaybe<Scalars['String']['input']>;
  isActive?: InputMaybe<Scalars['Boolean']['input']>;
  name: Scalars['String']['input'];
};

export type BrandUpdateInput = {
  description?: InputMaybe<Scalars['String']['input']>;
  id: Scalars['DbUuid']['input'];
  isActive?: InputMaybe<Scalars['Boolean']['input']>;
  name?: InputMaybe<Scalars['String']['input']>;
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

export type Channel = {
  __typename?: 'Channel';
  createdAt: Scalars['LocalDateTime']['output'];
  description?: Maybe<Scalars['String']['output']>;
  id: Scalars['DbUuid']['output'];
  isActive: Scalars['Boolean']['output'];
  name: Scalars['String']['output'];
  updatedAt: Scalars['LocalDateTime']['output'];
};

export type ChannelNewInput = {
  description?: InputMaybe<Scalars['String']['input']>;
  isActive?: InputMaybe<Scalars['Boolean']['input']>;
  name: Scalars['String']['input'];
};

export type ChannelUpdateInput = {
  description?: InputMaybe<Scalars['String']['input']>;
  id: Scalars['DbUuid']['input'];
  isActive?: InputMaybe<Scalars['Boolean']['input']>;
  name?: InputMaybe<Scalars['String']['input']>;
};

/** Cost Center */
export type CostCenter = {
  __typename?: 'CostCenter';
  code: Scalars['String']['output'];
  createdAt: Scalars['LocalDateTime']['output'];
  description?: Maybe<Scalars['String']['output']>;
  id: Scalars['DbUuid']['output'];
  name: Scalars['String']['output'];
  state: CostCenterState;
  updatedAt: Scalars['LocalDateTime']['output'];
};

export enum CostCenterState {
  Active = 'ACTIVE',
  Inactive = 'INACTIVE'
}

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

export type Discount = {
  __typename?: 'Discount';
  createdAt: Scalars['LocalDateTime']['output'];
  description?: Maybe<Scalars['String']['output']>;
  discountType: DiscountType;
  endDate?: Maybe<Scalars['LocalDateTime']['output']>;
  id: Scalars['DbUuid']['output'];
  name: Scalars['String']['output'];
  scope: DiscountScope;
  startDate?: Maybe<Scalars['LocalDateTime']['output']>;
  state: DiscountState;
  updatedAt: Scalars['LocalDateTime']['output'];
  value: Scalars['Money']['output'];
};

export type DiscountNewInput = {
  description?: InputMaybe<Scalars['String']['input']>;
  discountType: DiscountType;
  endDate?: InputMaybe<Scalars['LocalDateTime']['input']>;
  name: Scalars['String']['input'];
  scope: DiscountScope;
  startDate?: InputMaybe<Scalars['LocalDateTime']['input']>;
  state?: InputMaybe<DiscountState>;
  value: Scalars['Money']['input'];
};

export enum DiscountScope {
  AllItems = 'ALL_ITEMS',
  SpecificItems = 'SPECIFIC_ITEMS'
}

export enum DiscountState {
  Active = 'ACTIVE',
  Expired = 'EXPIRED',
  Inactive = 'INACTIVE',
  Scheduled = 'SCHEDULED'
}

export enum DiscountType {
  FixedAmount = 'FIXED_AMOUNT',
  Percentage = 'PERCENTAGE'
}

export type DiscountUpdateInput = {
  description?: InputMaybe<Scalars['String']['input']>;
  discountType?: InputMaybe<DiscountType>;
  endDate?: InputMaybe<Scalars['LocalDateTime']['input']>;
  id: Scalars['DbUuid']['input'];
  name?: InputMaybe<Scalars['String']['input']>;
  scope?: InputMaybe<DiscountScope>;
  startDate?: InputMaybe<Scalars['LocalDateTime']['input']>;
  state?: InputMaybe<DiscountState>;
  value?: InputMaybe<Scalars['Money']['input']>;
};

export type Expense = {
  __typename?: 'Expense';
  amount: Scalars['Money']['output'];
  category: PurchaseCategory;
  categoryId: Scalars['DbUuid']['output'];
  costCenter: CostCenter;
  costCenterId: Scalars['DbUuid']['output'];
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
  costCenterId: Scalars['DbUuid']['input'];
  description?: InputMaybe<Scalars['String']['input']>;
  expenseDate: Scalars['LocalDateTime']['input'];
  title: Scalars['String']['input'];
};

export type ExpenseUpdateInput = {
  amount?: InputMaybe<Scalars['Money']['input']>;
  categoryId?: InputMaybe<Scalars['DbUuid']['input']>;
  costCenterId?: InputMaybe<Scalars['DbUuid']['input']>;
  description?: InputMaybe<Scalars['String']['input']>;
  expenseDate?: InputMaybe<Scalars['LocalDateTime']['input']>;
  id: Scalars['DbUuid']['input'];
  title?: InputMaybe<Scalars['String']['input']>;
};

export type Item = {
  __typename?: 'Item';
  category: ItemGroup;
  createdAt: Scalars['LocalDateTime']['output'];
  defaultVariant?: Maybe<ItemVariant>;
  description?: Maybe<Scalars['String']['output']>;
  hasVariants: Scalars['Boolean']['output'];
  id: Scalars['DbUuid']['output'];
  name: Scalars['String']['output'];
  nature: ItemNature;
  price: Scalars['Money']['output'];
  state: ItemState;
  taxes: Array<Tax>;
  updatedAt: Scalars['LocalDateTime']['output'];
  variants: Array<ItemVariant>;
};

export type ItemDiscountNewInput = {
  discountId: Scalars['DbUuid']['input'];
  itemId: Scalars['DbUuid']['input'];
};

/** A relationship between an item and a discount */
export type ItemDiscountObject = {
  __typename?: 'ItemDiscountObject';
  discountId: Scalars['DbUuid']['output'];
  itemId: Scalars['DbUuid']['output'];
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

export type ItemVariant = {
  __typename?: 'ItemVariant';
  createdAt: Scalars['LocalDateTime']['output'];
  finalPrice: Scalars['Money']['output'];
  id: Scalars['DbUuid']['output'];
  isDefault: Scalars['Boolean']['output'];
  item: Item;
  priceAdjustment?: Maybe<Scalars['Money']['output']>;
  sku?: Maybe<Scalars['String']['output']>;
  updatedAt: Scalars['LocalDateTime']['output'];
  variantValues: Array<VariantValue>;
};

export type ItemVariantNewInput = {
  isDefault?: InputMaybe<Scalars['Boolean']['input']>;
  itemId: Scalars['DbUuid']['input'];
  priceAdjustment?: InputMaybe<Scalars['Money']['input']>;
  sku?: InputMaybe<Scalars['String']['input']>;
  variantValueIds: Array<Scalars['DbUuid']['input']>;
};

export type ItemVariantUpdateInput = {
  id: Scalars['DbUuid']['input'];
  isDefault?: InputMaybe<Scalars['Boolean']['input']>;
  priceAdjustment?: InputMaybe<Scalars['Money']['input']>;
  sku?: InputMaybe<Scalars['String']['input']>;
  updatedAt?: InputMaybe<Scalars['LocalDateTime']['input']>;
};

export type Mutation = {
  __typename?: 'Mutation';
  addItemDiscount: ItemDiscountObject;
  addUser: User;
  assignTaxToGroup: Scalars['Int']['output'];
  assignTaxToItem: Scalars['Int']['output'];
  assignVariantValueToItemVariant: Scalars['Int']['output'];
  createBrand: Brand;
  createCart: Cart;
  createChannel: Channel;
  createCostCenter: CostCenter;
  createCustomer: Customer;
  createDiscount: Discount;
  createExpense: Expense;
  createItem: Item;
  createItemCategory: ItemGroup;
  createItemVariant: ItemVariant;
  createPaymentMethod: PaymentMethod;
  createPurchaseCategory: PurchaseCategory;
  createSalesChargeType: SalesChargeType;
  createSalesOrder: SalesOrder;
  createSalesOrderPayment: SalesOrderPayment;
  createSupplier: Supplier;
  createTax: Tax;
  createTaxGroup: TaxGroup;
  createVariantType: VariantType;
  createVariantValue: VariantValue;
  deleteBrand: Scalars['Int']['output'];
  deleteCart: Scalars['Int']['output'];
  deleteChannel: Scalars['Int']['output'];
  deleteCostCenter: Scalars['DbUuid']['output'];
  deleteCustomer: Scalars['Int']['output'];
  deleteDiscount: Scalars['Int']['output'];
  deleteExpense: Scalars['Int']['output'];
  deleteItem: Scalars['Int']['output'];
  deleteItemCategory: Scalars['Int']['output'];
  deleteItemVariant: Scalars['Int']['output'];
  deletePaymentMethod: Scalars['DbUuid']['output'];
  deletePurchaseCategory: Scalars['DbUuid']['output'];
  deleteSalesChargeType: Scalars['Boolean']['output'];
  deleteSupplier: Scalars['Int']['output'];
  deleteTax: Scalars['Int']['output'];
  deleteTaxGroup: Scalars['Int']['output'];
  deleteUser: Scalars['Int']['output'];
  deleteVariantType: Scalars['Int']['output'];
  deleteVariantValue: Scalars['Int']['output'];
  login: Scalars['Boolean']['output'];
  logout: Scalars['Boolean']['output'];
  removeItemDiscount: Scalars['Boolean']['output'];
  removeTaxFromGroup: Scalars['Int']['output'];
  removeTaxFromItem: Scalars['Int']['output'];
  removeVariantValueFromItemVariant: Scalars['Int']['output'];
  updateBrand: Brand;
  updateCart: Cart;
  updateChannel: Channel;
  updateCostCenter: CostCenter;
  updateCustomer: Customer;
  updateDiscount: Discount;
  updateExpense: Expense;
  updateItem: Item;
  updateItemCategory: ItemGroup;
  updateItemVariant: ItemVariant;
  updatePaymentMethod: PaymentMethod;
  updatePurchaseCategory: PurchaseCategory;
  updateSalesChargeType: SalesChargeType;
  updateSalesOrderPayment: SalesOrderPayment;
  updateSupplier: Supplier;
  updateTax: Tax;
  updateTaxGroup: TaxGroup;
  updateUser: User;
  updateVariantType: VariantType;
  updateVariantValue: VariantValue;
  voidSalesOrder: SalesOrder;
  voidSalesOrderPayment: SalesOrderPayment;
};


export type MutationAddItemDiscountArgs = {
  itemDiscount: ItemDiscountNewInput;
};


export type MutationAddUserArgs = {
  user: UserNewInput;
};


export type MutationAssignTaxToGroupArgs = {
  taxGroupId: Scalars['DbUuid']['input'];
  taxId: Scalars['DbUuid']['input'];
};


export type MutationAssignTaxToItemArgs = {
  input: ItemTaxNewInput;
};


export type MutationAssignVariantValueToItemVariantArgs = {
  itemVariantId: Scalars['DbUuid']['input'];
  variantValueId: Scalars['DbUuid']['input'];
};


export type MutationCreateBrandArgs = {
  input: BrandNewInput;
};


export type MutationCreateCartArgs = {
  cart: CartNewInput;
};


export type MutationCreateChannelArgs = {
  input: ChannelNewInput;
};


export type MutationCreateCostCenterArgs = {
  code: Scalars['String']['input'];
  description?: InputMaybe<Scalars['String']['input']>;
  name: Scalars['String']['input'];
  state?: InputMaybe<CostCenterState>;
};


export type MutationCreateCustomerArgs = {
  customer: CustomerNewInput;
};


export type MutationCreateDiscountArgs = {
  discount: DiscountNewInput;
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


export type MutationCreateItemVariantArgs = {
  input: ItemVariantNewInput;
};


export type MutationCreatePaymentMethodArgs = {
  code: Scalars['String']['input'];
  description?: InputMaybe<Scalars['String']['input']>;
  name: Scalars['String']['input'];
  state?: InputMaybe<PaymentMethodState>;
};


export type MutationCreatePurchaseCategoryArgs = {
  description?: InputMaybe<Scalars['String']['input']>;
  name: Scalars['String']['input'];
  state?: InputMaybe<PurchaseCategoryState>;
};


export type MutationCreateSalesChargeTypeArgs = {
  chargeType: SalesChargeTypeNewInput;
};


export type MutationCreateSalesOrderArgs = {
  salesOrder: SalesOrderNewInput;
};


export type MutationCreateSalesOrderPaymentArgs = {
  payment: SalesOrderPaymentNewInput;
};


export type MutationCreateSupplierArgs = {
  supplier: SupplierNewInput;
};


export type MutationCreateTaxArgs = {
  input: TaxNewInput;
};


export type MutationCreateTaxGroupArgs = {
  input: TaxGroupNewInput;
};


export type MutationCreateVariantTypeArgs = {
  input: VariantTypeNewInput;
};


export type MutationCreateVariantValueArgs = {
  input: VariantValueNewInput;
};


export type MutationDeleteBrandArgs = {
  id: Scalars['DbUuid']['input'];
};


export type MutationDeleteCartArgs = {
  id: Scalars['DbUuid']['input'];
};


export type MutationDeleteChannelArgs = {
  id: Scalars['DbUuid']['input'];
};


export type MutationDeleteCostCenterArgs = {
  id: Scalars['DbUuid']['input'];
};


export type MutationDeleteCustomerArgs = {
  id: Scalars['DbUuid']['input'];
};


export type MutationDeleteDiscountArgs = {
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


export type MutationDeleteItemVariantArgs = {
  id: Scalars['DbUuid']['input'];
};


export type MutationDeletePaymentMethodArgs = {
  id: Scalars['DbUuid']['input'];
};


export type MutationDeletePurchaseCategoryArgs = {
  id: Scalars['DbUuid']['input'];
};


export type MutationDeleteSalesChargeTypeArgs = {
  id: Scalars['DbUuid']['input'];
};


export type MutationDeleteSupplierArgs = {
  id: Scalars['DbUuid']['input'];
};


export type MutationDeleteTaxArgs = {
  id: Scalars['DbUuid']['input'];
};


export type MutationDeleteTaxGroupArgs = {
  id: Scalars['DbUuid']['input'];
};


export type MutationDeleteUserArgs = {
  id: Scalars['DbUuid']['input'];
};


export type MutationDeleteVariantTypeArgs = {
  id: Scalars['DbUuid']['input'];
};


export type MutationDeleteVariantValueArgs = {
  id: Scalars['DbUuid']['input'];
};


export type MutationLoginArgs = {
  password: Scalars['String']['input'];
  username: Scalars['String']['input'];
};


export type MutationRemoveItemDiscountArgs = {
  discountId: Scalars['DbUuid']['input'];
  itemId: Scalars['DbUuid']['input'];
};


export type MutationRemoveTaxFromGroupArgs = {
  taxGroupId: Scalars['DbUuid']['input'];
  taxId: Scalars['DbUuid']['input'];
};


export type MutationRemoveTaxFromItemArgs = {
  itemId: Scalars['DbUuid']['input'];
  taxId: Scalars['DbUuid']['input'];
};


export type MutationRemoveVariantValueFromItemVariantArgs = {
  itemVariantId: Scalars['DbUuid']['input'];
  variantValueId: Scalars['DbUuid']['input'];
};


export type MutationUpdateBrandArgs = {
  input: BrandUpdateInput;
};


export type MutationUpdateCartArgs = {
  cart: CartUpdateInput;
};


export type MutationUpdateChannelArgs = {
  input: ChannelUpdateInput;
};


export type MutationUpdateCostCenterArgs = {
  code?: InputMaybe<Scalars['String']['input']>;
  description?: InputMaybe<Scalars['String']['input']>;
  id: Scalars['DbUuid']['input'];
  name?: InputMaybe<Scalars['String']['input']>;
  state?: InputMaybe<CostCenterState>;
};


export type MutationUpdateCustomerArgs = {
  customer: CustomerUpdateInput;
};


export type MutationUpdateDiscountArgs = {
  discount: DiscountUpdateInput;
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


export type MutationUpdateItemVariantArgs = {
  input: ItemVariantUpdateInput;
};


export type MutationUpdatePaymentMethodArgs = {
  code?: InputMaybe<Scalars['String']['input']>;
  description?: InputMaybe<Scalars['String']['input']>;
  id: Scalars['DbUuid']['input'];
  name?: InputMaybe<Scalars['String']['input']>;
  state?: InputMaybe<PaymentMethodState>;
};


export type MutationUpdatePurchaseCategoryArgs = {
  description?: InputMaybe<Scalars['String']['input']>;
  id: Scalars['DbUuid']['input'];
  name?: InputMaybe<Scalars['String']['input']>;
  state?: InputMaybe<PurchaseCategoryState>;
};


export type MutationUpdateSalesChargeTypeArgs = {
  chargeType: SalesChargeTypeUpdateInput;
};


export type MutationUpdateSalesOrderPaymentArgs = {
  payment: SalesOrderPaymentUpdateInput;
};


export type MutationUpdateSupplierArgs = {
  supplier: SupplierUpdateInput;
};


export type MutationUpdateTaxArgs = {
  input: TaxUpdateInput;
};


export type MutationUpdateTaxGroupArgs = {
  input: TaxGroupUpdateInput;
};


export type MutationUpdateUserArgs = {
  user: UserUpdateInput;
};


export type MutationUpdateVariantTypeArgs = {
  input: VariantTypeUpdateInput;
};


export type MutationUpdateVariantValueArgs = {
  input: VariantValueUpdateInput;
};


export type MutationVoidSalesOrderArgs = {
  id: Scalars['DbUuid']['input'];
};


export type MutationVoidSalesOrderPaymentArgs = {
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

/** Payment Method */
export type PaymentMethod = {
  __typename?: 'PaymentMethod';
  code: Scalars['String']['output'];
  createdAt: Scalars['LocalDateTime']['output'];
  description?: Maybe<Scalars['String']['output']>;
  id: Scalars['DbUuid']['output'];
  name: Scalars['String']['output'];
  state: PaymentMethodState;
  updatedAt: Scalars['LocalDateTime']['output'];
};

export enum PaymentMethodState {
  Active = 'ACTIVE',
  Inactive = 'INACTIVE'
}

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
  activeBrands: Array<Brand>;
  activeChannels: Array<Channel>;
  allCostCenters: Array<CostCenter>;
  allPaymentMethods: Array<PaymentMethod>;
  allPurchaseCategories: Array<PurchaseCategory>;
  analyticsOverview: AnalyticsOverview;
  apiVersion: Scalars['String']['output'];
  brand: Brand;
  brands: Array<Brand>;
  cart: Cart;
  carts: Array<Cart>;
  channel: Channel;
  channels: Array<Channel>;
  costCenter: CostCenter;
  costCenters: Array<CostCenter>;
  customer: Customer;
  customerByPhone: Customer;
  customers: Array<Customer>;
  discount: Discount;
  discountItems: Array<ItemDiscountObject>;
  discounts: Array<Discount>;
  expense: Expense;
  expenses: Array<Expense>;
  expensesByCategory: Array<Expense>;
  item: Item;
  itemCategories: Array<ItemGroup>;
  itemDiscounts: Array<ItemDiscountObject>;
  itemVariant: ItemVariant;
  itemVariants: Array<ItemVariant>;
  items: Array<Item>;
  itemsCategory: ItemGroup;
  paymentMethod: PaymentMethod;
  paymentMethods: Array<PaymentMethod>;
  purchaseCategories: Array<PurchaseCategory>;
  purchaseCategory: PurchaseCategory;
  salesChargeType: SalesChargeType;
  salesChargeTypes: Array<SalesChargeType>;
  salesChargeTypesCount: Scalars['Int']['output'];
  salesOrder: SalesOrder;
  salesOrderPayments: Array<SalesOrderPayment>;
  salesOrders: Array<SalesOrder>;
  supplier: Supplier;
  suppliers: Array<Supplier>;
  tax: Tax;
  taxGroup: TaxGroup;
  taxGroups: Array<TaxGroup>;
  taxes: Array<Tax>;
  totalCarts: Scalars['Int']['output'];
  totalCostCenters: Scalars['Int']['output'];
  totalCustomers: Scalars['Int']['output'];
  totalExpenses: Scalars['Int']['output'];
  totalPaymentMethods: Scalars['Int']['output'];
  totalSalesOrders: Scalars['Int']['output'];
  totalSuppliers: Scalars['Int']['output'];
  totalTaxGroups: Scalars['Int']['output'];
  totalTaxes: Scalars['Int']['output'];
  totalVariantTypes: Scalars['Int']['output'];
  user: User;
  users: Array<User>;
  variantType: VariantType;
  variantTypes: Array<VariantType>;
  variantValue: VariantValue;
  variantValues: Array<VariantValue>;
};


export type QueryAnalyticsOverviewArgs = {
  days?: InputMaybe<Scalars['Int']['input']>;
};


export type QueryBrandArgs = {
  id: Scalars['DbUuid']['input'];
};


export type QueryCartArgs = {
  id: Scalars['DbUuid']['input'];
};


export type QueryCartsArgs = {
  first?: InputMaybe<Scalars['Int']['input']>;
  offset?: InputMaybe<Scalars['Int']['input']>;
};


export type QueryChannelArgs = {
  id: Scalars['DbUuid']['input'];
};


export type QueryCostCenterArgs = {
  id: Scalars['DbUuid']['input'];
};


export type QueryCostCentersArgs = {
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


export type QueryDiscountArgs = {
  id: Scalars['DbUuid']['input'];
};


export type QueryDiscountItemsArgs = {
  discountId: Scalars['DbUuid']['input'];
};


export type QueryDiscountsArgs = {
  first?: InputMaybe<Scalars['Int']['input']>;
  offset?: InputMaybe<Scalars['Int']['input']>;
  state?: InputMaybe<DiscountState>;
};


export type QueryExpenseArgs = {
  id: Scalars['DbUuid']['input'];
};


export type QueryExpensesArgs = {
  costCenterId?: InputMaybe<Scalars['DbUuid']['input']>;
  endDate?: InputMaybe<Scalars['String']['input']>;
  first?: InputMaybe<Scalars['Int']['input']>;
  offset?: InputMaybe<Scalars['Int']['input']>;
  startDate?: InputMaybe<Scalars['String']['input']>;
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


export type QueryItemDiscountsArgs = {
  itemId: Scalars['DbUuid']['input'];
};


export type QueryItemVariantArgs = {
  id: Scalars['DbUuid']['input'];
};


export type QueryItemVariantsArgs = {
  first?: InputMaybe<Scalars['Int']['input']>;
  itemId?: InputMaybe<Scalars['DbUuid']['input']>;
  offset?: InputMaybe<Scalars['Int']['input']>;
};


export type QueryItemsArgs = {
  first?: InputMaybe<Scalars['Int']['input']>;
  offset?: InputMaybe<Scalars['Int']['input']>;
};


export type QueryItemsCategoryArgs = {
  id: Scalars['DbUuid']['input'];
};


export type QueryPaymentMethodArgs = {
  id: Scalars['DbUuid']['input'];
};


export type QueryPaymentMethodsArgs = {
  first?: InputMaybe<Scalars['Int']['input']>;
  offset?: InputMaybe<Scalars['Int']['input']>;
};


export type QueryPurchaseCategoriesArgs = {
  first?: InputMaybe<Scalars['Int']['input']>;
  offset?: InputMaybe<Scalars['Int']['input']>;
};


export type QueryPurchaseCategoryArgs = {
  id: Scalars['DbUuid']['input'];
};


export type QuerySalesChargeTypeArgs = {
  id: Scalars['DbUuid']['input'];
};


export type QuerySalesChargeTypesArgs = {
  first?: InputMaybe<Scalars['Int']['input']>;
  offset?: InputMaybe<Scalars['Int']['input']>;
};


export type QuerySalesOrderArgs = {
  id: Scalars['DbUuid']['input'];
};


export type QuerySalesOrderPaymentsArgs = {
  orderId: Scalars['DbUuid']['input'];
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


export type QueryTaxGroupArgs = {
  id: Scalars['DbUuid']['input'];
};


export type QueryTaxGroupsArgs = {
  first?: InputMaybe<Scalars['Int']['input']>;
  offset?: InputMaybe<Scalars['Int']['input']>;
};


export type QueryTaxesArgs = {
  first?: InputMaybe<Scalars['Int']['input']>;
  offset?: InputMaybe<Scalars['Int']['input']>;
};


export type QueryTotalExpensesArgs = {
  costCenterId?: InputMaybe<Scalars['DbUuid']['input']>;
  endDate?: InputMaybe<Scalars['String']['input']>;
  startDate?: InputMaybe<Scalars['String']['input']>;
};


export type QueryUserArgs = {
  id: Scalars['DbUuid']['input'];
};


export type QueryUsersArgs = {
  first?: InputMaybe<Scalars['Int']['input']>;
  offset?: InputMaybe<Scalars['Int']['input']>;
};


export type QueryVariantTypeArgs = {
  id: Scalars['DbUuid']['input'];
};


export type QueryVariantTypesArgs = {
  first?: InputMaybe<Scalars['Int']['input']>;
  offset?: InputMaybe<Scalars['Int']['input']>;
};


export type QueryVariantValueArgs = {
  id: Scalars['DbUuid']['input'];
};


export type QueryVariantValuesArgs = {
  first?: InputMaybe<Scalars['Int']['input']>;
  offset?: InputMaybe<Scalars['Int']['input']>;
  variantTypeId?: InputMaybe<Scalars['DbUuid']['input']>;
};

export type SalesChargeType = {
  __typename?: 'SalesChargeType';
  createdAt: Scalars['LocalDateTime']['output'];
  description?: Maybe<Scalars['String']['output']>;
  id: Scalars['DbUuid']['output'];
  name: Scalars['String']['output'];
  updatedAt: Scalars['LocalDateTime']['output'];
};

export type SalesChargeTypeNewInput = {
  description?: InputMaybe<Scalars['String']['input']>;
  name: Scalars['String']['input'];
};

export type SalesChargeTypeUpdateInput = {
  description?: InputMaybe<Scalars['String']['input']>;
  id: Scalars['DbUuid']['input'];
  name?: InputMaybe<Scalars['String']['input']>;
};

export type SalesOrder = {
  __typename?: 'SalesOrder';
  billingAddress?: Maybe<Scalars['String']['output']>;
  channelId: Scalars['DbUuid']['output'];
  charges: Array<SalesOrderCharge>;
  costCenter: CostCenter;
  costCenterId: Scalars['DbUuid']['output'];
  createdAt: Scalars['LocalDateTime']['output'];
  createdBy: Scalars['DbUuid']['output'];
  customer?: Maybe<Customer>;
  customerId?: Maybe<Scalars['DbUuid']['output']>;
  customerName?: Maybe<Scalars['String']['output']>;
  customerPhoneNumber?: Maybe<Scalars['String']['output']>;
  discAmount: Scalars['Money']['output'];
  discountId?: Maybe<Scalars['DbUuid']['output']>;
  id: Scalars['DbUuid']['output'];
  items: Array<SalesOrderItem>;
  locationId: Scalars['DbUuid']['output'];
  netAmount: Scalars['Money']['output'];
  notes?: Maybe<Scalars['String']['output']>;
  orderDate: Scalars['LocalDateTime']['output'];
  orderReadableId: Scalars['String']['output'];
  orderState: SalesOrderState;
  paymentState: SalesOrderPaymentState;
  payments: Array<SalesOrderPayment>;
  shippingAddress?: Maybe<Scalars['String']['output']>;
  taxAmount: Scalars['Money']['output'];
  taxableAmount: Scalars['Money']['output'];
  totalAmount: Scalars['Money']['output'];
  totalPaidAmount: Scalars['Money']['output'];
  updatedAt: Scalars['LocalDateTime']['output'];
  updatedBy: Scalars['DbUuid']['output'];
};

export type SalesOrderCharge = {
  __typename?: 'SalesOrderCharge';
  amount: Scalars['Money']['output'];
  chargeTypeId: Scalars['DbUuid']['output'];
  chargeTypeName: Scalars['String']['output'];
  createdAt: Scalars['LocalDateTime']['output'];
  id: Scalars['DbUuid']['output'];
  orderId: Scalars['DbUuid']['output'];
  taxAmount: Scalars['Money']['output'];
  taxGroupId?: Maybe<Scalars['DbUuid']['output']>;
  updatedAt: Scalars['LocalDateTime']['output'];
};

export type SalesOrderChargeNewInput = {
  amount: Scalars['Money']['input'];
  chargeTypeId: Scalars['DbUuid']['input'];
  chargeTypeName: Scalars['String']['input'];
  taxAmount: Scalars['Money']['input'];
  taxGroupId?: InputMaybe<Scalars['DbUuid']['input']>;
};

export type SalesOrderItem = {
  __typename?: 'SalesOrderItem';
  createdAt: Scalars['LocalDateTime']['output'];
  id: Scalars['DbUuid']['output'];
  itemId?: Maybe<Scalars['DbUuid']['output']>;
  itemName: Scalars['String']['output'];
  orderId: Scalars['DbUuid']['output'];
  priceAmount: Scalars['Money']['output'];
  quantity: Scalars['Int']['output'];
  taxAmount: Scalars['Money']['output'];
  totalAmount: Scalars['Money']['output'];
  updatedAt: Scalars['LocalDateTime']['output'];
};

export type SalesOrderItemInput = {
  discAmount: Scalars['Money']['input'];
  itemId?: InputMaybe<Scalars['DbUuid']['input']>;
  itemName: Scalars['String']['input'];
  priceAmount: Scalars['Money']['input'];
  quantity: Scalars['Int']['input'];
  sku?: InputMaybe<Scalars['String']['input']>;
  taxAmount: Scalars['Money']['input'];
  taxableAmount: Scalars['Money']['input'];
  totalAmount: Scalars['Money']['input'];
};

export type SalesOrderNewInput = {
  billingAddress?: InputMaybe<Scalars['String']['input']>;
  channelId: Scalars['DbUuid']['input'];
  charges?: InputMaybe<Array<SalesOrderChargeNewInput>>;
  costCenterId: Scalars['DbUuid']['input'];
  customerId?: InputMaybe<Scalars['DbUuid']['input']>;
  customerName?: InputMaybe<Scalars['String']['input']>;
  customerPhoneNumber?: InputMaybe<Scalars['String']['input']>;
  discAmount: Scalars['Money']['input'];
  discountId?: InputMaybe<Scalars['DbUuid']['input']>;
  items: Array<SalesOrderItemInput>;
  locationId: Scalars['DbUuid']['input'];
  netAmount: Scalars['Money']['input'];
  notes?: InputMaybe<Scalars['String']['input']>;
  orderDate: Scalars['LocalDateTime']['input'];
  shippingAddress?: InputMaybe<Scalars['String']['input']>;
  taxAmount: Scalars['Money']['input'];
  taxableAmount: Scalars['Money']['input'];
  totalAmount: Scalars['Money']['input'];
};

/** Sales Order Payment */
export type SalesOrderPayment = {
  __typename?: 'SalesOrderPayment';
  amount: Scalars['Money']['output'];
  id: Scalars['DbUuid']['output'];
  notes?: Maybe<Scalars['String']['output']>;
  orderId: Scalars['DbUuid']['output'];
  paymentDate: Scalars['LocalDateTime']['output'];
  paymentMethodId: Scalars['DbUuid']['output'];
  referenceNumber?: Maybe<Scalars['String']['output']>;
  state: SalesOrderPaymentState;
};

export type SalesOrderPaymentNewInput = {
  amount: Scalars['Money']['input'];
  notes?: InputMaybe<Scalars['String']['input']>;
  orderId: Scalars['DbUuid']['input'];
  paymentDate: Scalars['LocalDateTime']['input'];
  paymentMethodId: Scalars['DbUuid']['input'];
  referenceNumber?: InputMaybe<Scalars['String']['input']>;
  state?: InputMaybe<SalesOrderPaymentState>;
};

export enum SalesOrderPaymentState {
  Failed = 'FAILED',
  Paid = 'PAID',
  PartiallyPaid = 'PARTIALLY_PAID',
  PartiallyRefunded = 'PARTIALLY_REFUNDED',
  Pending = 'PENDING',
  Refunded = 'REFUNDED',
  Voided = 'VOIDED'
}

export type SalesOrderPaymentUpdateInput = {
  amount?: InputMaybe<Scalars['Money']['input']>;
  id: Scalars['DbUuid']['input'];
  notes?: InputMaybe<Scalars['String']['input']>;
  paymentDate?: InputMaybe<Scalars['LocalDateTime']['input']>;
  paymentMethodId?: InputMaybe<Scalars['DbUuid']['input']>;
  referenceNumber?: InputMaybe<Scalars['String']['input']>;
  state?: InputMaybe<SalesOrderPaymentState>;
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

export type TaxGroup = {
  __typename?: 'TaxGroup';
  createdAt: Scalars['LocalDateTime']['output'];
  description?: Maybe<Scalars['String']['output']>;
  id: Scalars['DbUuid']['output'];
  name: Scalars['String']['output'];
  taxes: Array<Tax>;
  updatedAt: Scalars['LocalDateTime']['output'];
};

export type TaxGroupNewInput = {
  description?: InputMaybe<Scalars['String']['input']>;
  name: Scalars['String']['input'];
  taxIds?: InputMaybe<Array<Scalars['DbUuid']['input']>>;
};

export type TaxGroupUpdateInput = {
  description?: InputMaybe<Scalars['String']['input']>;
  id: Scalars['DbUuid']['input'];
  name?: InputMaybe<Scalars['String']['input']>;
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

export type VariantType = {
  __typename?: 'VariantType';
  createdAt: Scalars['LocalDateTime']['output'];
  description?: Maybe<Scalars['String']['output']>;
  id: Scalars['DbUuid']['output'];
  name: Scalars['String']['output'];
  updatedAt: Scalars['LocalDateTime']['output'];
  values: Array<VariantValue>;
};

export type VariantTypeNewInput = {
  description?: InputMaybe<Scalars['String']['input']>;
  name: Scalars['String']['input'];
};

export type VariantTypeUpdateInput = {
  description?: InputMaybe<Scalars['String']['input']>;
  id: Scalars['DbUuid']['input'];
  name?: InputMaybe<Scalars['String']['input']>;
  updatedAt?: InputMaybe<Scalars['LocalDateTime']['input']>;
};

export type VariantValue = {
  __typename?: 'VariantValue';
  createdAt: Scalars['LocalDateTime']['output'];
  displayOrder: Scalars['Int']['output'];
  id: Scalars['DbUuid']['output'];
  updatedAt: Scalars['LocalDateTime']['output'];
  value: Scalars['String']['output'];
  variantType: VariantType;
};

export type VariantValueNewInput = {
  displayOrder?: InputMaybe<Scalars['Int']['input']>;
  value: Scalars['String']['input'];
  variantTypeId: Scalars['DbUuid']['input'];
};

export type VariantValueUpdateInput = {
  displayOrder?: InputMaybe<Scalars['Int']['input']>;
  id: Scalars['DbUuid']['input'];
  updatedAt?: InputMaybe<Scalars['LocalDateTime']['input']>;
  value?: InputMaybe<Scalars['String']['input']>;
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

export type GetDiscountsQueryVariables = Exact<{
  first?: InputMaybe<Scalars['Int']['input']>;
  offset?: InputMaybe<Scalars['Int']['input']>;
  state?: InputMaybe<DiscountState>;
}>;


export type GetDiscountsQuery = { __typename?: 'Query', discounts: Array<{ __typename?: 'Discount', id: string, name: string, description?: string | null, discountType: DiscountType, value: string, scope: DiscountScope, state: DiscountState, startDate?: string | null, endDate?: string | null, createdAt: string, updatedAt: string }> };

export type GetDiscountQueryVariables = Exact<{
  id: Scalars['DbUuid']['input'];
}>;


export type GetDiscountQuery = { __typename?: 'Query', discount: { __typename?: 'Discount', id: string, name: string, description?: string | null, discountType: DiscountType, value: string, scope: DiscountScope, state: DiscountState, startDate?: string | null, endDate?: string | null, createdAt: string, updatedAt: string } };

export type CreateDiscountMutationVariables = Exact<{
  discount: DiscountNewInput;
}>;


export type CreateDiscountMutation = { __typename?: 'Mutation', createDiscount: { __typename?: 'Discount', id: string, name: string, description?: string | null, discountType: DiscountType, value: string, scope: DiscountScope, state: DiscountState, startDate?: string | null, endDate?: string | null, createdAt: string, updatedAt: string } };

export type UpdateDiscountMutationVariables = Exact<{
  discount: DiscountUpdateInput;
}>;


export type UpdateDiscountMutation = { __typename?: 'Mutation', updateDiscount: { __typename?: 'Discount', id: string, name: string, description?: string | null, discountType: DiscountType, value: string, scope: DiscountScope, state: DiscountState, startDate?: string | null, endDate?: string | null, createdAt: string, updatedAt: string } };

export type DeleteDiscountMutationVariables = Exact<{
  id: Scalars['DbUuid']['input'];
}>;


export type DeleteDiscountMutation = { __typename?: 'Mutation', deleteDiscount: number };

export type GetItemsQueryVariables = Exact<{
  first: Scalars['Int']['input'];
  offset: Scalars['Int']['input'];
}>;


export type GetItemsQuery = { __typename?: 'Query', items: Array<{ __typename?: 'Item', id: string, name: string, description?: string | null, nature: ItemNature, state: ItemState, price: string, createdAt: string, updatedAt: string, hasVariants: boolean, category: { __typename?: 'ItemGroup', id: string, name: string, description?: string | null, state: ItemGroupState, createdAt: string, updatedAt: string }, taxes: Array<{ __typename?: 'Tax', id: string, name: string, rate: string, description?: string | null, createdAt: string, updatedAt: string }>, variants: Array<{ __typename?: 'ItemVariant', id: string, sku?: string | null, priceAdjustment?: string | null, isDefault: boolean, finalPrice: string, variantValues: Array<{ __typename?: 'VariantValue', id: string, value: string, variantType: { __typename?: 'VariantType', id: string, name: string } }> }> }> };

export type GetItemQueryVariables = Exact<{
  id: Scalars['DbUuid']['input'];
}>;


export type GetItemQuery = { __typename?: 'Query', item: { __typename?: 'Item', id: string, name: string, description?: string | null, nature: ItemNature, state: ItemState, price: string, createdAt: string, updatedAt: string, hasVariants: boolean, category: { __typename?: 'ItemGroup', id: string, name: string, description?: string | null, state: ItemGroupState, createdAt: string, updatedAt: string }, taxes: Array<{ __typename?: 'Tax', id: string, name: string, rate: string, description?: string | null, createdAt: string, updatedAt: string }>, variants: Array<{ __typename?: 'ItemVariant', id: string, sku?: string | null, priceAdjustment?: string | null, isDefault: boolean, finalPrice: string, variantValues: Array<{ __typename?: 'VariantValue', id: string, value: string, variantType: { __typename?: 'VariantType', id: string, name: string } }> }> } };

export type GetItemCategoriesQueryVariables = Exact<{ [key: string]: never; }>;


export type GetItemCategoriesQuery = { __typename?: 'Query', itemCategories: Array<{ __typename?: 'ItemGroup', id: string, name: string, description?: string | null, state: ItemGroupState, createdAt: string, updatedAt: string }> };

export type GetItemTaxesQueryVariables = Exact<{ [key: string]: never; }>;


export type GetItemTaxesQuery = { __typename?: 'Query', taxes: Array<{ __typename?: 'Tax', id: string, name: string, rate: string, description?: string | null, createdAt: string, updatedAt: string }> };

export type CreateItemMutationVariables = Exact<{
  input: NewItem;
}>;


export type CreateItemMutation = { __typename?: 'Mutation', createItem: { __typename?: 'Item', id: string, name: string, description?: string | null, nature: ItemNature, state: ItemState, price: string, createdAt: string, updatedAt: string, hasVariants: boolean, category: { __typename?: 'ItemGroup', id: string, name: string, description?: string | null, state: ItemGroupState, createdAt: string, updatedAt: string }, taxes: Array<{ __typename?: 'Tax', id: string, name: string, rate: string, description?: string | null, createdAt: string, updatedAt: string }>, variants: Array<{ __typename?: 'ItemVariant', id: string, sku?: string | null, priceAdjustment?: string | null, isDefault: boolean, finalPrice: string, variantValues: Array<{ __typename?: 'VariantValue', id: string, value: string, variantType: { __typename?: 'VariantType', id: string, name: string } }> }> } };

export type UpdateItemMutationVariables = Exact<{
  input: UpdateItem;
}>;


export type UpdateItemMutation = { __typename?: 'Mutation', updateItem: { __typename?: 'Item', id: string, name: string, description?: string | null, nature: ItemNature, state: ItemState, price: string, createdAt: string, updatedAt: string, hasVariants: boolean, category: { __typename?: 'ItemGroup', id: string, name: string, description?: string | null, state: ItemGroupState, createdAt: string, updatedAt: string }, taxes: Array<{ __typename?: 'Tax', id: string, name: string, rate: string, description?: string | null, createdAt: string, updatedAt: string }>, variants: Array<{ __typename?: 'ItemVariant', id: string, sku?: string | null, priceAdjustment?: string | null, isDefault: boolean, finalPrice: string, variantValues: Array<{ __typename?: 'VariantValue', id: string, value: string, variantType: { __typename?: 'VariantType', id: string, name: string } }> }> } };

export type DeleteItemMutationVariables = Exact<{
  id: Scalars['DbUuid']['input'];
}>;


export type DeleteItemMutation = { __typename?: 'Mutation', deleteItem: number };

export type GetItemDiscountsQueryVariables = Exact<{
  itemId: Scalars['DbUuid']['input'];
}>;


export type GetItemDiscountsQuery = { __typename?: 'Query', itemDiscounts: Array<{ __typename?: 'ItemDiscountObject', itemId: string, discountId: string }> };

export type GetDiscountItemsQueryVariables = Exact<{
  discountId: Scalars['DbUuid']['input'];
}>;


export type GetDiscountItemsQuery = { __typename?: 'Query', discountItems: Array<{ __typename?: 'ItemDiscountObject', itemId: string, discountId: string }> };

export type AddItemDiscountMutationVariables = Exact<{
  itemDiscount: ItemDiscountNewInput;
}>;


export type AddItemDiscountMutation = { __typename?: 'Mutation', addItemDiscount: { __typename?: 'ItemDiscountObject', itemId: string, discountId: string } };

export type RemoveItemDiscountMutationVariables = Exact<{
  itemId: Scalars['DbUuid']['input'];
  discountId: Scalars['DbUuid']['input'];
}>;


export type RemoveItemDiscountMutation = { __typename?: 'Mutation', removeItemDiscount: boolean };

export type GetVariantTypesQueryVariables = Exact<{
  first: Scalars['Int']['input'];
  offset: Scalars['Int']['input'];
}>;


export type GetVariantTypesQuery = { __typename?: 'Query', variantTypes: Array<{ __typename?: 'VariantType', id: string, name: string, description?: string | null, createdAt: string, updatedAt: string }> };

export type GetVariantTypeQueryVariables = Exact<{
  id: Scalars['DbUuid']['input'];
}>;


export type GetVariantTypeQuery = { __typename?: 'Query', variantType: { __typename?: 'VariantType', id: string, name: string, description?: string | null, createdAt: string, updatedAt: string, values: Array<{ __typename?: 'VariantValue', id: string, value: string, displayOrder: number, createdAt: string, updatedAt: string }> } };

export type GetTotalVariantTypesQueryVariables = Exact<{ [key: string]: never; }>;


export type GetTotalVariantTypesQuery = { __typename?: 'Query', totalVariantTypes: number };

export type GetVariantValuesQueryVariables = Exact<{
  variantTypeId?: InputMaybe<Scalars['DbUuid']['input']>;
  first: Scalars['Int']['input'];
  offset: Scalars['Int']['input'];
}>;


export type GetVariantValuesQuery = { __typename?: 'Query', variantValues: Array<{ __typename?: 'VariantValue', id: string, value: string, displayOrder: number, createdAt: string, updatedAt: string, variantType: { __typename?: 'VariantType', id: string, name: string, description?: string | null } }> };

export type GetVariantValueQueryVariables = Exact<{
  id: Scalars['DbUuid']['input'];
}>;


export type GetVariantValueQuery = { __typename?: 'Query', variantValue: { __typename?: 'VariantValue', id: string, value: string, displayOrder: number, createdAt: string, updatedAt: string, variantType: { __typename?: 'VariantType', id: string, name: string, description?: string | null } } };

export type GetItemVariantsQueryVariables = Exact<{
  itemId: Scalars['DbUuid']['input'];
  first: Scalars['Int']['input'];
  offset: Scalars['Int']['input'];
}>;


export type GetItemVariantsQuery = { __typename?: 'Query', itemVariants: Array<{ __typename?: 'ItemVariant', id: string, sku?: string | null, priceAdjustment?: string | null, isDefault: boolean, createdAt: string, updatedAt: string, finalPrice: string, variantValues: Array<{ __typename?: 'VariantValue', id: string, value: string, displayOrder: number, variantType: { __typename?: 'VariantType', id: string, name: string } }> }> };

export type GetItemVariantQueryVariables = Exact<{
  id: Scalars['DbUuid']['input'];
}>;


export type GetItemVariantQuery = { __typename?: 'Query', itemVariant: { __typename?: 'ItemVariant', id: string, sku?: string | null, priceAdjustment?: string | null, isDefault: boolean, createdAt: string, updatedAt: string, finalPrice: string, item: { __typename?: 'Item', id: string, name: string, price: string }, variantValues: Array<{ __typename?: 'VariantValue', id: string, value: string, displayOrder: number, variantType: { __typename?: 'VariantType', id: string, name: string } }> } };

export type CreateVariantTypeMutationVariables = Exact<{
  input: VariantTypeNewInput;
}>;


export type CreateVariantTypeMutation = { __typename?: 'Mutation', createVariantType: { __typename?: 'VariantType', id: string, name: string, description?: string | null, createdAt: string, updatedAt: string } };

export type UpdateVariantTypeMutationVariables = Exact<{
  input: VariantTypeUpdateInput;
}>;


export type UpdateVariantTypeMutation = { __typename?: 'Mutation', updateVariantType: { __typename?: 'VariantType', id: string, name: string, description?: string | null, createdAt: string, updatedAt: string } };

export type DeleteVariantTypeMutationVariables = Exact<{
  id: Scalars['DbUuid']['input'];
}>;


export type DeleteVariantTypeMutation = { __typename?: 'Mutation', deleteVariantType: number };

export type CreateVariantValueMutationVariables = Exact<{
  input: VariantValueNewInput;
}>;


export type CreateVariantValueMutation = { __typename?: 'Mutation', createVariantValue: { __typename?: 'VariantValue', id: string, value: string, displayOrder: number, createdAt: string, updatedAt: string, variantType: { __typename?: 'VariantType', id: string, name: string } } };

export type UpdateVariantValueMutationVariables = Exact<{
  input: VariantValueUpdateInput;
}>;


export type UpdateVariantValueMutation = { __typename?: 'Mutation', updateVariantValue: { __typename?: 'VariantValue', id: string, value: string, displayOrder: number, createdAt: string, updatedAt: string } };

export type DeleteVariantValueMutationVariables = Exact<{
  id: Scalars['DbUuid']['input'];
}>;


export type DeleteVariantValueMutation = { __typename?: 'Mutation', deleteVariantValue: number };

export type CreateItemVariantMutationVariables = Exact<{
  input: ItemVariantNewInput;
}>;


export type CreateItemVariantMutation = { __typename?: 'Mutation', createItemVariant: { __typename?: 'ItemVariant', id: string, sku?: string | null, priceAdjustment?: string | null, isDefault: boolean, createdAt: string, updatedAt: string, finalPrice: string } };

export type UpdateItemVariantMutationVariables = Exact<{
  input: ItemVariantUpdateInput;
}>;


export type UpdateItemVariantMutation = { __typename?: 'Mutation', updateItemVariant: { __typename?: 'ItemVariant', id: string, sku?: string | null, priceAdjustment?: string | null, isDefault: boolean, createdAt: string, updatedAt: string, finalPrice: string } };

export type DeleteItemVariantMutationVariables = Exact<{
  id: Scalars['DbUuid']['input'];
}>;


export type DeleteItemVariantMutation = { __typename?: 'Mutation', deleteItemVariant: number };

export type AssignVariantValueToItemVariantMutationVariables = Exact<{
  itemVariantId: Scalars['DbUuid']['input'];
  variantValueId: Scalars['DbUuid']['input'];
}>;


export type AssignVariantValueToItemVariantMutation = { __typename?: 'Mutation', assignVariantValueToItemVariant: number };

export type RemoveVariantValueFromItemVariantMutationVariables = Exact<{
  itemVariantId: Scalars['DbUuid']['input'];
  variantValueId: Scalars['DbUuid']['input'];
}>;


export type RemoveVariantValueFromItemVariantMutation = { __typename?: 'Mutation', removeVariantValueFromItemVariant: number };

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

export type GetPosCategoriesQueryVariables = Exact<{
  first: Scalars['Int']['input'];
}>;


export type GetPosCategoriesQuery = { __typename?: 'Query', itemCategories: Array<{ __typename?: 'ItemGroup', id: string, name: string, description?: string | null, state: ItemGroupState, createdAt: string, updatedAt: string }> };

export type GetPosItemsQueryVariables = Exact<{
  first: Scalars['Int']['input'];
  offset: Scalars['Int']['input'];
}>;


export type GetPosItemsQuery = { __typename?: 'Query', items: Array<{ __typename?: 'Item', id: string, name: string, description?: string | null, nature: ItemNature, state: ItemState, price: string, createdAt: string, updatedAt: string, hasVariants: boolean, category: { __typename?: 'ItemGroup', id: string, name: string, description?: string | null, state: ItemGroupState, createdAt: string, updatedAt: string }, taxes: Array<{ __typename?: 'Tax', id: string, name: string, rate: string, description?: string | null, createdAt: string, updatedAt: string }>, variants: Array<{ __typename?: 'ItemVariant', id: string, sku?: string | null, priceAdjustment?: string | null, isDefault: boolean, finalPrice: string, variantValues: Array<{ __typename?: 'VariantValue', id: string, value: string, variantType: { __typename?: 'VariantType', id: string, name: string } }> }> }> };

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

export type PosCreateSalesOrderMutationVariables = Exact<{
  salesOrder: SalesOrderNewInput;
}>;


export type PosCreateSalesOrderMutation = { __typename?: 'Mutation', createSalesOrder: { __typename?: 'SalesOrder', id: string, customerName?: string | null, orderDate: string, netAmount: string, taxAmount: string, totalAmount: string, orderState: SalesOrderState } };

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
  state?: InputMaybe<PurchaseCategoryState>;
}>;


export type CreatePurchaseCategoryMutation = { __typename?: 'Mutation', createPurchaseCategory: { __typename?: 'PurchaseCategory', id: string, name: string, description?: string | null, state: PurchaseCategoryState, createdAt: string, updatedAt: string } };

export type UpdatePurchaseCategoryMutationVariables = Exact<{
  id: Scalars['DbUuid']['input'];
  name?: InputMaybe<Scalars['String']['input']>;
  description?: InputMaybe<Scalars['String']['input']>;
  state?: InputMaybe<PurchaseCategoryState>;
}>;


export type UpdatePurchaseCategoryMutation = { __typename?: 'Mutation', updatePurchaseCategory: { __typename?: 'PurchaseCategory', id: string, name: string, description?: string | null, state: PurchaseCategoryState, createdAt: string, updatedAt: string } };

export type DeletePurchaseCategoryMutationVariables = Exact<{
  id: Scalars['DbUuid']['input'];
}>;


export type DeletePurchaseCategoryMutation = { __typename?: 'Mutation', deletePurchaseCategory: string };

export type GetExpensesQueryVariables = Exact<{
  first: Scalars['Int']['input'];
  offset: Scalars['Int']['input'];
  costCenterId?: InputMaybe<Scalars['DbUuid']['input']>;
  startDate?: InputMaybe<Scalars['String']['input']>;
  endDate?: InputMaybe<Scalars['String']['input']>;
}>;


export type GetExpensesQuery = { __typename?: 'Query', totalExpenses: number, expenses: Array<{ __typename?: 'Expense', id: string, title: string, amount: string, expenseDate: string, categoryId: string, costCenterId: string, description?: string | null, createdAt: string, updatedAt: string, category: { __typename?: 'PurchaseCategory', id: string, name: string }, costCenter: { __typename?: 'CostCenter', id: string, name: string, code: string } }> };

export type GetPurchaseCategoriesForExpensesQueryVariables = Exact<{ [key: string]: never; }>;


export type GetPurchaseCategoriesForExpensesQuery = { __typename?: 'Query', allPurchaseCategories: Array<{ __typename?: 'PurchaseCategory', id: string, name: string }> };

export type GetCostCentersForExpensesQueryVariables = Exact<{ [key: string]: never; }>;


export type GetCostCentersForExpensesQuery = { __typename?: 'Query', allCostCenters: Array<{ __typename?: 'CostCenter', id: string, name: string, code: string, state: CostCenterState }> };

export type GetExpensesByCategoryQueryVariables = Exact<{
  categoryId: Scalars['DbUuid']['input'];
  first: Scalars['Int']['input'];
  offset: Scalars['Int']['input'];
}>;


export type GetExpensesByCategoryQuery = { __typename?: 'Query', expensesByCategory: Array<{ __typename?: 'Expense', id: string, title: string, amount: string, expenseDate: string, categoryId: string, costCenterId: string, description?: string | null, createdAt: string, updatedAt: string, category: { __typename?: 'PurchaseCategory', id: string, name: string }, costCenter: { __typename?: 'CostCenter', id: string, name: string, code: string } }> };

export type CreateExpenseMutationVariables = Exact<{
  input: ExpenseNewInput;
}>;


export type CreateExpenseMutation = { __typename?: 'Mutation', createExpense: { __typename?: 'Expense', id: string, title: string, amount: string, expenseDate: string, categoryId: string, costCenterId: string, description?: string | null, createdAt: string, updatedAt: string, category: { __typename?: 'PurchaseCategory', id: string, name: string }, costCenter: { __typename?: 'CostCenter', id: string, name: string, code: string } } };

export type UpdateExpenseMutationVariables = Exact<{
  input: ExpenseUpdateInput;
}>;


export type UpdateExpenseMutation = { __typename?: 'Mutation', updateExpense: { __typename?: 'Expense', id: string, title: string, amount: string, expenseDate: string, categoryId: string, costCenterId: string, description?: string | null, createdAt: string, updatedAt: string, category: { __typename?: 'PurchaseCategory', id: string, name: string }, costCenter: { __typename?: 'CostCenter', id: string, name: string, code: string } } };

export type DeleteExpenseMutationVariables = Exact<{
  id: Scalars['DbUuid']['input'];
}>;


export type DeleteExpenseMutation = { __typename?: 'Mutation', deleteExpense: number };

export type GetSalesChargeTypesQueryVariables = Exact<{
  first: Scalars['Int']['input'];
  offset: Scalars['Int']['input'];
}>;


export type GetSalesChargeTypesQuery = { __typename?: 'Query', salesChargeTypesCount: number, salesChargeTypes: Array<{ __typename?: 'SalesChargeType', id: string, name: string, description?: string | null, createdAt: string, updatedAt: string }> };

export type GetSalesChargeTypeQueryVariables = Exact<{
  id: Scalars['DbUuid']['input'];
}>;


export type GetSalesChargeTypeQuery = { __typename?: 'Query', salesChargeType: { __typename?: 'SalesChargeType', id: string, name: string, description?: string | null, createdAt: string, updatedAt: string } };

export type CreateSalesChargeTypeMutationVariables = Exact<{
  input: SalesChargeTypeNewInput;
}>;


export type CreateSalesChargeTypeMutation = { __typename?: 'Mutation', createSalesChargeType: { __typename?: 'SalesChargeType', id: string, name: string, description?: string | null, createdAt: string, updatedAt: string } };

export type UpdateSalesChargeTypeMutationVariables = Exact<{
  input: SalesChargeTypeUpdateInput;
}>;


export type UpdateSalesChargeTypeMutation = { __typename?: 'Mutation', updateSalesChargeType: { __typename?: 'SalesChargeType', id: string, name: string, description?: string | null, createdAt: string, updatedAt: string } };

export type DeleteSalesChargeTypeMutationVariables = Exact<{
  id: Scalars['DbUuid']['input'];
}>;


export type DeleteSalesChargeTypeMutation = { __typename?: 'Mutation', deleteSalesChargeType: boolean };

export type GetSalesOrdersQueryVariables = Exact<{
  first: Scalars['Int']['input'];
  offset: Scalars['Int']['input'];
}>;


export type GetSalesOrdersQuery = { __typename?: 'Query', totalSalesOrders: number, salesOrders: Array<{ __typename?: 'SalesOrder', id: string, customerId?: string | null, customerName?: string | null, customerPhoneNumber?: string | null, orderDate: string, netAmount: string, discAmount: string, taxableAmount: string, taxAmount: string, totalAmount: string, orderState: SalesOrderState, costCenterId: string, createdAt: string, updatedAt: string, totalPaidAmount: string, customer?: { __typename?: 'Customer', id: string, fullName: string, phone?: string | null, createdAt: string, updatedAt: string } | null, items: Array<{ __typename?: 'SalesOrderItem', id: string, orderId: string, itemId?: string | null, itemName: string, quantity: number, priceAmount: string, taxAmount: string, totalAmount: string, createdAt: string, updatedAt: string }>, payments: Array<{ __typename?: 'SalesOrderPayment', id: string, orderId: string, paymentMethodId: string, paymentDate: string, amount: string, referenceNumber?: string | null, notes?: string | null, state: SalesOrderPaymentState }> }> };

export type GetSalesOrderQueryVariables = Exact<{
  id: Scalars['DbUuid']['input'];
}>;


export type GetSalesOrderQuery = { __typename?: 'Query', salesOrder: { __typename?: 'SalesOrder', id: string, customerId?: string | null, customerName?: string | null, customerPhoneNumber?: string | null, orderDate: string, netAmount: string, discAmount: string, taxableAmount: string, taxAmount: string, totalAmount: string, orderState: SalesOrderState, costCenterId: string, createdAt: string, updatedAt: string, totalPaidAmount: string, items: Array<{ __typename?: 'SalesOrderItem', id: string, orderId: string, itemId?: string | null, itemName: string, quantity: number, priceAmount: string, taxAmount: string, totalAmount: string, createdAt: string, updatedAt: string }>, payments: Array<{ __typename?: 'SalesOrderPayment', id: string, orderId: string, paymentMethodId: string, paymentDate: string, amount: string, referenceNumber?: string | null, notes?: string | null, state: SalesOrderPaymentState }> } };

export type GetSalesOrdersCustomersQueryVariables = Exact<{ [key: string]: never; }>;


export type GetSalesOrdersCustomersQuery = { __typename?: 'Query', customers: Array<{ __typename?: 'Customer', id: string, fullName: string, phone?: string | null, email?: string | null }> };

export type GetSalesOrdersPaymentMethodsQueryVariables = Exact<{ [key: string]: never; }>;


export type GetSalesOrdersPaymentMethodsQuery = { __typename?: 'Query', paymentMethods: Array<{ __typename?: 'PaymentMethod', id: string, name: string, code: string }> };

export type GetSalesOrdersCostCentersQueryVariables = Exact<{ [key: string]: never; }>;


export type GetSalesOrdersCostCentersQuery = { __typename?: 'Query', costCenters: Array<{ __typename?: 'CostCenter', id: string, name: string, code: string }> };

export type SalesOrdersCreateSalesOrderMutationVariables = Exact<{
  order: SalesOrderNewInput;
}>;


export type SalesOrdersCreateSalesOrderMutation = { __typename?: 'Mutation', createSalesOrder: { __typename?: 'SalesOrder', id: string, customerName?: string | null, totalAmount: string, orderState: SalesOrderState } };

export type CreateSalesOrderPaymentMutationVariables = Exact<{
  payment: SalesOrderPaymentNewInput;
}>;


export type CreateSalesOrderPaymentMutation = { __typename?: 'Mutation', createSalesOrderPayment: { __typename?: 'SalesOrderPayment', id: string, orderId: string, paymentMethodId: string, paymentDate: string, amount: string, referenceNumber?: string | null, notes?: string | null, state: SalesOrderPaymentState } };

export type VoidSalesOrderMutationVariables = Exact<{
  id: Scalars['DbUuid']['input'];
}>;


export type VoidSalesOrderMutation = { __typename?: 'Mutation', voidSalesOrder: { __typename?: 'SalesOrder', id: string, orderState: SalesOrderState } };

export type VoidSalesOrderPaymentMutationVariables = Exact<{
  id: Scalars['DbUuid']['input'];
}>;


export type VoidSalesOrderPaymentMutation = { __typename?: 'Mutation', voidSalesOrderPayment: { __typename?: 'SalesOrderPayment', id: string, state: SalesOrderPaymentState } };

export type GetBrandsQueryVariables = Exact<{ [key: string]: never; }>;


export type GetBrandsQuery = { __typename?: 'Query', brands: Array<{ __typename?: 'Brand', id: string, name: string, description?: string | null, isActive: boolean, createdAt: string, updatedAt: string }> };

export type GetBrandQueryVariables = Exact<{
  id: Scalars['DbUuid']['input'];
}>;


export type GetBrandQuery = { __typename?: 'Query', brand: { __typename?: 'Brand', id: string, name: string, description?: string | null, isActive: boolean, createdAt: string, updatedAt: string } };

export type GetActiveBrandsQueryVariables = Exact<{ [key: string]: never; }>;


export type GetActiveBrandsQuery = { __typename?: 'Query', activeBrands: Array<{ __typename?: 'Brand', id: string, name: string, description?: string | null, isActive: boolean, createdAt: string, updatedAt: string }> };

export type CreateBrandMutationVariables = Exact<{
  input: BrandNewInput;
}>;


export type CreateBrandMutation = { __typename?: 'Mutation', createBrand: { __typename?: 'Brand', id: string, name: string, description?: string | null, isActive: boolean, createdAt: string, updatedAt: string } };

export type UpdateBrandMutationVariables = Exact<{
  input: BrandUpdateInput;
}>;


export type UpdateBrandMutation = { __typename?: 'Mutation', updateBrand: { __typename?: 'Brand', id: string, name: string, description?: string | null, isActive: boolean, createdAt: string, updatedAt: string } };

export type DeleteBrandMutationVariables = Exact<{
  id: Scalars['DbUuid']['input'];
}>;


export type DeleteBrandMutation = { __typename?: 'Mutation', deleteBrand: number };

export type GetChannelsQueryVariables = Exact<{ [key: string]: never; }>;


export type GetChannelsQuery = { __typename?: 'Query', channels: Array<{ __typename?: 'Channel', id: string, name: string, description?: string | null, isActive: boolean, createdAt: string, updatedAt: string }> };

export type GetChannelQueryVariables = Exact<{
  id: Scalars['DbUuid']['input'];
}>;


export type GetChannelQuery = { __typename?: 'Query', channel: { __typename?: 'Channel', id: string, name: string, description?: string | null, isActive: boolean, createdAt: string, updatedAt: string } };

export type GetActiveChannelsQueryVariables = Exact<{ [key: string]: never; }>;


export type GetActiveChannelsQuery = { __typename?: 'Query', activeChannels: Array<{ __typename?: 'Channel', id: string, name: string, description?: string | null, isActive: boolean, createdAt: string, updatedAt: string }> };

export type CreateChannelMutationVariables = Exact<{
  input: ChannelNewInput;
}>;


export type CreateChannelMutation = { __typename?: 'Mutation', createChannel: { __typename?: 'Channel', id: string, name: string, description?: string | null, isActive: boolean, createdAt: string, updatedAt: string } };

export type UpdateChannelMutationVariables = Exact<{
  input: ChannelUpdateInput;
}>;


export type UpdateChannelMutation = { __typename?: 'Mutation', updateChannel: { __typename?: 'Channel', id: string, name: string, description?: string | null, isActive: boolean, createdAt: string, updatedAt: string } };

export type DeleteChannelMutationVariables = Exact<{
  id: Scalars['DbUuid']['input'];
}>;


export type DeleteChannelMutation = { __typename?: 'Mutation', deleteChannel: number };

export type GetCostCentersQueryVariables = Exact<{
  first?: InputMaybe<Scalars['Int']['input']>;
  offset?: InputMaybe<Scalars['Int']['input']>;
}>;


export type GetCostCentersQuery = { __typename?: 'Query', totalCostCenters: number, costCenters: Array<{ __typename?: 'CostCenter', id: string, name: string, code: string, description?: string | null, state: CostCenterState, createdAt: string, updatedAt: string }> };

export type GetCostCenterQueryVariables = Exact<{
  id: Scalars['DbUuid']['input'];
}>;


export type GetCostCenterQuery = { __typename?: 'Query', costCenter: { __typename?: 'CostCenter', id: string, name: string, code: string, description?: string | null, state: CostCenterState, createdAt: string, updatedAt: string } };

export type GetAllCostCentersQueryVariables = Exact<{ [key: string]: never; }>;


export type GetAllCostCentersQuery = { __typename?: 'Query', allCostCenters: Array<{ __typename?: 'CostCenter', id: string, name: string, code: string, description?: string | null, state: CostCenterState }> };

export type CreateCostCenterMutationVariables = Exact<{
  name: Scalars['String']['input'];
  code: Scalars['String']['input'];
  description?: InputMaybe<Scalars['String']['input']>;
  state?: InputMaybe<CostCenterState>;
}>;


export type CreateCostCenterMutation = { __typename?: 'Mutation', createCostCenter: { __typename?: 'CostCenter', id: string, name: string, code: string, description?: string | null, state: CostCenterState, createdAt: string, updatedAt: string } };

export type UpdateCostCenterMutationVariables = Exact<{
  id: Scalars['DbUuid']['input'];
  name?: InputMaybe<Scalars['String']['input']>;
  code?: InputMaybe<Scalars['String']['input']>;
  description?: InputMaybe<Scalars['String']['input']>;
  state?: InputMaybe<CostCenterState>;
}>;


export type UpdateCostCenterMutation = { __typename?: 'Mutation', updateCostCenter: { __typename?: 'CostCenter', id: string, name: string, code: string, description?: string | null, state: CostCenterState, createdAt: string, updatedAt: string } };

export type DeleteCostCenterMutationVariables = Exact<{
  id: Scalars['DbUuid']['input'];
}>;


export type DeleteCostCenterMutation = { __typename?: 'Mutation', deleteCostCenter: string };

export type GetPaymentMethodsQueryVariables = Exact<{
  first: Scalars['Int']['input'];
  offset: Scalars['Int']['input'];
}>;


export type GetPaymentMethodsQuery = { __typename?: 'Query', paymentMethods: Array<{ __typename?: 'PaymentMethod', id: string, name: string, code: string, description?: string | null, state: PaymentMethodState, createdAt: string, updatedAt: string }> };

export type GetTotalPaymentMethodsQueryVariables = Exact<{ [key: string]: never; }>;


export type GetTotalPaymentMethodsQuery = { __typename?: 'Query', totalPaymentMethods: number };

export type GetPaymentMethodQueryVariables = Exact<{
  id: Scalars['DbUuid']['input'];
}>;


export type GetPaymentMethodQuery = { __typename?: 'Query', paymentMethod: { __typename?: 'PaymentMethod', id: string, name: string, code: string, description?: string | null, state: PaymentMethodState, createdAt: string, updatedAt: string } };

export type CreatePaymentMethodMutationVariables = Exact<{
  name: Scalars['String']['input'];
  code: Scalars['String']['input'];
  description?: InputMaybe<Scalars['String']['input']>;
  state?: InputMaybe<PaymentMethodState>;
}>;


export type CreatePaymentMethodMutation = { __typename?: 'Mutation', createPaymentMethod: { __typename?: 'PaymentMethod', id: string, name: string, code: string, description?: string | null, state: PaymentMethodState, createdAt: string, updatedAt: string } };

export type UpdatePaymentMethodMutationVariables = Exact<{
  id: Scalars['DbUuid']['input'];
  name?: InputMaybe<Scalars['String']['input']>;
  code?: InputMaybe<Scalars['String']['input']>;
  description?: InputMaybe<Scalars['String']['input']>;
  state?: InputMaybe<PaymentMethodState>;
}>;


export type UpdatePaymentMethodMutation = { __typename?: 'Mutation', updatePaymentMethod: { __typename?: 'PaymentMethod', id: string, name: string, code: string, description?: string | null, state: PaymentMethodState, createdAt: string, updatedAt: string } };

export type DeletePaymentMethodMutationVariables = Exact<{
  id: Scalars['DbUuid']['input'];
}>;


export type DeletePaymentMethodMutation = { __typename?: 'Mutation', deletePaymentMethod: string };

export type GetTaxGroupsQueryVariables = Exact<{
  first: Scalars['Int']['input'];
  offset: Scalars['Int']['input'];
}>;


export type GetTaxGroupsQuery = { __typename?: 'Query', totalTaxGroups: number, taxGroups: Array<{ __typename?: 'TaxGroup', id: string, name: string, description?: string | null, createdAt: string, updatedAt: string, taxes: Array<{ __typename?: 'Tax', id: string, name: string, rate: string }> }> };

export type GetTaxGroupQueryVariables = Exact<{
  id: Scalars['DbUuid']['input'];
}>;


export type GetTaxGroupQuery = { __typename?: 'Query', taxGroup: { __typename?: 'TaxGroup', id: string, name: string, description?: string | null, createdAt: string, updatedAt: string, taxes: Array<{ __typename?: 'Tax', id: string, name: string, rate: string }> } };

export type GetAllTaxesQueryVariables = Exact<{ [key: string]: never; }>;


export type GetAllTaxesQuery = { __typename?: 'Query', taxes: Array<{ __typename?: 'Tax', id: string, name: string, rate: string }> };

export type CreateTaxGroupMutationVariables = Exact<{
  input: TaxGroupNewInput;
}>;


export type CreateTaxGroupMutation = { __typename?: 'Mutation', createTaxGroup: { __typename?: 'TaxGroup', id: string, name: string, description?: string | null, createdAt: string, updatedAt: string, taxes: Array<{ __typename?: 'Tax', id: string, name: string, rate: string }> } };

export type UpdateTaxGroupMutationVariables = Exact<{
  input: TaxGroupUpdateInput;
}>;


export type UpdateTaxGroupMutation = { __typename?: 'Mutation', updateTaxGroup: { __typename?: 'TaxGroup', id: string, name: string, description?: string | null, createdAt: string, updatedAt: string, taxes: Array<{ __typename?: 'Tax', id: string, name: string, rate: string }> } };

export type DeleteTaxGroupMutationVariables = Exact<{
  id: Scalars['DbUuid']['input'];
}>;


export type DeleteTaxGroupMutation = { __typename?: 'Mutation', deleteTaxGroup: number };

export type AssignTaxToGroupMutationVariables = Exact<{
  taxGroupId: Scalars['DbUuid']['input'];
  taxId: Scalars['DbUuid']['input'];
}>;


export type AssignTaxToGroupMutation = { __typename?: 'Mutation', assignTaxToGroup: number };

export type RemoveTaxFromGroupMutationVariables = Exact<{
  taxGroupId: Scalars['DbUuid']['input'];
  taxId: Scalars['DbUuid']['input'];
}>;


export type RemoveTaxFromGroupMutation = { __typename?: 'Mutation', removeTaxFromGroup: number };

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
export const GetDiscountsDocument = new TypedDocumentString(`
    query getDiscounts($first: Int, $offset: Int, $state: DiscountState) {
  discounts(first: $first, offset: $offset, state: $state) {
    id
    name
    description
    discountType
    value
    scope
    state
    startDate
    endDate
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<GetDiscountsQuery, GetDiscountsQueryVariables>;
export const GetDiscountDocument = new TypedDocumentString(`
    query getDiscount($id: DbUuid!) {
  discount(id: $id) {
    id
    name
    description
    discountType
    value
    scope
    state
    startDate
    endDate
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<GetDiscountQuery, GetDiscountQueryVariables>;
export const CreateDiscountDocument = new TypedDocumentString(`
    mutation createDiscount($discount: DiscountNewInput!) {
  createDiscount(discount: $discount) {
    id
    name
    description
    discountType
    value
    scope
    state
    startDate
    endDate
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<CreateDiscountMutation, CreateDiscountMutationVariables>;
export const UpdateDiscountDocument = new TypedDocumentString(`
    mutation updateDiscount($discount: DiscountUpdateInput!) {
  updateDiscount(discount: $discount) {
    id
    name
    description
    discountType
    value
    scope
    state
    startDate
    endDate
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<UpdateDiscountMutation, UpdateDiscountMutationVariables>;
export const DeleteDiscountDocument = new TypedDocumentString(`
    mutation deleteDiscount($id: DbUuid!) {
  deleteDiscount(id: $id)
}
    `) as unknown as TypedDocumentString<DeleteDiscountMutation, DeleteDiscountMutationVariables>;
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
    hasVariants
    variants {
      id
      sku
      priceAdjustment
      isDefault
      finalPrice
      variantValues {
        id
        value
        variantType {
          id
          name
        }
      }
    }
  }
}
    `) as unknown as TypedDocumentString<GetItemsQuery, GetItemsQueryVariables>;
export const GetItemDocument = new TypedDocumentString(`
    query getItem($id: DbUuid!) {
  item(id: $id) {
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
    hasVariants
    variants {
      id
      sku
      priceAdjustment
      isDefault
      finalPrice
      variantValues {
        id
        value
        variantType {
          id
          name
        }
      }
    }
  }
}
    `) as unknown as TypedDocumentString<GetItemQuery, GetItemQueryVariables>;
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
    hasVariants
    variants {
      id
      sku
      priceAdjustment
      isDefault
      finalPrice
      variantValues {
        id
        value
        variantType {
          id
          name
        }
      }
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
    hasVariants
    variants {
      id
      sku
      priceAdjustment
      isDefault
      finalPrice
      variantValues {
        id
        value
        variantType {
          id
          name
        }
      }
    }
  }
}
    `) as unknown as TypedDocumentString<UpdateItemMutation, UpdateItemMutationVariables>;
export const DeleteItemDocument = new TypedDocumentString(`
    mutation deleteItem($id: DbUuid!) {
  deleteItem(id: $id)
}
    `) as unknown as TypedDocumentString<DeleteItemMutation, DeleteItemMutationVariables>;
export const GetItemDiscountsDocument = new TypedDocumentString(`
    query getItemDiscounts($itemId: DbUuid!) {
  itemDiscounts(itemId: $itemId) {
    itemId
    discountId
  }
}
    `) as unknown as TypedDocumentString<GetItemDiscountsQuery, GetItemDiscountsQueryVariables>;
export const GetDiscountItemsDocument = new TypedDocumentString(`
    query getDiscountItems($discountId: DbUuid!) {
  discountItems(discountId: $discountId) {
    itemId
    discountId
  }
}
    `) as unknown as TypedDocumentString<GetDiscountItemsQuery, GetDiscountItemsQueryVariables>;
export const AddItemDiscountDocument = new TypedDocumentString(`
    mutation addItemDiscount($itemDiscount: ItemDiscountNewInput!) {
  addItemDiscount(itemDiscount: $itemDiscount) {
    itemId
    discountId
  }
}
    `) as unknown as TypedDocumentString<AddItemDiscountMutation, AddItemDiscountMutationVariables>;
export const RemoveItemDiscountDocument = new TypedDocumentString(`
    mutation removeItemDiscount($itemId: DbUuid!, $discountId: DbUuid!) {
  removeItemDiscount(itemId: $itemId, discountId: $discountId)
}
    `) as unknown as TypedDocumentString<RemoveItemDiscountMutation, RemoveItemDiscountMutationVariables>;
export const GetVariantTypesDocument = new TypedDocumentString(`
    query getVariantTypes($first: Int!, $offset: Int!) {
  variantTypes(first: $first, offset: $offset) {
    id
    name
    description
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<GetVariantTypesQuery, GetVariantTypesQueryVariables>;
export const GetVariantTypeDocument = new TypedDocumentString(`
    query getVariantType($id: DbUuid!) {
  variantType(id: $id) {
    id
    name
    description
    createdAt
    updatedAt
    values {
      id
      value
      displayOrder
      createdAt
      updatedAt
    }
  }
}
    `) as unknown as TypedDocumentString<GetVariantTypeQuery, GetVariantTypeQueryVariables>;
export const GetTotalVariantTypesDocument = new TypedDocumentString(`
    query getTotalVariantTypes {
  totalVariantTypes
}
    `) as unknown as TypedDocumentString<GetTotalVariantTypesQuery, GetTotalVariantTypesQueryVariables>;
export const GetVariantValuesDocument = new TypedDocumentString(`
    query getVariantValues($variantTypeId: DbUuid, $first: Int!, $offset: Int!) {
  variantValues(variantTypeId: $variantTypeId, first: $first, offset: $offset) {
    id
    value
    displayOrder
    createdAt
    updatedAt
    variantType {
      id
      name
      description
    }
  }
}
    `) as unknown as TypedDocumentString<GetVariantValuesQuery, GetVariantValuesQueryVariables>;
export const GetVariantValueDocument = new TypedDocumentString(`
    query getVariantValue($id: DbUuid!) {
  variantValue(id: $id) {
    id
    value
    displayOrder
    createdAt
    updatedAt
    variantType {
      id
      name
      description
    }
  }
}
    `) as unknown as TypedDocumentString<GetVariantValueQuery, GetVariantValueQueryVariables>;
export const GetItemVariantsDocument = new TypedDocumentString(`
    query getItemVariants($itemId: DbUuid!, $first: Int!, $offset: Int!) {
  itemVariants(itemId: $itemId, first: $first, offset: $offset) {
    id
    sku
    priceAdjustment
    isDefault
    createdAt
    updatedAt
    finalPrice
    variantValues {
      id
      value
      displayOrder
      variantType {
        id
        name
      }
    }
  }
}
    `) as unknown as TypedDocumentString<GetItemVariantsQuery, GetItemVariantsQueryVariables>;
export const GetItemVariantDocument = new TypedDocumentString(`
    query getItemVariant($id: DbUuid!) {
  itemVariant(id: $id) {
    id
    sku
    priceAdjustment
    isDefault
    createdAt
    updatedAt
    finalPrice
    item {
      id
      name
      price
    }
    variantValues {
      id
      value
      displayOrder
      variantType {
        id
        name
      }
    }
  }
}
    `) as unknown as TypedDocumentString<GetItemVariantQuery, GetItemVariantQueryVariables>;
export const CreateVariantTypeDocument = new TypedDocumentString(`
    mutation createVariantType($input: VariantTypeNewInput!) {
  createVariantType(input: $input) {
    id
    name
    description
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<CreateVariantTypeMutation, CreateVariantTypeMutationVariables>;
export const UpdateVariantTypeDocument = new TypedDocumentString(`
    mutation updateVariantType($input: VariantTypeUpdateInput!) {
  updateVariantType(input: $input) {
    id
    name
    description
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<UpdateVariantTypeMutation, UpdateVariantTypeMutationVariables>;
export const DeleteVariantTypeDocument = new TypedDocumentString(`
    mutation deleteVariantType($id: DbUuid!) {
  deleteVariantType(id: $id)
}
    `) as unknown as TypedDocumentString<DeleteVariantTypeMutation, DeleteVariantTypeMutationVariables>;
export const CreateVariantValueDocument = new TypedDocumentString(`
    mutation createVariantValue($input: VariantValueNewInput!) {
  createVariantValue(input: $input) {
    id
    value
    displayOrder
    createdAt
    updatedAt
    variantType {
      id
      name
    }
  }
}
    `) as unknown as TypedDocumentString<CreateVariantValueMutation, CreateVariantValueMutationVariables>;
export const UpdateVariantValueDocument = new TypedDocumentString(`
    mutation updateVariantValue($input: VariantValueUpdateInput!) {
  updateVariantValue(input: $input) {
    id
    value
    displayOrder
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<UpdateVariantValueMutation, UpdateVariantValueMutationVariables>;
export const DeleteVariantValueDocument = new TypedDocumentString(`
    mutation deleteVariantValue($id: DbUuid!) {
  deleteVariantValue(id: $id)
}
    `) as unknown as TypedDocumentString<DeleteVariantValueMutation, DeleteVariantValueMutationVariables>;
export const CreateItemVariantDocument = new TypedDocumentString(`
    mutation createItemVariant($input: ItemVariantNewInput!) {
  createItemVariant(input: $input) {
    id
    sku
    priceAdjustment
    isDefault
    createdAt
    updatedAt
    finalPrice
  }
}
    `) as unknown as TypedDocumentString<CreateItemVariantMutation, CreateItemVariantMutationVariables>;
export const UpdateItemVariantDocument = new TypedDocumentString(`
    mutation updateItemVariant($input: ItemVariantUpdateInput!) {
  updateItemVariant(input: $input) {
    id
    sku
    priceAdjustment
    isDefault
    createdAt
    updatedAt
    finalPrice
  }
}
    `) as unknown as TypedDocumentString<UpdateItemVariantMutation, UpdateItemVariantMutationVariables>;
export const DeleteItemVariantDocument = new TypedDocumentString(`
    mutation deleteItemVariant($id: DbUuid!) {
  deleteItemVariant(id: $id)
}
    `) as unknown as TypedDocumentString<DeleteItemVariantMutation, DeleteItemVariantMutationVariables>;
export const AssignVariantValueToItemVariantDocument = new TypedDocumentString(`
    mutation assignVariantValueToItemVariant($itemVariantId: DbUuid!, $variantValueId: DbUuid!) {
  assignVariantValueToItemVariant(
    itemVariantId: $itemVariantId
    variantValueId: $variantValueId
  )
}
    `) as unknown as TypedDocumentString<AssignVariantValueToItemVariantMutation, AssignVariantValueToItemVariantMutationVariables>;
export const RemoveVariantValueFromItemVariantDocument = new TypedDocumentString(`
    mutation removeVariantValueFromItemVariant($itemVariantId: DbUuid!, $variantValueId: DbUuid!) {
  removeVariantValueFromItemVariant(
    itemVariantId: $itemVariantId
    variantValueId: $variantValueId
  )
}
    `) as unknown as TypedDocumentString<RemoveVariantValueFromItemVariantMutation, RemoveVariantValueFromItemVariantMutationVariables>;
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
    hasVariants
    variants {
      id
      sku
      priceAdjustment
      isDefault
      finalPrice
      variantValues {
        id
        value
        variantType {
          id
          name
        }
      }
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
export const PosCreateSalesOrderDocument = new TypedDocumentString(`
    mutation PosCreateSalesOrder($salesOrder: SalesOrderNewInput!) {
  createSalesOrder(salesOrder: $salesOrder) {
    id
    customerName
    orderDate
    netAmount
    taxAmount
    totalAmount
    orderState
  }
}
    `) as unknown as TypedDocumentString<PosCreateSalesOrderMutation, PosCreateSalesOrderMutationVariables>;
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
    mutation createPurchaseCategory($name: String!, $description: String, $state: PurchaseCategoryState) {
  createPurchaseCategory(name: $name, description: $description, state: $state) {
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
    mutation updatePurchaseCategory($id: DbUuid!, $name: String, $description: String, $state: PurchaseCategoryState) {
  updatePurchaseCategory(
    id: $id
    name: $name
    description: $description
    state: $state
  ) {
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
    query GetExpenses($first: Int!, $offset: Int!, $costCenterId: DbUuid, $startDate: String, $endDate: String) {
  expenses(
    first: $first
    offset: $offset
    costCenterId: $costCenterId
    startDate: $startDate
    endDate: $endDate
  ) {
    id
    title
    amount
    expenseDate
    categoryId
    costCenterId
    description
    createdAt
    updatedAt
    category {
      id
      name
    }
    costCenter {
      id
      name
      code
    }
  }
  totalExpenses(
    costCenterId: $costCenterId
    startDate: $startDate
    endDate: $endDate
  )
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
export const GetCostCentersForExpensesDocument = new TypedDocumentString(`
    query GetCostCentersForExpenses {
  allCostCenters {
    id
    name
    code
    state
  }
}
    `) as unknown as TypedDocumentString<GetCostCentersForExpensesQuery, GetCostCentersForExpensesQueryVariables>;
export const GetExpensesByCategoryDocument = new TypedDocumentString(`
    query GetExpensesByCategory($categoryId: DbUuid!, $first: Int!, $offset: Int!) {
  expensesByCategory(categoryId: $categoryId, first: $first, offset: $offset) {
    id
    title
    amount
    expenseDate
    categoryId
    costCenterId
    description
    createdAt
    updatedAt
    category {
      id
      name
    }
    costCenter {
      id
      name
      code
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
    costCenterId
    description
    createdAt
    updatedAt
    category {
      id
      name
    }
    costCenter {
      id
      name
      code
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
    costCenterId
    description
    createdAt
    updatedAt
    category {
      id
      name
    }
    costCenter {
      id
      name
      code
    }
  }
}
    `) as unknown as TypedDocumentString<UpdateExpenseMutation, UpdateExpenseMutationVariables>;
export const DeleteExpenseDocument = new TypedDocumentString(`
    mutation DeleteExpense($id: DbUuid!) {
  deleteExpense(id: $id)
}
    `) as unknown as TypedDocumentString<DeleteExpenseMutation, DeleteExpenseMutationVariables>;
export const GetSalesChargeTypesDocument = new TypedDocumentString(`
    query GetSalesChargeTypes($first: Int!, $offset: Int!) {
  salesChargeTypes(first: $first, offset: $offset) {
    id
    name
    description
    createdAt
    updatedAt
  }
  salesChargeTypesCount
}
    `) as unknown as TypedDocumentString<GetSalesChargeTypesQuery, GetSalesChargeTypesQueryVariables>;
export const GetSalesChargeTypeDocument = new TypedDocumentString(`
    query GetSalesChargeType($id: DbUuid!) {
  salesChargeType(id: $id) {
    id
    name
    description
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<GetSalesChargeTypeQuery, GetSalesChargeTypeQueryVariables>;
export const CreateSalesChargeTypeDocument = new TypedDocumentString(`
    mutation CreateSalesChargeType($input: SalesChargeTypeNewInput!) {
  createSalesChargeType(chargeType: $input) {
    id
    name
    description
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<CreateSalesChargeTypeMutation, CreateSalesChargeTypeMutationVariables>;
export const UpdateSalesChargeTypeDocument = new TypedDocumentString(`
    mutation UpdateSalesChargeType($input: SalesChargeTypeUpdateInput!) {
  updateSalesChargeType(chargeType: $input) {
    id
    name
    description
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<UpdateSalesChargeTypeMutation, UpdateSalesChargeTypeMutationVariables>;
export const DeleteSalesChargeTypeDocument = new TypedDocumentString(`
    mutation DeleteSalesChargeType($id: DbUuid!) {
  deleteSalesChargeType(id: $id)
}
    `) as unknown as TypedDocumentString<DeleteSalesChargeTypeMutation, DeleteSalesChargeTypeMutationVariables>;
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
    orderState
    costCenterId
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
    payments {
      id
      orderId
      paymentMethodId
      paymentDate
      amount
      referenceNumber
      notes
      state
    }
    totalPaidAmount
  }
  totalSalesOrders
}
    `) as unknown as TypedDocumentString<GetSalesOrdersQuery, GetSalesOrdersQueryVariables>;
export const GetSalesOrderDocument = new TypedDocumentString(`
    query GetSalesOrder($id: DbUuid!) {
  salesOrder(id: $id) {
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
    orderState
    costCenterId
    createdAt
    updatedAt
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
    payments {
      id
      orderId
      paymentMethodId
      paymentDate
      amount
      referenceNumber
      notes
      state
    }
    totalPaidAmount
  }
}
    `) as unknown as TypedDocumentString<GetSalesOrderQuery, GetSalesOrderQueryVariables>;
export const GetSalesOrdersCustomersDocument = new TypedDocumentString(`
    query GetSalesOrdersCustomers {
  customers {
    id
    fullName
    phone
    email
  }
}
    `) as unknown as TypedDocumentString<GetSalesOrdersCustomersQuery, GetSalesOrdersCustomersQueryVariables>;
export const GetSalesOrdersPaymentMethodsDocument = new TypedDocumentString(`
    query GetSalesOrdersPaymentMethods {
  paymentMethods {
    id
    name
    code
  }
}
    `) as unknown as TypedDocumentString<GetSalesOrdersPaymentMethodsQuery, GetSalesOrdersPaymentMethodsQueryVariables>;
export const GetSalesOrdersCostCentersDocument = new TypedDocumentString(`
    query GetSalesOrdersCostCenters {
  costCenters {
    id
    name
    code
  }
}
    `) as unknown as TypedDocumentString<GetSalesOrdersCostCentersQuery, GetSalesOrdersCostCentersQueryVariables>;
export const SalesOrdersCreateSalesOrderDocument = new TypedDocumentString(`
    mutation SalesOrdersCreateSalesOrder($order: SalesOrderNewInput!) {
  createSalesOrder(salesOrder: $order) {
    id
    customerName
    totalAmount
    orderState
  }
}
    `) as unknown as TypedDocumentString<SalesOrdersCreateSalesOrderMutation, SalesOrdersCreateSalesOrderMutationVariables>;
export const CreateSalesOrderPaymentDocument = new TypedDocumentString(`
    mutation CreateSalesOrderPayment($payment: SalesOrderPaymentNewInput!) {
  createSalesOrderPayment(payment: $payment) {
    id
    orderId
    paymentMethodId
    paymentDate
    amount
    referenceNumber
    notes
    state
  }
}
    `) as unknown as TypedDocumentString<CreateSalesOrderPaymentMutation, CreateSalesOrderPaymentMutationVariables>;
export const VoidSalesOrderDocument = new TypedDocumentString(`
    mutation VoidSalesOrder($id: DbUuid!) {
  voidSalesOrder(id: $id) {
    id
    orderState
  }
}
    `) as unknown as TypedDocumentString<VoidSalesOrderMutation, VoidSalesOrderMutationVariables>;
export const VoidSalesOrderPaymentDocument = new TypedDocumentString(`
    mutation VoidSalesOrderPayment($id: DbUuid!) {
  voidSalesOrderPayment(id: $id) {
    id
    state
  }
}
    `) as unknown as TypedDocumentString<VoidSalesOrderPaymentMutation, VoidSalesOrderPaymentMutationVariables>;
export const GetBrandsDocument = new TypedDocumentString(`
    query GetBrands {
  brands {
    id
    name
    description
    isActive
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<GetBrandsQuery, GetBrandsQueryVariables>;
export const GetBrandDocument = new TypedDocumentString(`
    query GetBrand($id: DbUuid!) {
  brand(id: $id) {
    id
    name
    description
    isActive
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<GetBrandQuery, GetBrandQueryVariables>;
export const GetActiveBrandsDocument = new TypedDocumentString(`
    query GetActiveBrands {
  activeBrands {
    id
    name
    description
    isActive
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<GetActiveBrandsQuery, GetActiveBrandsQueryVariables>;
export const CreateBrandDocument = new TypedDocumentString(`
    mutation CreateBrand($input: BrandNewInput!) {
  createBrand(input: $input) {
    id
    name
    description
    isActive
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<CreateBrandMutation, CreateBrandMutationVariables>;
export const UpdateBrandDocument = new TypedDocumentString(`
    mutation UpdateBrand($input: BrandUpdateInput!) {
  updateBrand(input: $input) {
    id
    name
    description
    isActive
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<UpdateBrandMutation, UpdateBrandMutationVariables>;
export const DeleteBrandDocument = new TypedDocumentString(`
    mutation DeleteBrand($id: DbUuid!) {
  deleteBrand(id: $id)
}
    `) as unknown as TypedDocumentString<DeleteBrandMutation, DeleteBrandMutationVariables>;
export const GetChannelsDocument = new TypedDocumentString(`
    query GetChannels {
  channels {
    id
    name
    description
    isActive
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<GetChannelsQuery, GetChannelsQueryVariables>;
export const GetChannelDocument = new TypedDocumentString(`
    query GetChannel($id: DbUuid!) {
  channel(id: $id) {
    id
    name
    description
    isActive
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<GetChannelQuery, GetChannelQueryVariables>;
export const GetActiveChannelsDocument = new TypedDocumentString(`
    query GetActiveChannels {
  activeChannels {
    id
    name
    description
    isActive
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<GetActiveChannelsQuery, GetActiveChannelsQueryVariables>;
export const CreateChannelDocument = new TypedDocumentString(`
    mutation CreateChannel($input: ChannelNewInput!) {
  createChannel(input: $input) {
    id
    name
    description
    isActive
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<CreateChannelMutation, CreateChannelMutationVariables>;
export const UpdateChannelDocument = new TypedDocumentString(`
    mutation UpdateChannel($input: ChannelUpdateInput!) {
  updateChannel(input: $input) {
    id
    name
    description
    isActive
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<UpdateChannelMutation, UpdateChannelMutationVariables>;
export const DeleteChannelDocument = new TypedDocumentString(`
    mutation DeleteChannel($id: DbUuid!) {
  deleteChannel(id: $id)
}
    `) as unknown as TypedDocumentString<DeleteChannelMutation, DeleteChannelMutationVariables>;
export const GetCostCentersDocument = new TypedDocumentString(`
    query GetCostCenters($first: Int, $offset: Int) {
  costCenters(first: $first, offset: $offset) {
    id
    name
    code
    description
    state
    createdAt
    updatedAt
  }
  totalCostCenters
}
    `) as unknown as TypedDocumentString<GetCostCentersQuery, GetCostCentersQueryVariables>;
export const GetCostCenterDocument = new TypedDocumentString(`
    query GetCostCenter($id: DbUuid!) {
  costCenter(id: $id) {
    id
    name
    code
    description
    state
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<GetCostCenterQuery, GetCostCenterQueryVariables>;
export const GetAllCostCentersDocument = new TypedDocumentString(`
    query GetAllCostCenters {
  allCostCenters {
    id
    name
    code
    description
    state
  }
}
    `) as unknown as TypedDocumentString<GetAllCostCentersQuery, GetAllCostCentersQueryVariables>;
export const CreateCostCenterDocument = new TypedDocumentString(`
    mutation CreateCostCenter($name: String!, $code: String!, $description: String, $state: CostCenterState) {
  createCostCenter(
    name: $name
    code: $code
    description: $description
    state: $state
  ) {
    id
    name
    code
    description
    state
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<CreateCostCenterMutation, CreateCostCenterMutationVariables>;
export const UpdateCostCenterDocument = new TypedDocumentString(`
    mutation UpdateCostCenter($id: DbUuid!, $name: String, $code: String, $description: String, $state: CostCenterState) {
  updateCostCenter(
    id: $id
    name: $name
    code: $code
    description: $description
    state: $state
  ) {
    id
    name
    code
    description
    state
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<UpdateCostCenterMutation, UpdateCostCenterMutationVariables>;
export const DeleteCostCenterDocument = new TypedDocumentString(`
    mutation DeleteCostCenter($id: DbUuid!) {
  deleteCostCenter(id: $id)
}
    `) as unknown as TypedDocumentString<DeleteCostCenterMutation, DeleteCostCenterMutationVariables>;
export const GetPaymentMethodsDocument = new TypedDocumentString(`
    query GetPaymentMethods($first: Int!, $offset: Int!) {
  paymentMethods(first: $first, offset: $offset) {
    id
    name
    code
    description
    state
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<GetPaymentMethodsQuery, GetPaymentMethodsQueryVariables>;
export const GetTotalPaymentMethodsDocument = new TypedDocumentString(`
    query GetTotalPaymentMethods {
  totalPaymentMethods
}
    `) as unknown as TypedDocumentString<GetTotalPaymentMethodsQuery, GetTotalPaymentMethodsQueryVariables>;
export const GetPaymentMethodDocument = new TypedDocumentString(`
    query GetPaymentMethod($id: DbUuid!) {
  paymentMethod(id: $id) {
    id
    name
    code
    description
    state
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<GetPaymentMethodQuery, GetPaymentMethodQueryVariables>;
export const CreatePaymentMethodDocument = new TypedDocumentString(`
    mutation CreatePaymentMethod($name: String!, $code: String!, $description: String, $state: PaymentMethodState) {
  createPaymentMethod(
    name: $name
    code: $code
    description: $description
    state: $state
  ) {
    id
    name
    code
    description
    state
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<CreatePaymentMethodMutation, CreatePaymentMethodMutationVariables>;
export const UpdatePaymentMethodDocument = new TypedDocumentString(`
    mutation UpdatePaymentMethod($id: DbUuid!, $name: String, $code: String, $description: String, $state: PaymentMethodState) {
  updatePaymentMethod(
    id: $id
    name: $name
    code: $code
    description: $description
    state: $state
  ) {
    id
    name
    code
    description
    state
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<UpdatePaymentMethodMutation, UpdatePaymentMethodMutationVariables>;
export const DeletePaymentMethodDocument = new TypedDocumentString(`
    mutation DeletePaymentMethod($id: DbUuid!) {
  deletePaymentMethod(id: $id)
}
    `) as unknown as TypedDocumentString<DeletePaymentMethodMutation, DeletePaymentMethodMutationVariables>;
export const GetTaxGroupsDocument = new TypedDocumentString(`
    query GetTaxGroups($first: Int!, $offset: Int!) {
  taxGroups(first: $first, offset: $offset) {
    id
    name
    description
    createdAt
    updatedAt
    taxes {
      id
      name
      rate
    }
  }
  totalTaxGroups
}
    `) as unknown as TypedDocumentString<GetTaxGroupsQuery, GetTaxGroupsQueryVariables>;
export const GetTaxGroupDocument = new TypedDocumentString(`
    query GetTaxGroup($id: DbUuid!) {
  taxGroup(id: $id) {
    id
    name
    description
    createdAt
    updatedAt
    taxes {
      id
      name
      rate
    }
  }
}
    `) as unknown as TypedDocumentString<GetTaxGroupQuery, GetTaxGroupQueryVariables>;
export const GetAllTaxesDocument = new TypedDocumentString(`
    query GetAllTaxes {
  taxes(first: 100, offset: 0) {
    id
    name
    rate
  }
}
    `) as unknown as TypedDocumentString<GetAllTaxesQuery, GetAllTaxesQueryVariables>;
export const CreateTaxGroupDocument = new TypedDocumentString(`
    mutation CreateTaxGroup($input: TaxGroupNewInput!) {
  createTaxGroup(input: $input) {
    id
    name
    description
    createdAt
    updatedAt
    taxes {
      id
      name
      rate
    }
  }
}
    `) as unknown as TypedDocumentString<CreateTaxGroupMutation, CreateTaxGroupMutationVariables>;
export const UpdateTaxGroupDocument = new TypedDocumentString(`
    mutation UpdateTaxGroup($input: TaxGroupUpdateInput!) {
  updateTaxGroup(input: $input) {
    id
    name
    description
    createdAt
    updatedAt
    taxes {
      id
      name
      rate
    }
  }
}
    `) as unknown as TypedDocumentString<UpdateTaxGroupMutation, UpdateTaxGroupMutationVariables>;
export const DeleteTaxGroupDocument = new TypedDocumentString(`
    mutation DeleteTaxGroup($id: DbUuid!) {
  deleteTaxGroup(id: $id)
}
    `) as unknown as TypedDocumentString<DeleteTaxGroupMutation, DeleteTaxGroupMutationVariables>;
export const AssignTaxToGroupDocument = new TypedDocumentString(`
    mutation AssignTaxToGroup($taxGroupId: DbUuid!, $taxId: DbUuid!) {
  assignTaxToGroup(taxGroupId: $taxGroupId, taxId: $taxId)
}
    `) as unknown as TypedDocumentString<AssignTaxToGroupMutation, AssignTaxToGroupMutationVariables>;
export const RemoveTaxFromGroupDocument = new TypedDocumentString(`
    mutation RemoveTaxFromGroup($taxGroupId: DbUuid!, $taxId: DbUuid!) {
  removeTaxFromGroup(taxGroupId: $taxGroupId, taxId: $taxId)
}
    `) as unknown as TypedDocumentString<RemoveTaxFromGroupMutation, RemoveTaxFromGroupMutationVariables>;
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