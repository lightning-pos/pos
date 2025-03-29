'use client'
import { useState, useCallback, useEffect } from 'react'
import { Content } from '@carbon/react'
import DataTable from '@/components/ui/DataTable'
import AddPurchaseCategoryModal from './add_category_modal'
import EditPurchaseCategoryModal from './edit_category_modal'
import DeletePurchaseCategoryModal from './delete_category_modal'
import { gql } from '@/lib/graphql/execute'
import {
    GetPurchaseCategoriesDocument,
    CreatePurchaseCategoryDocument,
    UpdatePurchaseCategoryDocument,
    DeletePurchaseCategoryDocument,
    PurchaseCategory
} from '@/lib/graphql/graphql'

// Table row type
interface TableRow extends PurchaseCategory { }

const PurchaseCategories = () => {
    // State declarations
    const [categories, setCategories] = useState<TableRow[]>([])
    const [loading, setLoading] = useState(true)
    const [currentPage, setCurrentPage] = useState(1)
    const [pageSize, setPageSize] = useState(10)
    const [totalCategories, setTotalCategories] = useState(0)

    // Modal States
    const [selectedCategory, setSelectedCategory] = useState<PurchaseCategory | null>(null)
    const [isAddModalOpen, setIsAddModalOpen] = useState(false)
    const [isEditModalOpen, setIsEditModalOpen] = useState(false)
    const [isDeleteModalOpen, setIsDeleteModalOpen] = useState(false)

    // Fetch categories with pagination
    const fetchCategories = useCallback(async (page: number, size: number) => {
        setLoading(true)
        try {
            const offset = (page - 1) * size
            const result = await gql(GetPurchaseCategoriesDocument, { first: size, offset })
            setCategories(result.purchaseCategories)
            // Note: Assuming the backend will eventually provide a totalPurchaseCategories query
            // For now, we'll set a temporary value based on the fetched data
            setTotalCategories(result.purchaseCategories.length >= size ? size * page + 1 : result.purchaseCategories.length + offset)
        } catch (error) {
            console.error('Error fetching purchase categories:', error)
        } finally {
            setLoading(false)
        }
    }, [])

    // Fetch categories on component mount and when pagination changes
    useEffect(() => {
        fetchCategories(currentPage, pageSize)
    }, [fetchCategories, currentPage, pageSize])

    const handleAddCategory = async (name: string, description: string | null) => {
        try {
            await gql(CreatePurchaseCategoryDocument, {
                name,
                description
            })
            setIsAddModalOpen(false)
            fetchCategories(currentPage, pageSize)
        } catch (error) {
            console.error('Error adding purchase category:', error)
        }
    }

    const handleEditCategory = async (update: { name?: string | null, description?: string | null }) => {
        if (!selectedCategory) return

        try {
            await gql(UpdatePurchaseCategoryDocument, {
                id: selectedCategory.id,
                name: update.name || null,
                description: update.description
            })
            setIsEditModalOpen(false)
            setSelectedCategory(null)
            fetchCategories(currentPage, pageSize)
        } catch (error) {
            console.error('Error updating purchase category:', error)
        }
    }

    const handleDeleteCategory = async (id: string) => {
        try {
            await gql(DeletePurchaseCategoryDocument, { id })
            setIsDeleteModalOpen(false)
            setSelectedCategory(null)
            fetchCategories(currentPage, pageSize)
        } catch (error) {
            console.error('Error deleting purchase category:', error)
        }
    }

    const headers = [
        { key: 'name', header: 'Name' },
        { key: 'description', header: 'Description' },
        { key: 'state', header: 'Status' }
    ]

    return (
        <Content className='min-h-[calc(100dvh-3rem)] p-0 flex flex-col'>
            <div className="p-4 flex flex-col gap-4">
                <div className="flex justify-between items-center">
                    <h1 className="text-2xl font-bold">Purchase Categories</h1>
                </div>

                <DataTable<TableRow>
                    title="Purchase Categories"
                    description="Manage your purchase categories here. You can add, edit, or delete purchase categories as needed."
                    headers={headers}
                    tableRows={categories}
                    loading={loading}
                    totalItems={totalCategories}
                    currentPage={currentPage}
                    pageSize={pageSize}
                    pageSizes={[10, 20, 30, 40, 50]}
                    onPageChange={(page, size) => {
                        setCurrentPage(page)
                        setPageSize(size)
                        fetchCategories(page, size)
                    }}
                    onAddClick={() => setIsAddModalOpen(true)}
                    onEditClick={(item) => {
                        setSelectedCategory(item)
                        setIsEditModalOpen(true)
                    }}
                    onDeleteClick={(item) => {
                        setSelectedCategory(item)
                        setIsDeleteModalOpen(true)
                    }}
                />

                <AddPurchaseCategoryModal
                    open={isAddModalOpen}
                    onRequestClose={() => setIsAddModalOpen(false)}
                    onSave={handleAddCategory}
                />

                {selectedCategory && (
                    <EditPurchaseCategoryModal
                        open={isEditModalOpen}
                        onRequestClose={() => {
                            setIsEditModalOpen(false)
                            setSelectedCategory(null)
                        }}
                        onSave={handleEditCategory}
                        category={selectedCategory}
                    />
                )}

                {selectedCategory && (
                    <DeletePurchaseCategoryModal
                        isOpen={isDeleteModalOpen}
                        onClose={() => {
                            setIsDeleteModalOpen(false)
                            setSelectedCategory(null)
                        }}
                        onDelete={() => handleDeleteCategory(selectedCategory.id)}
                        categoryName={selectedCategory.name}
                    />
                )}
            </div>
        </Content>
    )
}

export default PurchaseCategories
