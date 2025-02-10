'use client'
import React, { useState, useEffect, useCallback } from 'react'
import { Content } from '@carbon/react'
import DataTable from '@/components/ui/DataTable'
import SaveCategoryModal from './save_category_modal'
import DeleteCategoryModal from './delete_category_modal'
import { invoke } from '@tauri-apps/api/core'

interface ItemCategory {
    id: string
    name: string
    description?: string
    state: 'ACTIVE' | 'INACTIVE' | 'DELETED'
    createdAt: string
    updatedAt: string
}

interface NewItemCategory {
    id?: string
    name: string
    description?: string
    state?: 'ACTIVE' | 'INACTIVE' | 'DELETED'
}

const Categories = () => {
    const [categories, setCategories] = useState<ItemCategory[]>([])
    const [loading, setLoading] = useState(true)
    const [editingCategory, setEditingCategory] = useState<NewItemCategory | null>(null)
    const [isModalOpen, setIsModalOpen] = useState(false)
    const [isDeleteModalOpen, setIsDeleteModalOpen] = useState(false)
    const [currentPage, setCurrentPage] = useState(1)
    const [pageSize, setPageSize] = useState(10)

    const fetchCategories = useCallback(async (page: number, size: number) => {
        setLoading(true)
        try {
            const offset = (page - 1) * size
            const result: Array<{ itemCategories: ItemCategory[] }> = await invoke('graphql', {
                query: `#graphql
                    query {
                        itemCategories(first: ${size}, offset: ${offset}) {
                            id name description state createdAt updatedAt
                        }
                    }
                `,
            })
            setCategories(result[0].itemCategories)
        } catch (error) {
            console.error('Error fetching categories:', error)
        } finally {
            setLoading(false)
        }
    }, [])

    useEffect(() => {
        fetchCategories(currentPage, pageSize)
    }, [fetchCategories, currentPage, pageSize])

    const handleAddCategory = async (e: React.FormEvent) => {
        e.preventDefault()
        if (!editingCategory) return

        try {
            await invoke('graphql', {
                query: `#graphql
                    mutation {
                        createItemCategory(
                            newCategory: { name: "${editingCategory.name}", description: "${editingCategory.description || ''}" }
                        ) {
                            id name description state createdAt updatedAt
                        }
                    }
                `,
            })

            setIsModalOpen(false)
            setEditingCategory(null)
            fetchCategories(currentPage, pageSize)
        } catch (error) {
            console.error('Error creating category:', error)
            alert('Failed to create category')
        }
    }

    const handleEditCategory = async (e: React.FormEvent) => {
        e.preventDefault()
        if (!editingCategory || !editingCategory.id) return

        try {
            await invoke('graphql', {
                query: `#graphql
                    mutation {
                        updateItemCategory(
                            category: {
                                id: "${editingCategory.id}",
                                name: "${editingCategory.name}",
                                description: "${editingCategory.description || ''}",
                                state: ${editingCategory.state || 'ACTIVE'}
                            }
                        ) {
                            id name description state createdAt updatedAt
                        }
                    }
                `,
            })

            setIsModalOpen(false)
            setEditingCategory(null)
            fetchCategories(currentPage, pageSize)
        } catch (error) {
            console.error('Error updating category:', error)
            alert('Failed to update category')
        }
    }

    const handleDeleteCategory = async () => {
        if (!editingCategory?.id) return

        try {
            await invoke('graphql', {
                query: `#graphql
                    mutation {
                        deleteItemCategory(id: "${editingCategory.id}")
                    }
                `,
            })

            setIsDeleteModalOpen(false)
            setEditingCategory(null)
            fetchCategories(currentPage, pageSize)
        } catch (error) {
            console.error('Error deleting category:', error)
            alert('Failed to delete category')
        }
    }

    const headers = [
        { key: 'name', header: 'Name' },
        { key: 'description', header: 'Description' },
        { key: 'state', header: 'State' }
    ]

    const handleOpenAddModal = () => {
        setEditingCategory({ name: '', description: '', state: 'ACTIVE' })
        setIsModalOpen(true)
    }

    const handleOpenEditModal = (category: ItemCategory) => {
        setEditingCategory(category)
        setIsModalOpen(true)
    }

    const handleOpenDeleteModal = (category: ItemCategory) => {
        setEditingCategory(category)
        setIsDeleteModalOpen(true)
    }

    return (
        <Content className='min-h-[calc(100dvh-3rem)] p-0 flex flex-col'>
            <div className="p-4 flex-grow flex flex-col" style={{ height: 'calc(100vh - 12rem)' }}>
                <DataTable<ItemCategory>
                    title="Category"
                    description="Manage your categories here. You can add, edit, or delete categories as needed."
                    headers={headers}
                    tableRows={categories}
                    loading={loading}
                    totalItems={categories.length}
                    currentPage={currentPage}
                    pageSize={pageSize}
                    pageSizes={[10, 20, 30, 40, 50]}
                    onPageChange={(page, pageSize) => {
                        setCurrentPage(page)
                        setPageSize(pageSize)
                        fetchCategories(page, pageSize)
                    }}
                    onAddClick={handleOpenAddModal}
                    onEditClick={handleOpenEditModal}
                    onDeleteClick={handleOpenDeleteModal}
                />
            </div>
            <SaveCategoryModal
                isOpen={isModalOpen}
                onClose={() => setIsModalOpen(false)}
                onSave={editingCategory?.id ? handleEditCategory : handleAddCategory}
                category={editingCategory}
                setCategory={setEditingCategory}
            />
            <DeleteCategoryModal
                isOpen={isDeleteModalOpen}
                onClose={() => setIsDeleteModalOpen(false)}
                onDelete={handleDeleteCategory}
                categoryName={editingCategory?.name || ''}
            />
        </Content>
    )
}

export default Categories
