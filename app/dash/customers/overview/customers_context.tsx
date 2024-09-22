'use client'
import React, { createContext, useState, useEffect, useCallback, useContext } from 'react'
import { drizzleDb } from '@/components/providers/system_provider'
import { customersTable, Customer, NewCustomer } from '@/lib/pglite/schema'
import { uid } from 'uid'
import { eq } from 'drizzle-orm'

// Define the shape of the context
interface CustomersContextType {
  // State
  customers: Customer[]
  loading: boolean
  editingCustomer: Partial<Customer> | null
  isModalOpen: boolean
  isDeleteModalOpen: boolean
  currentPage: number
  pageSize: number

  // Actions
  fetchCustomers: () => Promise<void>
  handleAddOrUpdateCustomer: () => Promise<void>
  handleDeleteCustomer: (id: string) => Promise<void>
  setEditingCustomer: React.Dispatch<React.SetStateAction<Partial<Customer> | null>>
  setIsModalOpen: React.Dispatch<React.SetStateAction<boolean>>
  setIsDeleteModalOpen: React.Dispatch<React.SetStateAction<boolean>>
  setCurrentPage: React.Dispatch<React.SetStateAction<number>>
  setPageSize: React.Dispatch<React.SetStateAction<number>>
}

// Create the context
const CustomersContext = createContext<CustomersContextType | undefined>(undefined)

// CustomersProvider component
export const CustomersProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  // State declarations
  const [customers, setCustomers] = useState<Customer[]>([])
  const [loading, setLoading] = useState(true)
  const [editingCustomer, setEditingCustomer] = useState<Partial<Customer> | null>(null)
  const [isModalOpen, setIsModalOpen] = useState(false)
  const [isDeleteModalOpen, setIsDeleteModalOpen] = useState(false)
  const [currentPage, setCurrentPage] = useState(1)
  const [pageSize, setPageSize] = useState(10)

  // Fetch customers from the database
  const fetchCustomers = useCallback(async () => {
    setLoading(true)
    try {
      const result = await drizzleDb.select().from(customersTable)
      setCustomers(result)
    } catch (error) {
      console.error('Error fetching customers:', error)
    } finally {
      setLoading(false)
    }
  }, [])

  // Fetch customers on component mount
  useEffect(() => {
    fetchCustomers()
  }, [fetchCustomers])

  // Add or update a customer
  const handleAddOrUpdateCustomer = useCallback(async () => {
    if (!editingCustomer) return

    try {
      if (editingCustomer.id) {
        // Update existing customer
        await drizzleDb.update(customersTable)
          .set(editingCustomer as Customer)
          .where(eq(customersTable.id, editingCustomer.id))
        setCustomers(prevCustomers =>
          prevCustomers.map(c => c.id === editingCustomer.id ? { ...c, ...editingCustomer } : c)
        )
      } else {
        // Add new customer
        const newCustomer: NewCustomer = {
          id: uid(),
          name: editingCustomer.name || '',
          email: editingCustomer.email || null,
          phoneNumber: editingCustomer.phoneNumber || null,
          countryCode: editingCustomer.countryCode || null,
        }
        await drizzleDb.insert(customersTable).values([newCustomer])
        setCustomers(prevCustomers => [...prevCustomers, newCustomer as Customer])
      }
      setIsModalOpen(false)
      setEditingCustomer(null)
    } catch (error) {
      console.error('Error saving customer:', error)
    }
  }, [editingCustomer])

  // Delete a customer
  const handleDeleteCustomer = useCallback(async (id: string) => {
    try {
      await drizzleDb.delete(customersTable).where(eq(customersTable.id, id))
      setCustomers(prevCustomers => prevCustomers.filter(c => c.id !== id))
      setIsDeleteModalOpen(false)
      setEditingCustomer(null)
    } catch (error) {
      console.error('Error deleting customer:', error)
    }
  }, [])

  // Context value
  const contextValue: CustomersContextType = {
    // State
    customers,
    loading,
    editingCustomer,
    isModalOpen,
    isDeleteModalOpen,
    currentPage,
    pageSize,

    // Actions
    fetchCustomers,
    handleAddOrUpdateCustomer,
    handleDeleteCustomer,
    setEditingCustomer,
    setIsModalOpen,
    setIsDeleteModalOpen,
    setCurrentPage,
    setPageSize
  }

  return (
    <CustomersContext.Provider value={contextValue}>
      {children}
    </CustomersContext.Provider>
  )
}

// Custom hook to use the customers context
export const useCustomers = () => {
  const context = useContext(CustomersContext)
  if (context === undefined) {
    throw new Error('useCustomers must be used within a CustomersProvider')
  }
  return context
}
