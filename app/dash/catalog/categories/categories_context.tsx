'use client'
import React, { createContext, useState, useEffect, useCallback, useContext } from 'react'
import { drizzleDb } from '@/components/providers/system_provider'
import { itemCategoriesTable } from '@/lib/pglite/schema'
import { eq } from 'drizzle-orm'
import { uid } from 'uid'

// Define CategorySchema based on the itemCategories schema
type CategorySchema = typeof itemCategoriesTable.$inferSelect

interface CategoriesContextType {
  categories: CategorySchema[]
  loading: boolean
  editingCategory: CategorySchema | null
  isModalOpen: boolean
  isDeleteModalOpen: boolean
  currentPage: number
  pageSize: number
  fetchCategories: () => Promise<void>
  handleSaveCategory: (e: React.FormEvent) => Promise<void>
  handleDeleteCategory: () => Promise<void>
  setEditingCategory: React.Dispatch<React.SetStateAction<CategorySchema | null>>
  setIsModalOpen: React.Dispatch<React.SetStateAction<boolean>>
  setIsDeleteModalOpen: React.Dispatch<React.SetStateAction<boolean>>
  setCurrentPage: React.Dispatch<React.SetStateAction<number>>
  setPageSize: React.Dispatch<React.SetStateAction<number>>
}

const CategoriesContext = createContext<CategoriesContextType | undefined>(undefined)

export const CategoriesProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [categories, setCategories] = useState<CategorySchema[]>([])
  const [loading, setLoading] = useState(true)
  const [editingCategory, setEditingCategory] = useState<CategorySchema | null>(null)
  const [isModalOpen, setIsModalOpen] = useState(false)
  const [isDeleteModalOpen, setIsDeleteModalOpen] = useState(false)
  const [currentPage, setCurrentPage] = useState(1)
  const [pageSize, setPageSize] = useState(10)

  const fetchCategories = useCallback(async () => {
    setLoading(true)
    try {
      const result = await drizzleDb.select().from(itemCategoriesTable)
      setCategories(result)
    } catch (error) {
      console.error('Error fetching categories:', error)
    } finally {
      setLoading(false)
    }
  }, [])

  useEffect(() => {
    fetchCategories()
  }, [fetchCategories])

  const handleSaveCategory = useCallback(async (e: React.FormEvent) => {
    e.preventDefault()
    if (!editingCategory) return
    try {
      if (editingCategory.id) {
        await drizzleDb.update(itemCategoriesTable)
          .set({
            name: editingCategory.name,
            description: editingCategory.description,
            state: editingCategory.state
          })
          .where(eq(itemCategoriesTable.id, editingCategory.id))
      } else {
        await drizzleDb.insert(itemCategoriesTable).values({
          id: uid(),
          name: editingCategory.name,
          description: editingCategory.description,
          state: editingCategory.state
        })
      }
      setIsModalOpen(false)
      setEditingCategory(null)
      fetchCategories()
    } catch (error) {
      console.error('Error saving category:', error)
    }
  }, [editingCategory, fetchCategories])

  const handleDeleteCategory = useCallback(async () => {
    if (!editingCategory?.id) return
    try {
      await drizzleDb.delete(itemCategoriesTable)
        .where(eq(itemCategoriesTable.id, editingCategory.id))
      setIsDeleteModalOpen(false)
      setEditingCategory(null)
      fetchCategories()
    } catch (error) {
      console.error('Error deleting category:', error)
    }
  }, [editingCategory, fetchCategories])

  return (
    <CategoriesContext.Provider value={{
      categories,
      loading,
      editingCategory,
      isModalOpen,
      isDeleteModalOpen,
      currentPage,
      pageSize,
      fetchCategories,
      handleSaveCategory,
      handleDeleteCategory,
      setEditingCategory,
      setIsModalOpen,
      setIsDeleteModalOpen,
      setCurrentPage,
      setPageSize
    }}>
      {children}
    </CategoriesContext.Provider>
  )
}

export const useCategories = () => {
  const context = useContext(CategoriesContext)
  if (context === undefined) {
    throw new Error('useCategories must be used within a CategoriesProvider')
  }
  return context
}
