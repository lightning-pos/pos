'use client'
import { useState, useCallback, useEffect } from 'react'
import { Content } from '@carbon/react'
import DataTable from '@/components/ui/DataTable'
import { GetCategoriesDocument, ItemGroup, ItemGroupNew, ItemGroupUpdate, CreateCategoryDocument, UpdateCategoryDocument, DeleteCategoryDocument } from '@/lib/graphql/graphql'
import { gql } from '@/lib/graphql/execute'
import AddCategoryModal from './add_category_modal'
import EditCategoryModal from './edit_category_modal'
import DeleteCategoryModal from './delete_category_modal'

const Categories = () => {
    const [categories, setCategories] = useState<ItemGroup[]>([])
    const [currentPage, setCurrentPage] = useState(1)
    const [pageSize, setPageSize] = useState(10)

    // Modal States
    const [selectedCategory, setSelectedCategory] = useState<ItemGroup | null>(null)
    const [isAddModalOpen, setIsAddModalOpen] = useState(false)
    const [isEditModalOpen, setIsEditModalOpen] = useState(false)
    const [isDeleteModalOpen, setIsDeleteModalOpen] = useState(false)

    const fetchCategories = useCallback(async (page: number, size: number) => {
        try {
            const result = await gql(GetCategoriesDocument, {
                first: size,
                offset: (page - 1) * size
            })
            setCategories(result.itemCategories)
        } catch (error) {
            console.error('Error fetching categories:', error)
        }
    }, [])

    const handleAddCategory = async (category: ItemGroupNew) => {
        try {
            await gql(CreateCategoryDocument, { input: category })
            await fetchCategories(currentPage, pageSize)
            setIsAddModalOpen(false)
        } catch (error) {
            console.error('Error adding category:', error)
        }
    }

    const handleEditCategory = async (category: ItemGroupUpdate) => {
        try {
            await gql(UpdateCategoryDocument, { input: category })
            await fetchCategories(currentPage, pageSize)
            setIsEditModalOpen(false)
            setSelectedCategory(null)
        } catch (error) {
            console.error('Error updating category:', error)
        }
    }

    const handleDeleteCategory = async (id: string) => {
        try {
            await gql(DeleteCategoryDocument, { id })
            await fetchCategories(currentPage, pageSize)
            setIsDeleteModalOpen(false)
            setSelectedCategory(null)
        } catch (error) {
            console.error('Error deleting category:', error)
        }
    }

    useEffect(() => {
        fetchCategories(currentPage, pageSize)
    }, [fetchCategories, currentPage, pageSize])

    const headers = [
        { key: 'name', header: 'Name' },
        { key: 'description', header: 'Description' },
        { key: 'state', header: 'Status' }
    ]

    return (
        <Content className='min-h-[calc(100dvh-3rem)] p-0 flex flex-col'>
            <div className="p-4 flex flex-col gap-4">
                <div className="flex justify-between items-center">
                    <h1 className="text-2xl font-bold">Categories</h1>
                </div>

                <DataTable<ItemGroup>
                    title="Categories"
                    description="Manage your categories here. You can add, edit, or delete categories as needed."
                    headers={headers}
                    tableRows={categories}
                    loading={false}
                    totalItems={categories.length}
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

                <AddCategoryModal
                    open={isAddModalOpen}
                    onRequestClose={() => setIsAddModalOpen(false)}
                    onSave={handleAddCategory}
                />

                {selectedCategory && (
                    <EditCategoryModal
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
                    <DeleteCategoryModal
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

export default Categories
