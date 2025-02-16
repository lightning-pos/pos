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
    "query getCategories($first: Int!, $offset: Int!) {\n  itemCategories(first: $first, offset: $offset) {\n    id\n    name\n    description\n    state\n    createdAt\n    updatedAt\n  }\n}\n\nquery getCategory($id: DbUuid!) {\n  itemsCategory(id: $id) {\n    id\n    name\n    description\n    state\n    createdAt\n    updatedAt\n  }\n}\n\nmutation createCategory($input: ItemGroupNew!) {\n  createItemCategory(newCategory: $input) {\n    id\n    name\n    description\n    state\n    createdAt\n    updatedAt\n  }\n}\n\nmutation updateCategory($input: ItemGroupUpdate!) {\n  updateItemCategory(category: $input) {\n    id\n    name\n    description\n    state\n    createdAt\n    updatedAt\n  }\n}\n\nmutation deleteCategory($id: DbUuid!) {\n  deleteItemCategory(id: $id)\n}": typeof types.GetCategoriesDocument,
};
const documents: Documents = {
    "query getCategories($first: Int!, $offset: Int!) {\n  itemCategories(first: $first, offset: $offset) {\n    id\n    name\n    description\n    state\n    createdAt\n    updatedAt\n  }\n}\n\nquery getCategory($id: DbUuid!) {\n  itemsCategory(id: $id) {\n    id\n    name\n    description\n    state\n    createdAt\n    updatedAt\n  }\n}\n\nmutation createCategory($input: ItemGroupNew!) {\n  createItemCategory(newCategory: $input) {\n    id\n    name\n    description\n    state\n    createdAt\n    updatedAt\n  }\n}\n\nmutation updateCategory($input: ItemGroupUpdate!) {\n  updateItemCategory(category: $input) {\n    id\n    name\n    description\n    state\n    createdAt\n    updatedAt\n  }\n}\n\nmutation deleteCategory($id: DbUuid!) {\n  deleteItemCategory(id: $id)\n}": types.GetCategoriesDocument,
};

/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "query getCategories($first: Int!, $offset: Int!) {\n  itemCategories(first: $first, offset: $offset) {\n    id\n    name\n    description\n    state\n    createdAt\n    updatedAt\n  }\n}\n\nquery getCategory($id: DbUuid!) {\n  itemsCategory(id: $id) {\n    id\n    name\n    description\n    state\n    createdAt\n    updatedAt\n  }\n}\n\nmutation createCategory($input: ItemGroupNew!) {\n  createItemCategory(newCategory: $input) {\n    id\n    name\n    description\n    state\n    createdAt\n    updatedAt\n  }\n}\n\nmutation updateCategory($input: ItemGroupUpdate!) {\n  updateItemCategory(category: $input) {\n    id\n    name\n    description\n    state\n    createdAt\n    updatedAt\n  }\n}\n\nmutation deleteCategory($id: DbUuid!) {\n  deleteItemCategory(id: $id)\n}"): typeof import('./graphql').GetCategoriesDocument;


export function graphql(source: string) {
  return (documents as any)[source] ?? {};
}
