'use client'
import React, { useState, useEffect, useCallback } from 'react'
import { Content } from '@carbon/react'
import DataTable from '@/components/ui/DataTable'
import SaveCategoryModal from './save_category_modal'
import DeleteCategoryModal from './delete_category_modal'
import { itemCategoriesTable, ItemCategory, NewItemCategory } from '@/lib/db/sqlite/schema'
import { eq } from 'drizzle-orm'
import { useDb } from '@/components/providers/drizzle_provider'
import { invoke } from '@tauri-apps/api/core'

const Categories = () => {
    const db = useDb()

    const [categories, setCategories] = useState<ItemCategory[]>([])
    const [loading, setLoading] = useState(true)
    const [editingCategory, setEditingCategory] = useState<NewItemCategory | null>(null)
    const [isModalOpen, setIsModalOpen] = useState(false)
    const [isDeleteModalOpen, setIsDeleteModalOpen] = useState(false)
    const [currentPage, setCurrentPage] = useState(1)
    const [pageSize, setPageSize] = useState(10)

    const fetchCategories = useCallback(async (page: number, size: number) => {
        setLoading(false)
        try {
            const offset = (page - 1) * size
            const result: Array<{ itemCategories: ItemCategory[] }> = await invoke('graphql', {
                query:
                    ` query {
                        itemCategories(first: ${size}, offset: ${offset}) {
                            id
                            name
                            description
                            state
                            createdAt
                            updatedAt
                        }
                    }
                ` })
            setCategories(result[0].itemCategories)
        } catch (error) {
            console.error('Error fetching categories:', error)
        }
    }, [db])

    useEffect(() => {
        fetchCategories(currentPage, pageSize)
    }, [fetchCategories, currentPage, pageSize])

    const handleAddCategory = async (e: React.FormEvent) => {
        e.preventDefault()
        if (!editingCategory) return

        await invoke('create_item_category', { name: editingCategory.name, description: editingCategory.description })

        setIsModalOpen(false)
        setEditingCategory(null)
        fetchCategories(currentPage, pageSize)
    }

    const handleEditCategory = async (e: React.FormEvent) => {
        e.preventDefault()
        if (!editingCategory || !editingCategory.id) return
        try {
            await db.update(itemCategoriesTable)
                .set({
                    name: editingCategory.name,
                    description: editingCategory.description,
                    state: editingCategory.state
                })
                .where(eq(itemCategoriesTable.id, editingCategory.id))
            setIsModalOpen(false)
            setEditingCategory(null)
            fetchCategories(currentPage, pageSize)
        } catch (error) {
            console.error('Error editing category:', error)
        }
    }

    const handleDeleteCategory = async () => {
        if (!editingCategory?.id) return
        try {
            await db.delete(itemCategoriesTable)
                .where(eq(itemCategoriesTable.id, editingCategory.id))
            setIsDeleteModalOpen(false)
            setEditingCategory(null)
            fetchCategories(currentPage, pageSize)
        } catch (error) {
            console.error('Error deleting category:', error)
        }
    }

    const headers = [
        { key: 'name', header: 'Name' },
        { key: 'description', header: 'Description' },
        { key: 'state', header: 'State' }
    ]

    const handleOpenAddModal = () => {
        setEditingCategory({} as NewItemCategory)
        setIsModalOpen(true)
    }

    const handleOpenEditModal = (category: NewItemCategory) => {
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
