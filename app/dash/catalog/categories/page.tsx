'use client'
import React from 'react'
import { Content } from '@carbon/react'
import { CategoriesProvider, useCategories } from './categories_context'
import DataTable from '@/components/ui/DataTable'
import SaveCategoryModal from './save_category_modal'
import DeleteCategoryModal from './delete_category_modal'
import { itemCategories } from '@/lib/pglite/schema'

type CategorySchema = typeof itemCategories.$inferSelect

const CategoriesContent = () => {
  const {
    categories,
    loading,
    currentPage,
    pageSize,
    setCurrentPage,
    setPageSize,
    setEditingCategory,
    setIsModalOpen,
    setIsDeleteModalOpen
  } = useCategories()

  const headers = [
    { key: 'name', header: 'Name' },
    { key: 'description', header: 'Description' },
    { key: 'state', header: 'State' }
  ]

  const handleAddCategory = () => {
    setEditingCategory({ id: '', name: '', description: '', state: 'active' } as CategorySchema)
    setIsModalOpen(true)
  }

  const handleEditCategory = (category: CategorySchema) => {
    setEditingCategory(category)
    setIsModalOpen(true)
  }

  const handleDeleteCategory = (category: CategorySchema) => {
    setEditingCategory(category)
    setIsDeleteModalOpen(true)
  }

  return (
    <Content className='min-h-[calc(100dvh-3rem)] p-0 flex flex-col'>
      <div className="p-4 flex-grow flex flex-col" style={{ height: 'calc(100vh - 12rem)' }}>
        <DataTable<CategorySchema>
          title="Category"
          description="Manage your categories here. You can add, edit, or delete categories as needed."
          headers={headers}
          rows={categories}
          loading={loading}
          totalItems={categories.length}
          currentPage={currentPage}
          pageSize={pageSize}
          pageSizes={[10, 20, 30, 40, 50]}
          onPageChange={(page, pageSize) => {
            setCurrentPage(page)
            setPageSize(pageSize)
          }}
          onAddClick={handleAddCategory}
          onEditClick={handleEditCategory}
          onDeleteClick={handleDeleteCategory}
        />
      </div>
      <SaveCategoryModal />
      <DeleteCategoryModal />
    </Content>
  )
}

const Categories = () => {
  return (
    <CategoriesProvider>
      <CategoriesContent />
    </CategoriesProvider>
  )
}

export default Categories
