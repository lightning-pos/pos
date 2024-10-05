"use client";
import React, { useState, useEffect, useCallback } from "react";
import { Content } from "@carbon/react";
import DataTable from "@/components/ui/DataTable";
import AddItemModal from "./add_item_modal";
import EditItemModal from "./edit_item_modal";
import DeleteItemModal from "./delete_item_modal";
import {
  Item,
  NewItem,
  ItemCategory,
  Tax,
  ItemTax,
  itemCategoriesTable,
  taxesTable,
} from "@/lib/db/sqlite/schema";
import { useDb } from "@/components/providers/drizzle_provider";
import { money } from "@/lib/util/money";

interface TableRow extends Item {
  priceTransformed: string;
  taxes: ItemTax[];
  taxesTransformed: string;
  category: ItemCategory;
  categoryTransformed: string;
}

const Items = () => {
  const db = useDb()

  // Model States
  const [itemsList, setItemsList] = useState<TableRow[]>([]);
  const [selectedItem, setSelectedItem] = useState<Item | null>(null);
  const [categories, setCategories] = useState<ItemCategory[]>([]);
  const [taxesList, setTaxesList] = useState<Tax[]>([]);
  const [selectedTaxIds, setSelectedTaxIds] = useState<string[]>([]);

  // UI States
  const [loading, setLoading] = useState(true);
  const [currentPage, setCurrentPage] = useState(1);
  const [pageSize, setPageSize] = useState(10);
  const [isAddModalOpen, setIsAddModalOpen] = useState(false);
  const [isEditModalOpen, setIsEditModalOpen] = useState(false);
  const [isDeleteModalOpen, setIsDeleteModalOpen] = useState(false);

  const fetchData = useCallback(async (page: number, size: number) => {
    setLoading(true);
    const offset = (page - 1) * size
    // Fetch items with category and taxes
    const itemsResult = await db.query.itemsTable.findMany({
      with: {
        category: true,
        taxes: {
          with: {
            tax: true,
          },
        },
      },
      limit: size,
      offset: offset
    });

    // Transform the itemsResult to the TableRow type
    const tableRows: TableRow[] = itemsResult.map((item) => ({
      ...item,
      priceTransformed: money(item.price, 'INR').format(),
      categoryTransformed: item.category.name || "Unknown",
      taxesTransformed: item.taxes.map((tax) => tax.tax?.name || "Unknown").join(", "),
    }));

    setItemsList(tableRows);

    // Fetch categories
    const categoriesResult = await db.select().from(itemCategoriesTable);
    setCategories(categoriesResult);

    // Fetch taxes
    const taxesResult = await db.select().from(taxesTable);
    setTaxesList(taxesResult);

    setLoading(false);
  }, [db])

  useEffect(() => {
    fetchData(currentPage, pageSize);
  }, [fetchData, currentPage, pageSize]);

  const headers = [
    { key: "name", header: "Name" },
    { key: "priceTransformed", header: "Price" },
    { key: "categoryTransformed", header: "Category" },
    { key: "taxesTransformed", header: "Taxes" },
  ];

  const handleOpenEditModal = (item: TableRow) => {
    const editItem = {
      id: item.id,
      name: item.name,
      description: item.description,
      price: item.price,
      categoryId: item.categoryId,
    };
    setSelectedItem(editItem as Item);
    setSelectedTaxIds(item.taxes.map((tax) => tax.taxId));
    setIsEditModalOpen(true);
  };

  const handleOpenDeleteModal = (item: TableRow) => {
    const deleteItem = {
      id: item.id,
      name: item.name,
    };
    setSelectedItem(deleteItem as Item);
    setIsDeleteModalOpen(true);
  };

  return (
    <Content className="min-h-[calc(100dvh-3rem)] p-0 flex flex-col">
      <div className="p-4 flex-grow flex flex-col" style={{ height: "calc(100vh - 12rem)" }}>
        <DataTable<TableRow>
          title="Menu Items"
          description="Manage your menu items here. You can add, edit, or delete items as needed."
          headers={headers}
          tableRows={itemsList}
          loading={loading}
          totalItems={itemsList.length}
          currentPage={currentPage}
          pageSize={pageSize}
          pageSizes={[10, 20, 30, 40, 50]}
          onPageChange={(page, pageSize) => {
            setCurrentPage(page);
            setPageSize(pageSize);
            fetchData(page, pageSize);
          }}
          onAddClick={() => setIsAddModalOpen(true)}
          onEditClick={(row) => handleOpenEditModal(itemsList.find(item => item.id === row.id) as TableRow)}
          onDeleteClick={(row) => handleOpenDeleteModal(itemsList.find(item => item.id === row.id) as TableRow)}
        />
      </div>
      <AddItemModal
        open={isAddModalOpen}
        onRequestClose={() => setIsAddModalOpen(false)}
        onRequestSubmit={() => {
          fetchData(currentPage, pageSize);
          setIsAddModalOpen(false);
        }}
        categories={categories}
        taxesList={taxesList}
      />
      <EditItemModal
        open={isEditModalOpen}
        onRequestClose={() => {
          setIsEditModalOpen(false);
          setSelectedItem(null);
        }}
        onRequestSubmit={() => {
          fetchData(currentPage, pageSize);
          setIsEditModalOpen(false);
          setSelectedItem(null);
        }}
        item={selectedItem}
        categories={categories}
        taxesList={taxesList}
        selectedTaxes={selectedTaxIds}
      />
      <DeleteItemModal
        open={isDeleteModalOpen}
        onRequestClose={() => {
          setIsDeleteModalOpen(false);
          setSelectedItem(null);
        }}
        onRequestSubmit={() => {
          fetchData(currentPage, pageSize);
          setIsDeleteModalOpen(false);
          setSelectedItem(null);
        }}
        itemId={selectedItem?.id || ""}
        itemName={selectedItem?.name || ""}
      />
    </Content>
  );
};

export default Items;
