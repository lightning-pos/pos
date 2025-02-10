"use client";
import React, { useState, useEffect, useCallback } from "react";
import { Content } from "@carbon/react";
import DataTable from "@/components/ui/DataTable";
import AddItemModal from "./add_item_modal";
import EditItemModal from "./edit_item_modal";
import DeleteItemModal from "./delete_item_modal";
import { invoke } from "@tauri-apps/api/core";

interface Tax {
    id: string
    name: string
    rate: number
    description?: string
}

interface ItemCategory {
    id: string
    name: string
    description?: string
    state: 'ACTIVE' | 'INACTIVE' | 'DELETED'
}

interface Item {
    id: string
    name: string
    description?: string
    price: number
    nature: 'GOODS' | 'SERVICE'
    state: 'ACTIVE' | 'INACTIVE' | 'DELETED'
    categoryId: string
    category: {
        name: string
    }
    taxes: Tax[]
}

interface TableRow extends Item {
    priceTransformed: string;
    categoryTransformed: string;
    taxesTransformed: string;
}

const formatPrice = (price: number): string => {
    return new Intl.NumberFormat('en-IN', {
        style: 'currency',
        currency: 'INR'
    }).format(price);
};

const Items = () => {
    // Model States
    const [itemsList, setItemsList] = useState<TableRow[]>([]);
    const [selectedItem, setSelectedItem] = useState<Item | null>(null);
    const [categories, setCategories] = useState<ItemCategory[]>([]);
    const [taxesList, setTaxesList] = useState<Tax[]>([]);
    const [selectedTaxIds, setSelectedTaxIds] = useState<string[]>([]);

    // UI States
    const [loading, setLoading] = useState(false);
    const [currentPage, setCurrentPage] = useState(1);
    const [pageSize, setPageSize] = useState(10);
    const [isAddModalOpen, setIsAddModalOpen] = useState(false);
    const [isEditModalOpen, setIsEditModalOpen] = useState(false);
    const [isDeleteModalOpen, setIsDeleteModalOpen] = useState(false);

    const fetchData = useCallback(async (page: number, size: number) => {
        setLoading(true);
        try {
            const offset = (page - 1) * size

            const result: Array<{ items: Item[] }> = await invoke('graphql', {
                query: `#graphql
                    query {
                        items(first: ${size}, offset: ${offset}) {
                            id
                            name
                            description
                            price
                            state
                            nature
                            categoryId
                            createdAt
                            updatedAt
                            category {
                                name
                            }
                            taxes {
                                id
                                name
                                rate
                            }
                        }
                    }
                `
            })

            const itemsResult = result[0].items

            // Transform the itemsResult to the TableRow type
            const tableRows = itemsResult.map((item) => ({
                ...item,
                priceTransformed: formatPrice(item.price),
                categoryTransformed: item.category.name || "Unknown",
                taxesTransformed: item.taxes.map((tax) => tax.name).join(", "),
            }));
            setItemsList(tableRows);

            // Fetch categories
            const categoriesResult: Array<{ itemCategories: ItemCategory[] }> = await invoke('graphql', {
                query: `#graphql
                    query {
                        itemCategories {
                            id
                            name
                            description
                            state
                        }
                    }
                `
            })
            setCategories(categoriesResult[0].itemCategories);

            // Fetch taxes
            const taxesResult: Array<{ taxes: Tax[] }> = await invoke('graphql', {
                query: `#graphql
                    query {
                        taxes {
                            id
                            name
                            rate
                            description
                        }
                    }
                `
            })
            setTaxesList(taxesResult[0].taxes);
        } catch (error) {
            console.error('Error fetching data:', error)
        } finally {
            setLoading(false);
        }
    }, [])

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
        setSelectedItem(item);
        setSelectedTaxIds(item.taxes.map(tax => tax.id));
        setIsEditModalOpen(true);
    };

    const handleOpenDeleteModal = (item: TableRow) => {
        setSelectedItem(item);
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
