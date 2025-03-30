/* eslint-disable */
import * as types from './graphql';



/**
 * Map of all GraphQL operations in the project.
 *
 * This map has several performance disadvantages:
 * 1. It is not tree-shakeable, so it will include all operations in the project.
 * 2. It is not minifiable, so the string of a GraphQL query will be multiple times inside the bundle.
 * 3. It does not support dead code elimination, so it will add unused operations.
 *
 * Therefore it is highly recommended to use the babel or swc plugin for production.
 * Learn more about it here: https://the-guild.dev/graphql/codegen/plugins/presets/preset-client#reducing-bundle-size
 */
type Documents = {
    "query GetAnalyticsOverview($days: Int!) {\n  analyticsOverview(days: $days) {\n    totalSales\n    totalOrders\n    totalCustomers\n    totalProducts\n  }\n}": typeof types.GetAnalyticsOverviewDocument,
    "query getCategories($first: Int!, $offset: Int!) {\n  itemCategories(first: $first, offset: $offset) {\n    id\n    name\n    description\n    state\n    createdAt\n    updatedAt\n  }\n}\n\nquery getCategory($id: DbUuid!) {\n  itemsCategory(id: $id) {\n    id\n    name\n    description\n    state\n    createdAt\n    updatedAt\n  }\n}\n\nmutation createCategory($input: ItemGroupNew!) {\n  createItemCategory(newCategory: $input) {\n    id\n    name\n    description\n    state\n    createdAt\n    updatedAt\n  }\n}\n\nmutation updateCategory($input: ItemGroupUpdate!) {\n  updateItemCategory(category: $input) {\n    id\n    name\n    description\n    state\n    createdAt\n    updatedAt\n  }\n}\n\nmutation deleteCategory($id: DbUuid!) {\n  deleteItemCategory(id: $id)\n}": typeof types.GetCategoriesDocument,
    "query getItems($first: Int!, $offset: Int!) {\n  items(first: $first, offset: $offset) {\n    id\n    name\n    description\n    nature\n    state\n    price\n    createdAt\n    updatedAt\n    category {\n      id\n      name\n      description\n      state\n      createdAt\n      updatedAt\n    }\n    taxes {\n      id\n      name\n      rate\n      description\n      createdAt\n      updatedAt\n    }\n  }\n}\n\nquery getItemCategories {\n  itemCategories {\n    id\n    name\n    description\n    state\n    createdAt\n    updatedAt\n  }\n}\n\nquery getItemTaxes {\n  taxes {\n    id\n    name\n    rate\n    description\n    createdAt\n    updatedAt\n  }\n}\n\nmutation createItem($input: NewItem!) {\n  createItem(item: $input) {\n    id\n    name\n    description\n    nature\n    state\n    price\n    createdAt\n    updatedAt\n    category {\n      id\n      name\n      description\n      state\n      createdAt\n      updatedAt\n    }\n    taxes {\n      id\n      name\n      rate\n      description\n      createdAt\n      updatedAt\n    }\n  }\n}\n\nmutation updateItem($input: UpdateItem!) {\n  updateItem(item: $input) {\n    id\n    name\n    description\n    nature\n    state\n    price\n    createdAt\n    updatedAt\n    category {\n      id\n      name\n      description\n      state\n      createdAt\n      updatedAt\n    }\n    taxes {\n      id\n      name\n      rate\n      description\n      createdAt\n      updatedAt\n    }\n  }\n}\n\nmutation deleteItem($id: DbUuid!) {\n  deleteItem(id: $id)\n}": typeof types.GetItemsDocument,
    "query GetCustomers($first: Int!, $offset: Int!) {\n  customers(first: $first, offset: $offset) {\n    id\n    fullName\n    email\n    phone\n    address\n    createdAt\n    updatedAt\n  }\n  totalCustomers\n}\n\nmutation CreateCustomer($input: CustomerNewInput!) {\n  createCustomer(customer: $input) {\n    id\n    fullName\n    email\n    phone\n    address\n    createdAt\n    updatedAt\n  }\n}\n\nmutation UpdateCustomer($input: CustomerUpdateInput!) {\n  updateCustomer(customer: $input) {\n    id\n    fullName\n    email\n    phone\n    address\n    createdAt\n    updatedAt\n  }\n}\n\nmutation DeleteCustomer($id: DbUuid!) {\n  deleteCustomer(id: $id)\n}": typeof types.GetCustomersDocument,
    "query GetSalesOrders($first: Int!, $offset: Int!) {\n  salesOrders(first: $first, offset: $offset) {\n    id\n    customerId\n    customerName\n    customerPhoneNumber\n    orderDate\n    netAmount\n    discAmount\n    taxableAmount\n    taxAmount\n    totalAmount\n    state\n    createdAt\n    updatedAt\n    customer {\n      id\n      fullName\n      phone\n      createdAt\n      updatedAt\n    }\n    items {\n      id\n      orderId\n      itemId\n      itemName\n      quantity\n      priceAmount\n      taxAmount\n      totalAmount\n      createdAt\n      updatedAt\n    }\n  }\n  totalSalesOrders\n}": typeof types.GetSalesOrdersDocument,
    "query getPosCategories($first: Int!) {\n  itemCategories(first: $first) {\n    id\n    name\n    description\n    state\n    createdAt\n    updatedAt\n  }\n}\n\nquery getPosItems($first: Int!, $offset: Int!) {\n  items(first: $first, offset: $offset) {\n    id\n    name\n    description\n    nature\n    state\n    price\n    createdAt\n    updatedAt\n    category {\n      id\n      name\n      description\n      state\n      createdAt\n      updatedAt\n    }\n    taxes {\n      id\n      name\n      rate\n      description\n      createdAt\n      updatedAt\n    }\n  }\n}\n\nquery getPosTaxes {\n  taxes {\n    id\n    name\n    rate\n    description\n    createdAt\n    updatedAt\n  }\n}\n\nquery getPosCustomerByPhone($phone: String!) {\n  customerByPhone(phone: $phone) {\n    id\n    fullName\n    phone\n    email\n    address\n    createdAt\n    updatedAt\n  }\n}\n\nmutation createPosCustomer($fullName: String!, $phone: String!) {\n  createCustomer(customer: {fullName: $fullName, phone: $phone}) {\n    id\n    fullName\n    phone\n    email\n    address\n    createdAt\n    updatedAt\n  }\n}\n\nmutation createSalesOrder($salesOrder: SalesOrderNewInput!) {\n  createSalesOrder(salesOrder: $salesOrder) {\n    id\n    customerName\n    orderDate\n    netAmount\n    taxAmount\n    totalAmount\n    state\n  }\n}": typeof types.GetPosCategoriesDocument,
    "query getPurchaseCategories($first: Int!, $offset: Int!) {\n  purchaseCategories(first: $first, offset: $offset) {\n    id\n    name\n    description\n    state\n    createdAt\n    updatedAt\n  }\n}\n\nquery getPurchaseCategory($id: DbUuid!) {\n  purchaseCategory(id: $id) {\n    id\n    name\n    description\n    state\n    createdAt\n    updatedAt\n  }\n}\n\nmutation createPurchaseCategory($name: String!, $description: String, $state: PurchaseCategoryState) {\n  createPurchaseCategory(name: $name, description: $description, state: $state) {\n    id\n    name\n    description\n    state\n    createdAt\n    updatedAt\n  }\n}\n\nmutation updatePurchaseCategory($id: DbUuid!, $name: String, $description: String, $state: PurchaseCategoryState) {\n  updatePurchaseCategory(\n    id: $id\n    name: $name\n    description: $description\n    state: $state\n  ) {\n    id\n    name\n    description\n    state\n    createdAt\n    updatedAt\n  }\n}\n\nmutation deletePurchaseCategory($id: DbUuid!) {\n  deletePurchaseCategory(id: $id)\n}": typeof types.GetPurchaseCategoriesDocument,
    "query GetExpenses($first: Int!, $offset: Int!) {\n  expenses(first: $first, offset: $offset) {\n    id\n    title\n    amount\n    expenseDate\n    categoryId\n    costCenterId\n    description\n    createdAt\n    updatedAt\n    category {\n      id\n      name\n    }\n    costCenter {\n      id\n      name\n      code\n    }\n  }\n  totalExpenses\n}\n\nquery GetPurchaseCategoriesForExpenses {\n  allPurchaseCategories {\n    id\n    name\n  }\n}\n\nquery GetCostCentersForExpenses {\n  allCostCenters {\n    id\n    name\n    code\n    state\n  }\n}\n\nquery GetExpensesByCategory($categoryId: DbUuid!, $first: Int!, $offset: Int!) {\n  expensesByCategory(categoryId: $categoryId, first: $first, offset: $offset) {\n    id\n    title\n    amount\n    expenseDate\n    categoryId\n    costCenterId\n    description\n    createdAt\n    updatedAt\n    category {\n      id\n      name\n    }\n    costCenter {\n      id\n      name\n      code\n    }\n  }\n}\n\nmutation CreateExpense($input: ExpenseNewInput!) {\n  createExpense(expense: $input) {\n    id\n    title\n    amount\n    expenseDate\n    categoryId\n    costCenterId\n    description\n    createdAt\n    updatedAt\n    category {\n      id\n      name\n    }\n    costCenter {\n      id\n      name\n      code\n    }\n  }\n}\n\nmutation UpdateExpense($input: ExpenseUpdateInput!) {\n  updateExpense(expense: $input) {\n    id\n    title\n    amount\n    expenseDate\n    categoryId\n    costCenterId\n    description\n    createdAt\n    updatedAt\n    category {\n      id\n      name\n    }\n    costCenter {\n      id\n      name\n      code\n    }\n  }\n}\n\nmutation DeleteExpense($id: DbUuid!) {\n  deleteExpense(id: $id)\n}": typeof types.GetExpensesDocument,
    "query GetBrands {\n  brands {\n    id\n    name\n    description\n    isActive\n    createdAt\n    updatedAt\n  }\n}\n\nquery GetBrand($id: DbUuid!) {\n  brand(id: $id) {\n    id\n    name\n    description\n    isActive\n    createdAt\n    updatedAt\n  }\n}\n\nquery GetActiveBrands {\n  activeBrands {\n    id\n    name\n    description\n    isActive\n    createdAt\n    updatedAt\n  }\n}\n\nmutation CreateBrand($input: BrandNewInput!) {\n  createBrand(input: $input) {\n    id\n    name\n    description\n    isActive\n    createdAt\n    updatedAt\n  }\n}\n\nmutation UpdateBrand($input: BrandUpdateInput!) {\n  updateBrand(input: $input) {\n    id\n    name\n    description\n    isActive\n    createdAt\n    updatedAt\n  }\n}\n\nmutation DeleteBrand($id: DbUuid!) {\n  deleteBrand(id: $id)\n}": typeof types.GetBrandsDocument,
    "query GetChannels {\n  channels {\n    id\n    name\n    description\n    isActive\n    createdAt\n    updatedAt\n  }\n}\n\nquery GetChannel($id: DbUuid!) {\n  channel(id: $id) {\n    id\n    name\n    description\n    isActive\n    createdAt\n    updatedAt\n  }\n}\n\nquery GetActiveChannels {\n  activeChannels {\n    id\n    name\n    description\n    isActive\n    createdAt\n    updatedAt\n  }\n}\n\nmutation CreateChannel($input: ChannelNewInput!) {\n  createChannel(input: $input) {\n    id\n    name\n    description\n    isActive\n    createdAt\n    updatedAt\n  }\n}\n\nmutation UpdateChannel($input: ChannelUpdateInput!) {\n  updateChannel(input: $input) {\n    id\n    name\n    description\n    isActive\n    createdAt\n    updatedAt\n  }\n}\n\nmutation DeleteChannel($id: DbUuid!) {\n  deleteChannel(id: $id)\n}": typeof types.GetChannelsDocument,
    "query GetCostCenters($first: Int, $offset: Int) {\n  costCenters(first: $first, offset: $offset) {\n    id\n    name\n    code\n    description\n    state\n    createdAt\n    updatedAt\n  }\n  totalCostCenters\n}\n\nquery GetCostCenter($id: DbUuid!) {\n  costCenter(id: $id) {\n    id\n    name\n    code\n    description\n    state\n    createdAt\n    updatedAt\n  }\n}\n\nquery GetAllCostCenters {\n  allCostCenters {\n    id\n    name\n    code\n    description\n    state\n  }\n}\n\nmutation CreateCostCenter($name: String!, $code: String!, $description: String, $state: CostCenterState) {\n  createCostCenter(\n    name: $name\n    code: $code\n    description: $description\n    state: $state\n  ) {\n    id\n    name\n    code\n    description\n    state\n    createdAt\n    updatedAt\n  }\n}\n\nmutation UpdateCostCenter($id: DbUuid!, $name: String, $code: String, $description: String, $state: CostCenterState) {\n  updateCostCenter(\n    id: $id\n    name: $name\n    code: $code\n    description: $description\n    state: $state\n  ) {\n    id\n    name\n    code\n    description\n    state\n    createdAt\n    updatedAt\n  }\n}\n\nmutation DeleteCostCenter($id: DbUuid!) {\n  deleteCostCenter(id: $id)\n}": typeof types.GetCostCentersDocument,
    "query GetTaxes($first: Int!, $offset: Int!) {\n  taxes(first: $first, offset: $offset) {\n    id\n    name\n    rate\n    description\n    createdAt\n    updatedAt\n  }\n  totalTaxes\n}\n\nmutation CreateTax($input: TaxNewInput!) {\n  createTax(input: $input) {\n    id\n    name\n    rate\n    description\n    createdAt\n    updatedAt\n  }\n}\n\nmutation UpdateTax($input: TaxUpdateInput!) {\n  updateTax(input: $input) {\n    id\n    name\n    rate\n    description\n    createdAt\n    updatedAt\n  }\n}\n\nmutation DeleteTax($id: DbUuid!) {\n  deleteTax(id: $id)\n}": typeof types.GetTaxesDocument,
    "query GetSuppliers($first: Int!, $offset: Int!) {\n  suppliers(first: $first, offset: $offset) {\n    id\n    name\n    address\n    phone\n    createdAt\n    updatedAt\n  }\n  totalSuppliers\n}\n\nmutation CreateSupplier($input: SupplierNewInput!) {\n  createSupplier(supplier: $input) {\n    id\n    name\n    address\n    phone\n    createdAt\n    updatedAt\n  }\n}\n\nmutation UpdateSupplier($input: SupplierUpdateInput!) {\n  updateSupplier(supplier: $input) {\n    id\n    name\n    address\n    phone\n    createdAt\n    updatedAt\n  }\n}\n\nmutation DeleteSupplier($id: DbUuid!) {\n  deleteSupplier(id: $id)\n}": typeof types.GetSuppliersDocument,
};
const documents: Documents = {
    "query GetAnalyticsOverview($days: Int!) {\n  analyticsOverview(days: $days) {\n    totalSales\n    totalOrders\n    totalCustomers\n    totalProducts\n  }\n}": types.GetAnalyticsOverviewDocument,
    "query getCategories($first: Int!, $offset: Int!) {\n  itemCategories(first: $first, offset: $offset) {\n    id\n    name\n    description\n    state\n    createdAt\n    updatedAt\n  }\n}\n\nquery getCategory($id: DbUuid!) {\n  itemsCategory(id: $id) {\n    id\n    name\n    description\n    state\n    createdAt\n    updatedAt\n  }\n}\n\nmutation createCategory($input: ItemGroupNew!) {\n  createItemCategory(newCategory: $input) {\n    id\n    name\n    description\n    state\n    createdAt\n    updatedAt\n  }\n}\n\nmutation updateCategory($input: ItemGroupUpdate!) {\n  updateItemCategory(category: $input) {\n    id\n    name\n    description\n    state\n    createdAt\n    updatedAt\n  }\n}\n\nmutation deleteCategory($id: DbUuid!) {\n  deleteItemCategory(id: $id)\n}": types.GetCategoriesDocument,
    "query getItems($first: Int!, $offset: Int!) {\n  items(first: $first, offset: $offset) {\n    id\n    name\n    description\n    nature\n    state\n    price\n    createdAt\n    updatedAt\n    category {\n      id\n      name\n      description\n      state\n      createdAt\n      updatedAt\n    }\n    taxes {\n      id\n      name\n      rate\n      description\n      createdAt\n      updatedAt\n    }\n  }\n}\n\nquery getItemCategories {\n  itemCategories {\n    id\n    name\n    description\n    state\n    createdAt\n    updatedAt\n  }\n}\n\nquery getItemTaxes {\n  taxes {\n    id\n    name\n    rate\n    description\n    createdAt\n    updatedAt\n  }\n}\n\nmutation createItem($input: NewItem!) {\n  createItem(item: $input) {\n    id\n    name\n    description\n    nature\n    state\n    price\n    createdAt\n    updatedAt\n    category {\n      id\n      name\n      description\n      state\n      createdAt\n      updatedAt\n    }\n    taxes {\n      id\n      name\n      rate\n      description\n      createdAt\n      updatedAt\n    }\n  }\n}\n\nmutation updateItem($input: UpdateItem!) {\n  updateItem(item: $input) {\n    id\n    name\n    description\n    nature\n    state\n    price\n    createdAt\n    updatedAt\n    category {\n      id\n      name\n      description\n      state\n      createdAt\n      updatedAt\n    }\n    taxes {\n      id\n      name\n      rate\n      description\n      createdAt\n      updatedAt\n    }\n  }\n}\n\nmutation deleteItem($id: DbUuid!) {\n  deleteItem(id: $id)\n}": types.GetItemsDocument,
    "query GetCustomers($first: Int!, $offset: Int!) {\n  customers(first: $first, offset: $offset) {\n    id\n    fullName\n    email\n    phone\n    address\n    createdAt\n    updatedAt\n  }\n  totalCustomers\n}\n\nmutation CreateCustomer($input: CustomerNewInput!) {\n  createCustomer(customer: $input) {\n    id\n    fullName\n    email\n    phone\n    address\n    createdAt\n    updatedAt\n  }\n}\n\nmutation UpdateCustomer($input: CustomerUpdateInput!) {\n  updateCustomer(customer: $input) {\n    id\n    fullName\n    email\n    phone\n    address\n    createdAt\n    updatedAt\n  }\n}\n\nmutation DeleteCustomer($id: DbUuid!) {\n  deleteCustomer(id: $id)\n}": types.GetCustomersDocument,
    "query GetSalesOrders($first: Int!, $offset: Int!) {\n  salesOrders(first: $first, offset: $offset) {\n    id\n    customerId\n    customerName\n    customerPhoneNumber\n    orderDate\n    netAmount\n    discAmount\n    taxableAmount\n    taxAmount\n    totalAmount\n    state\n    createdAt\n    updatedAt\n    customer {\n      id\n      fullName\n      phone\n      createdAt\n      updatedAt\n    }\n    items {\n      id\n      orderId\n      itemId\n      itemName\n      quantity\n      priceAmount\n      taxAmount\n      totalAmount\n      createdAt\n      updatedAt\n    }\n  }\n  totalSalesOrders\n}": types.GetSalesOrdersDocument,
    "query getPosCategories($first: Int!) {\n  itemCategories(first: $first) {\n    id\n    name\n    description\n    state\n    createdAt\n    updatedAt\n  }\n}\n\nquery getPosItems($first: Int!, $offset: Int!) {\n  items(first: $first, offset: $offset) {\n    id\n    name\n    description\n    nature\n    state\n    price\n    createdAt\n    updatedAt\n    category {\n      id\n      name\n      description\n      state\n      createdAt\n      updatedAt\n    }\n    taxes {\n      id\n      name\n      rate\n      description\n      createdAt\n      updatedAt\n    }\n  }\n}\n\nquery getPosTaxes {\n  taxes {\n    id\n    name\n    rate\n    description\n    createdAt\n    updatedAt\n  }\n}\n\nquery getPosCustomerByPhone($phone: String!) {\n  customerByPhone(phone: $phone) {\n    id\n    fullName\n    phone\n    email\n    address\n    createdAt\n    updatedAt\n  }\n}\n\nmutation createPosCustomer($fullName: String!, $phone: String!) {\n  createCustomer(customer: {fullName: $fullName, phone: $phone}) {\n    id\n    fullName\n    phone\n    email\n    address\n    createdAt\n    updatedAt\n  }\n}\n\nmutation createSalesOrder($salesOrder: SalesOrderNewInput!) {\n  createSalesOrder(salesOrder: $salesOrder) {\n    id\n    customerName\n    orderDate\n    netAmount\n    taxAmount\n    totalAmount\n    state\n  }\n}": types.GetPosCategoriesDocument,
    "query getPurchaseCategories($first: Int!, $offset: Int!) {\n  purchaseCategories(first: $first, offset: $offset) {\n    id\n    name\n    description\n    state\n    createdAt\n    updatedAt\n  }\n}\n\nquery getPurchaseCategory($id: DbUuid!) {\n  purchaseCategory(id: $id) {\n    id\n    name\n    description\n    state\n    createdAt\n    updatedAt\n  }\n}\n\nmutation createPurchaseCategory($name: String!, $description: String, $state: PurchaseCategoryState) {\n  createPurchaseCategory(name: $name, description: $description, state: $state) {\n    id\n    name\n    description\n    state\n    createdAt\n    updatedAt\n  }\n}\n\nmutation updatePurchaseCategory($id: DbUuid!, $name: String, $description: String, $state: PurchaseCategoryState) {\n  updatePurchaseCategory(\n    id: $id\n    name: $name\n    description: $description\n    state: $state\n  ) {\n    id\n    name\n    description\n    state\n    createdAt\n    updatedAt\n  }\n}\n\nmutation deletePurchaseCategory($id: DbUuid!) {\n  deletePurchaseCategory(id: $id)\n}": types.GetPurchaseCategoriesDocument,
    "query GetExpenses($first: Int!, $offset: Int!) {\n  expenses(first: $first, offset: $offset) {\n    id\n    title\n    amount\n    expenseDate\n    categoryId\n    costCenterId\n    description\n    createdAt\n    updatedAt\n    category {\n      id\n      name\n    }\n    costCenter {\n      id\n      name\n      code\n    }\n  }\n  totalExpenses\n}\n\nquery GetPurchaseCategoriesForExpenses {\n  allPurchaseCategories {\n    id\n    name\n  }\n}\n\nquery GetCostCentersForExpenses {\n  allCostCenters {\n    id\n    name\n    code\n    state\n  }\n}\n\nquery GetExpensesByCategory($categoryId: DbUuid!, $first: Int!, $offset: Int!) {\n  expensesByCategory(categoryId: $categoryId, first: $first, offset: $offset) {\n    id\n    title\n    amount\n    expenseDate\n    categoryId\n    costCenterId\n    description\n    createdAt\n    updatedAt\n    category {\n      id\n      name\n    }\n    costCenter {\n      id\n      name\n      code\n    }\n  }\n}\n\nmutation CreateExpense($input: ExpenseNewInput!) {\n  createExpense(expense: $input) {\n    id\n    title\n    amount\n    expenseDate\n    categoryId\n    costCenterId\n    description\n    createdAt\n    updatedAt\n    category {\n      id\n      name\n    }\n    costCenter {\n      id\n      name\n      code\n    }\n  }\n}\n\nmutation UpdateExpense($input: ExpenseUpdateInput!) {\n  updateExpense(expense: $input) {\n    id\n    title\n    amount\n    expenseDate\n    categoryId\n    costCenterId\n    description\n    createdAt\n    updatedAt\n    category {\n      id\n      name\n    }\n    costCenter {\n      id\n      name\n      code\n    }\n  }\n}\n\nmutation DeleteExpense($id: DbUuid!) {\n  deleteExpense(id: $id)\n}": types.GetExpensesDocument,
    "query GetBrands {\n  brands {\n    id\n    name\n    description\n    isActive\n    createdAt\n    updatedAt\n  }\n}\n\nquery GetBrand($id: DbUuid!) {\n  brand(id: $id) {\n    id\n    name\n    description\n    isActive\n    createdAt\n    updatedAt\n  }\n}\n\nquery GetActiveBrands {\n  activeBrands {\n    id\n    name\n    description\n    isActive\n    createdAt\n    updatedAt\n  }\n}\n\nmutation CreateBrand($input: BrandNewInput!) {\n  createBrand(input: $input) {\n    id\n    name\n    description\n    isActive\n    createdAt\n    updatedAt\n  }\n}\n\nmutation UpdateBrand($input: BrandUpdateInput!) {\n  updateBrand(input: $input) {\n    id\n    name\n    description\n    isActive\n    createdAt\n    updatedAt\n  }\n}\n\nmutation DeleteBrand($id: DbUuid!) {\n  deleteBrand(id: $id)\n}": types.GetBrandsDocument,
    "query GetChannels {\n  channels {\n    id\n    name\n    description\n    isActive\n    createdAt\n    updatedAt\n  }\n}\n\nquery GetChannel($id: DbUuid!) {\n  channel(id: $id) {\n    id\n    name\n    description\n    isActive\n    createdAt\n    updatedAt\n  }\n}\n\nquery GetActiveChannels {\n  activeChannels {\n    id\n    name\n    description\n    isActive\n    createdAt\n    updatedAt\n  }\n}\n\nmutation CreateChannel($input: ChannelNewInput!) {\n  createChannel(input: $input) {\n    id\n    name\n    description\n    isActive\n    createdAt\n    updatedAt\n  }\n}\n\nmutation UpdateChannel($input: ChannelUpdateInput!) {\n  updateChannel(input: $input) {\n    id\n    name\n    description\n    isActive\n    createdAt\n    updatedAt\n  }\n}\n\nmutation DeleteChannel($id: DbUuid!) {\n  deleteChannel(id: $id)\n}": types.GetChannelsDocument,
    "query GetCostCenters($first: Int, $offset: Int) {\n  costCenters(first: $first, offset: $offset) {\n    id\n    name\n    code\n    description\n    state\n    createdAt\n    updatedAt\n  }\n  totalCostCenters\n}\n\nquery GetCostCenter($id: DbUuid!) {\n  costCenter(id: $id) {\n    id\n    name\n    code\n    description\n    state\n    createdAt\n    updatedAt\n  }\n}\n\nquery GetAllCostCenters {\n  allCostCenters {\n    id\n    name\n    code\n    description\n    state\n  }\n}\n\nmutation CreateCostCenter($name: String!, $code: String!, $description: String, $state: CostCenterState) {\n  createCostCenter(\n    name: $name\n    code: $code\n    description: $description\n    state: $state\n  ) {\n    id\n    name\n    code\n    description\n    state\n    createdAt\n    updatedAt\n  }\n}\n\nmutation UpdateCostCenter($id: DbUuid!, $name: String, $code: String, $description: String, $state: CostCenterState) {\n  updateCostCenter(\n    id: $id\n    name: $name\n    code: $code\n    description: $description\n    state: $state\n  ) {\n    id\n    name\n    code\n    description\n    state\n    createdAt\n    updatedAt\n  }\n}\n\nmutation DeleteCostCenter($id: DbUuid!) {\n  deleteCostCenter(id: $id)\n}": types.GetCostCentersDocument,
    "query GetTaxes($first: Int!, $offset: Int!) {\n  taxes(first: $first, offset: $offset) {\n    id\n    name\n    rate\n    description\n    createdAt\n    updatedAt\n  }\n  totalTaxes\n}\n\nmutation CreateTax($input: TaxNewInput!) {\n  createTax(input: $input) {\n    id\n    name\n    rate\n    description\n    createdAt\n    updatedAt\n  }\n}\n\nmutation UpdateTax($input: TaxUpdateInput!) {\n  updateTax(input: $input) {\n    id\n    name\n    rate\n    description\n    createdAt\n    updatedAt\n  }\n}\n\nmutation DeleteTax($id: DbUuid!) {\n  deleteTax(id: $id)\n}": types.GetTaxesDocument,
    "query GetSuppliers($first: Int!, $offset: Int!) {\n  suppliers(first: $first, offset: $offset) {\n    id\n    name\n    address\n    phone\n    createdAt\n    updatedAt\n  }\n  totalSuppliers\n}\n\nmutation CreateSupplier($input: SupplierNewInput!) {\n  createSupplier(supplier: $input) {\n    id\n    name\n    address\n    phone\n    createdAt\n    updatedAt\n  }\n}\n\nmutation UpdateSupplier($input: SupplierUpdateInput!) {\n  updateSupplier(supplier: $input) {\n    id\n    name\n    address\n    phone\n    createdAt\n    updatedAt\n  }\n}\n\nmutation DeleteSupplier($id: DbUuid!) {\n  deleteSupplier(id: $id)\n}": types.GetSuppliersDocument,
};

/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetAnalyticsOverview($days: Int!) {\n  analyticsOverview(days: $days) {\n    totalSales\n    totalOrders\n    totalCustomers\n    totalProducts\n  }\n}"): typeof import('./graphql').GetAnalyticsOverviewDocument;
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query getCategories($first: Int!, $offset: Int!) {\n  itemCategories(first: $first, offset: $offset) {\n    id\n    name\n    description\n    state\n    createdAt\n    updatedAt\n  }\n}\n\nquery getCategory($id: DbUuid!) {\n  itemsCategory(id: $id) {\n    id\n    name\n    description\n    state\n    createdAt\n    updatedAt\n  }\n}\n\nmutation createCategory($input: ItemGroupNew!) {\n  createItemCategory(newCategory: $input) {\n    id\n    name\n    description\n    state\n    createdAt\n    updatedAt\n  }\n}\n\nmutation updateCategory($input: ItemGroupUpdate!) {\n  updateItemCategory(category: $input) {\n    id\n    name\n    description\n    state\n    createdAt\n    updatedAt\n  }\n}\n\nmutation deleteCategory($id: DbUuid!) {\n  deleteItemCategory(id: $id)\n}"): typeof import('./graphql').GetCategoriesDocument;
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query getItems($first: Int!, $offset: Int!) {\n  items(first: $first, offset: $offset) {\n    id\n    name\n    description\n    nature\n    state\n    price\n    createdAt\n    updatedAt\n    category {\n      id\n      name\n      description\n      state\n      createdAt\n      updatedAt\n    }\n    taxes {\n      id\n      name\n      rate\n      description\n      createdAt\n      updatedAt\n    }\n  }\n}\n\nquery getItemCategories {\n  itemCategories {\n    id\n    name\n    description\n    state\n    createdAt\n    updatedAt\n  }\n}\n\nquery getItemTaxes {\n  taxes {\n    id\n    name\n    rate\n    description\n    createdAt\n    updatedAt\n  }\n}\n\nmutation createItem($input: NewItem!) {\n  createItem(item: $input) {\n    id\n    name\n    description\n    nature\n    state\n    price\n    createdAt\n    updatedAt\n    category {\n      id\n      name\n      description\n      state\n      createdAt\n      updatedAt\n    }\n    taxes {\n      id\n      name\n      rate\n      description\n      createdAt\n      updatedAt\n    }\n  }\n}\n\nmutation updateItem($input: UpdateItem!) {\n  updateItem(item: $input) {\n    id\n    name\n    description\n    nature\n    state\n    price\n    createdAt\n    updatedAt\n    category {\n      id\n      name\n      description\n      state\n      createdAt\n      updatedAt\n    }\n    taxes {\n      id\n      name\n      rate\n      description\n      createdAt\n      updatedAt\n    }\n  }\n}\n\nmutation deleteItem($id: DbUuid!) {\n  deleteItem(id: $id)\n}"): typeof import('./graphql').GetItemsDocument;
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetCustomers($first: Int!, $offset: Int!) {\n  customers(first: $first, offset: $offset) {\n    id\n    fullName\n    email\n    phone\n    address\n    createdAt\n    updatedAt\n  }\n  totalCustomers\n}\n\nmutation CreateCustomer($input: CustomerNewInput!) {\n  createCustomer(customer: $input) {\n    id\n    fullName\n    email\n    phone\n    address\n    createdAt\n    updatedAt\n  }\n}\n\nmutation UpdateCustomer($input: CustomerUpdateInput!) {\n  updateCustomer(customer: $input) {\n    id\n    fullName\n    email\n    phone\n    address\n    createdAt\n    updatedAt\n  }\n}\n\nmutation DeleteCustomer($id: DbUuid!) {\n  deleteCustomer(id: $id)\n}"): typeof import('./graphql').GetCustomersDocument;
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetSalesOrders($first: Int!, $offset: Int!) {\n  salesOrders(first: $first, offset: $offset) {\n    id\n    customerId\n    customerName\n    customerPhoneNumber\n    orderDate\n    netAmount\n    discAmount\n    taxableAmount\n    taxAmount\n    totalAmount\n    state\n    createdAt\n    updatedAt\n    customer {\n      id\n      fullName\n      phone\n      createdAt\n      updatedAt\n    }\n    items {\n      id\n      orderId\n      itemId\n      itemName\n      quantity\n      priceAmount\n      taxAmount\n      totalAmount\n      createdAt\n      updatedAt\n    }\n  }\n  totalSalesOrders\n}"): typeof import('./graphql').GetSalesOrdersDocument;
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query getPosCategories($first: Int!) {\n  itemCategories(first: $first) {\n    id\n    name\n    description\n    state\n    createdAt\n    updatedAt\n  }\n}\n\nquery getPosItems($first: Int!, $offset: Int!) {\n  items(first: $first, offset: $offset) {\n    id\n    name\n    description\n    nature\n    state\n    price\n    createdAt\n    updatedAt\n    category {\n      id\n      name\n      description\n      state\n      createdAt\n      updatedAt\n    }\n    taxes {\n      id\n      name\n      rate\n      description\n      createdAt\n      updatedAt\n    }\n  }\n}\n\nquery getPosTaxes {\n  taxes {\n    id\n    name\n    rate\n    description\n    createdAt\n    updatedAt\n  }\n}\n\nquery getPosCustomerByPhone($phone: String!) {\n  customerByPhone(phone: $phone) {\n    id\n    fullName\n    phone\n    email\n    address\n    createdAt\n    updatedAt\n  }\n}\n\nmutation createPosCustomer($fullName: String!, $phone: String!) {\n  createCustomer(customer: {fullName: $fullName, phone: $phone}) {\n    id\n    fullName\n    phone\n    email\n    address\n    createdAt\n    updatedAt\n  }\n}\n\nmutation createSalesOrder($salesOrder: SalesOrderNewInput!) {\n  createSalesOrder(salesOrder: $salesOrder) {\n    id\n    customerName\n    orderDate\n    netAmount\n    taxAmount\n    totalAmount\n    state\n  }\n}"): typeof import('./graphql').GetPosCategoriesDocument;
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query getPurchaseCategories($first: Int!, $offset: Int!) {\n  purchaseCategories(first: $first, offset: $offset) {\n    id\n    name\n    description\n    state\n    createdAt\n    updatedAt\n  }\n}\n\nquery getPurchaseCategory($id: DbUuid!) {\n  purchaseCategory(id: $id) {\n    id\n    name\n    description\n    state\n    createdAt\n    updatedAt\n  }\n}\n\nmutation createPurchaseCategory($name: String!, $description: String, $state: PurchaseCategoryState) {\n  createPurchaseCategory(name: $name, description: $description, state: $state) {\n    id\n    name\n    description\n    state\n    createdAt\n    updatedAt\n  }\n}\n\nmutation updatePurchaseCategory($id: DbUuid!, $name: String, $description: String, $state: PurchaseCategoryState) {\n  updatePurchaseCategory(\n    id: $id\n    name: $name\n    description: $description\n    state: $state\n  ) {\n    id\n    name\n    description\n    state\n    createdAt\n    updatedAt\n  }\n}\n\nmutation deletePurchaseCategory($id: DbUuid!) {\n  deletePurchaseCategory(id: $id)\n}"): typeof import('./graphql').GetPurchaseCategoriesDocument;
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetExpenses($first: Int!, $offset: Int!) {\n  expenses(first: $first, offset: $offset) {\n    id\n    title\n    amount\n    expenseDate\n    categoryId\n    costCenterId\n    description\n    createdAt\n    updatedAt\n    category {\n      id\n      name\n    }\n    costCenter {\n      id\n      name\n      code\n    }\n  }\n  totalExpenses\n}\n\nquery GetPurchaseCategoriesForExpenses {\n  allPurchaseCategories {\n    id\n    name\n  }\n}\n\nquery GetCostCentersForExpenses {\n  allCostCenters {\n    id\n    name\n    code\n    state\n  }\n}\n\nquery GetExpensesByCategory($categoryId: DbUuid!, $first: Int!, $offset: Int!) {\n  expensesByCategory(categoryId: $categoryId, first: $first, offset: $offset) {\n    id\n    title\n    amount\n    expenseDate\n    categoryId\n    costCenterId\n    description\n    createdAt\n    updatedAt\n    category {\n      id\n      name\n    }\n    costCenter {\n      id\n      name\n      code\n    }\n  }\n}\n\nmutation CreateExpense($input: ExpenseNewInput!) {\n  createExpense(expense: $input) {\n    id\n    title\n    amount\n    expenseDate\n    categoryId\n    costCenterId\n    description\n    createdAt\n    updatedAt\n    category {\n      id\n      name\n    }\n    costCenter {\n      id\n      name\n      code\n    }\n  }\n}\n\nmutation UpdateExpense($input: ExpenseUpdateInput!) {\n  updateExpense(expense: $input) {\n    id\n    title\n    amount\n    expenseDate\n    categoryId\n    costCenterId\n    description\n    createdAt\n    updatedAt\n    category {\n      id\n      name\n    }\n    costCenter {\n      id\n      name\n      code\n    }\n  }\n}\n\nmutation DeleteExpense($id: DbUuid!) {\n  deleteExpense(id: $id)\n}"): typeof import('./graphql').GetExpensesDocument;
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetBrands {\n  brands {\n    id\n    name\n    description\n    isActive\n    createdAt\n    updatedAt\n  }\n}\n\nquery GetBrand($id: DbUuid!) {\n  brand(id: $id) {\n    id\n    name\n    description\n    isActive\n    createdAt\n    updatedAt\n  }\n}\n\nquery GetActiveBrands {\n  activeBrands {\n    id\n    name\n    description\n    isActive\n    createdAt\n    updatedAt\n  }\n}\n\nmutation CreateBrand($input: BrandNewInput!) {\n  createBrand(input: $input) {\n    id\n    name\n    description\n    isActive\n    createdAt\n    updatedAt\n  }\n}\n\nmutation UpdateBrand($input: BrandUpdateInput!) {\n  updateBrand(input: $input) {\n    id\n    name\n    description\n    isActive\n    createdAt\n    updatedAt\n  }\n}\n\nmutation DeleteBrand($id: DbUuid!) {\n  deleteBrand(id: $id)\n}"): typeof import('./graphql').GetBrandsDocument;
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetChannels {\n  channels {\n    id\n    name\n    description\n    isActive\n    createdAt\n    updatedAt\n  }\n}\n\nquery GetChannel($id: DbUuid!) {\n  channel(id: $id) {\n    id\n    name\n    description\n    isActive\n    createdAt\n    updatedAt\n  }\n}\n\nquery GetActiveChannels {\n  activeChannels {\n    id\n    name\n    description\n    isActive\n    createdAt\n    updatedAt\n  }\n}\n\nmutation CreateChannel($input: ChannelNewInput!) {\n  createChannel(input: $input) {\n    id\n    name\n    description\n    isActive\n    createdAt\n    updatedAt\n  }\n}\n\nmutation UpdateChannel($input: ChannelUpdateInput!) {\n  updateChannel(input: $input) {\n    id\n    name\n    description\n    isActive\n    createdAt\n    updatedAt\n  }\n}\n\nmutation DeleteChannel($id: DbUuid!) {\n  deleteChannel(id: $id)\n}"): typeof import('./graphql').GetChannelsDocument;
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetCostCenters($first: Int, $offset: Int) {\n  costCenters(first: $first, offset: $offset) {\n    id\n    name\n    code\n    description\n    state\n    createdAt\n    updatedAt\n  }\n  totalCostCenters\n}\n\nquery GetCostCenter($id: DbUuid!) {\n  costCenter(id: $id) {\n    id\n    name\n    code\n    description\n    state\n    createdAt\n    updatedAt\n  }\n}\n\nquery GetAllCostCenters {\n  allCostCenters {\n    id\n    name\n    code\n    description\n    state\n  }\n}\n\nmutation CreateCostCenter($name: String!, $code: String!, $description: String, $state: CostCenterState) {\n  createCostCenter(\n    name: $name\n    code: $code\n    description: $description\n    state: $state\n  ) {\n    id\n    name\n    code\n    description\n    state\n    createdAt\n    updatedAt\n  }\n}\n\nmutation UpdateCostCenter($id: DbUuid!, $name: String, $code: String, $description: String, $state: CostCenterState) {\n  updateCostCenter(\n    id: $id\n    name: $name\n    code: $code\n    description: $description\n    state: $state\n  ) {\n    id\n    name\n    code\n    description\n    state\n    createdAt\n    updatedAt\n  }\n}\n\nmutation DeleteCostCenter($id: DbUuid!) {\n  deleteCostCenter(id: $id)\n}"): typeof import('./graphql').GetCostCentersDocument;
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetTaxes($first: Int!, $offset: Int!) {\n  taxes(first: $first, offset: $offset) {\n    id\n    name\n    rate\n    description\n    createdAt\n    updatedAt\n  }\n  totalTaxes\n}\n\nmutation CreateTax($input: TaxNewInput!) {\n  createTax(input: $input) {\n    id\n    name\n    rate\n    description\n    createdAt\n    updatedAt\n  }\n}\n\nmutation UpdateTax($input: TaxUpdateInput!) {\n  updateTax(input: $input) {\n    id\n    name\n    rate\n    description\n    createdAt\n    updatedAt\n  }\n}\n\nmutation DeleteTax($id: DbUuid!) {\n  deleteTax(id: $id)\n}"): typeof import('./graphql').GetTaxesDocument;
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query GetSuppliers($first: Int!, $offset: Int!) {\n  suppliers(first: $first, offset: $offset) {\n    id\n    name\n    address\n    phone\n    createdAt\n    updatedAt\n  }\n  totalSuppliers\n}\n\nmutation CreateSupplier($input: SupplierNewInput!) {\n  createSupplier(supplier: $input) {\n    id\n    name\n    address\n    phone\n    createdAt\n    updatedAt\n  }\n}\n\nmutation UpdateSupplier($input: SupplierUpdateInput!) {\n  updateSupplier(supplier: $input) {\n    id\n    name\n    address\n    phone\n    createdAt\n    updatedAt\n  }\n}\n\nmutation DeleteSupplier($id: DbUuid!) {\n  deleteSupplier(id: $id)\n}"): typeof import('./graphql').GetSuppliersDocument;


export function graphql(source: string) {
  return (documents as any)[source] ?? {};
}
