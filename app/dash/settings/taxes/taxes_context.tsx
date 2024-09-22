'use client'
import React, { createContext, useState, useEffect, useCallback, useContext } from 'react'
import { drizzleDb } from '@/components/providers/system_provider'
import { taxesTable as taxesTable } from '@/lib/pglite/schema'
import { eq } from 'drizzle-orm'
import { uid } from 'uid'

// Define TaxSchema based on the taxes schema
type TaxSchema = typeof taxesTable.$inferSelect

interface TaxesContextType {
  taxes: TaxSchema[]
  loading: boolean
  editingTax: TaxSchema | null
  isModalOpen: boolean
  isDeleteModalOpen: boolean
  currentPage: number
  pageSize: number
  fetchTaxes: () => Promise<void>
  handleSaveTax: (e: React.FormEvent) => Promise<void>
  handleDeleteTax: () => Promise<void>
  setEditingTax: React.Dispatch<React.SetStateAction<TaxSchema | null>>
  setIsModalOpen: React.Dispatch<React.SetStateAction<boolean>>
  setIsDeleteModalOpen: React.Dispatch<React.SetStateAction<boolean>>
  setCurrentPage: React.Dispatch<React.SetStateAction<number>>
  setPageSize: React.Dispatch<React.SetStateAction<number>>
}

const TaxesContext = createContext<TaxesContextType | undefined>(undefined)

export const TaxesProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [taxes, setTaxes] = useState<TaxSchema[]>([])
  const [loading, setLoading] = useState(true)
  const [editingTax, setEditingTax] = useState<TaxSchema | null>(null)
  const [isModalOpen, setIsModalOpen] = useState(false)
  const [isDeleteModalOpen, setIsDeleteModalOpen] = useState(false)
  const [currentPage, setCurrentPage] = useState(1)
  const [pageSize, setPageSize] = useState(10)

  const fetchTaxes = useCallback(async () => {
    setLoading(true)
    try {
      const result = await drizzleDb.select().from(taxesTable)
      setTaxes(result)
    } catch (error) {
      console.error('Error fetching taxes:', error)
    } finally {
      setLoading(false)
    }
  }, [])

  useEffect(() => {
    fetchTaxes()
  }, [fetchTaxes])

  const handleSaveTax = useCallback(async (e: React.FormEvent) => {
    e.preventDefault()
    if (!editingTax) return
    try {
      if (editingTax.id) {
        await drizzleDb.update(taxesTable)
          .set({
            name: editingTax.name,
            rate: editingTax.rate,
            description: editingTax.description
          })
          .where(eq(taxesTable.id, editingTax.id))
      } else {
        await drizzleDb.insert(taxesTable).values({
          id: uid(),
          name: editingTax.name,
          rate: editingTax.rate,
          description: editingTax.description
        })
      }
      setIsModalOpen(false)
      setEditingTax(null)
      fetchTaxes()
    } catch (error) {
      console.error('Error saving tax:', error)
    }
  }, [editingTax, fetchTaxes])

  const handleDeleteTax = useCallback(async () => {
    if (!editingTax?.id) return
    try {
      await drizzleDb.delete(taxesTable)
        .where(eq(taxesTable.id, editingTax.id))
      setIsDeleteModalOpen(false)
      setEditingTax(null)
      fetchTaxes()
    } catch (error) {
      console.error('Error deleting tax:', error)
    }
  }, [editingTax, fetchTaxes])

  return (
    <TaxesContext.Provider value={{
      taxes,
      loading,
      editingTax,
      isModalOpen,
      isDeleteModalOpen,
      currentPage,
      pageSize,
      fetchTaxes,
      handleSaveTax,
      handleDeleteTax,
      setEditingTax,
      setIsModalOpen,
      setIsDeleteModalOpen,
      setCurrentPage,
      setPageSize
    }}>
      {children}
    </TaxesContext.Provider>
  )
}

export const useTaxes = () => {
  const context = useContext(TaxesContext)
  if (context === undefined) {
    throw new Error('useTaxes must be used within a TaxesProvider')
  }
  return context
}
