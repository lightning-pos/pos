'use client'
import React, { useState, useEffect, useCallback } from 'react'
import { Content } from '@carbon/react'
import DataTable from '@/components/ui/DataTable'
import AddItemModal from './add_item_modal'
import EditItemModal from './edit_item_modal'
import DeleteItemModal from './delete_item_modal'
import { gql } from '@/lib/graphql/execute'
import {
    GetItemsDocument,
    GetItemCategoriesDocument,
    GetItemTaxesDocument,
    CreateItemDocument,
    UpdateItemDocument,
    DeleteItemDocument,
    Item,
    ItemGroup,
    Tax,
    NewItem,
    UpdateItem,
} from '@/lib/graphql/graphql'
import { formatCurrency } from '@/lib/util/number_format'

interface TableRow extends Item {
    priceTransformed: string
    categoryTransformed: string
    taxesTransformed: string
}

const Items = () => {
    // Model States
    const [itemsList, setItemsList] = useState<TableRow[]>([])
    const [selectedItem, setSelectedItem] = useState<Item | null>(null)
    const [categories, setCategories] = useState<ItemGroup[]>([])
    const [taxes, setTaxes] = useState<Tax[]>([])

    // UI States
    const [loading, setLoading] = useState(false)
    const [currentPage, setCurrentPage] = useState(1)
    const [pageSize, setPageSize] = useState(10)
    const [pageSizes] = useState([10, 20, 30, 40, 50])
    const [isAddModalOpen, setIsAddModalOpen] = useState(false)
    const [isEditModalOpen, setIsEditModalOpen] = useState(false)
    const [isDeleteModalOpen, setIsDeleteModalOpen] = useState(false)

    const fetchData = useCallback(async (page: number, size: number) => {
        setLoading(true)
        try {
            const offset = (page - 1) * size

            const result = await gql(GetItemsDocument, {
                first: size,
                offset: offset
            })

            // Transform the items to the TableRow type
            const tableRows = result.items.map((item) => ({
                ...item,
                priceTransformed: formatCurrency(item.price),
                categoryTransformed: item.category.name || 'Unknown',
                taxesTransformed: item.taxes.map((tax) => tax.name).join(', '),
            }))
            setItemsList(tableRows)

            // Fetch categories
            const categoriesResult = await gql(GetItemCategoriesDocument)
            setCategories(categoriesResult.itemCategories)

            // Fetch taxes
            const taxesResult = await gql(GetItemTaxesDocument)
            setTaxes(taxesResult.taxes)
        } catch (error) {
            console.error('Error fetching data:', error)
        } finally {
            setLoading(false)
        }
    }, [])

    useEffect(() => {
        fetchData(currentPage, pageSize)
    }, [fetchData, currentPage, pageSize])

    const handleAddItem = async (item: NewItem) => {
        try {
            await gql(CreateItemDocument, { input: item })
            await fetchData(currentPage, pageSize)
            setIsAddModalOpen(false)
        } catch (error) {
            console.error('Error adding item:', error)
        }
    }

    const handleEditItem = async (item: UpdateItem) => {
        try {
            await gql(UpdateItemDocument, { input: item })
            await fetchData(currentPage, pageSize)
            setIsEditModalOpen(false)
            setSelectedItem(null)
        } catch (error) {
            console.error('Error updating item:', error)
        }
    }

    const handleDeleteItem = async (id: string) => {
        try {
            await gql(DeleteItemDocument, { id })
            await fetchData(currentPage, pageSize)
            setIsDeleteModalOpen(false)
            setSelectedItem(null)
        } catch (error) {
            console.error('Error deleting item:', error)
        }
    }

    const headers = [
        { key: 'name', header: 'Name' },
        { key: 'categoryTransformed', header: 'Category' },
        { key: 'priceTransformed', header: 'Price' },
        { key: 'taxesTransformed', header: 'Taxes' },
        { key: 'state', header: 'State' },
    ]

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
                    pageSizes={pageSizes}
                    onPageChange={(page, size) => {
                        setCurrentPage(page)
                        setPageSize(size)
                        fetchData(page, size)
                    }}
                    onAddClick={() => setIsAddModalOpen(true)}
                    onEditClick={(row) => {
                        setSelectedItem(row)
                        setIsEditModalOpen(true)
                    }}
                    onDeleteClick={(row) => {
                        setSelectedItem(row)
                        setIsDeleteModalOpen(true)
                    }}
                />
            </div>

            <AddItemModal
                open={isAddModalOpen}
                onRequestClose={() => setIsAddModalOpen(false)}
                categories={categories}
                taxes={taxes}
                onSave={handleAddItem}
            />

            {selectedItem && (
                <EditItemModal
                    open={isEditModalOpen}
                    onRequestClose={() => {
                        setIsEditModalOpen(false)
                        setSelectedItem(null)
                    }}
                    item={selectedItem}
                    categories={categories}
                    taxes={taxes}
                    onSave={handleEditItem}
                />
            )}

            {selectedItem && (
                <DeleteItemModal
                    open={isDeleteModalOpen}
                    onRequestClose={() => {
                        setIsDeleteModalOpen(false)
                        setSelectedItem(null)
                    }}
                    item={selectedItem}
                    onSave={() => handleDeleteItem(selectedItem.id)}
                />
            )}
        </Content>
    )
}

export default Items
