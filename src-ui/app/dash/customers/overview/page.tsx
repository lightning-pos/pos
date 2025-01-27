'use client'
import React, { useState, useEffect, useCallback } from 'react'
import { Content } from '@carbon/react'
import { Customer, NewCustomer, customersTable } from '@/lib/db/sqlite/schema'
import { useDb } from '@/components/providers/drizzle_provider'
import { eq, sql } from 'drizzle-orm'
import { uid } from 'uid'
import SaveCustomerModal from './save_customer_modal'
import DataTable from '@/components/ui/DataTable'
import DeleteCustomerModal from './delete_customer_modal'

const CustomersOverview = () => {
  const db = useDb()
  // State declarations
  const [customers, setCustomers] = useState<Customer[]>([])
  const [totalCustomers, setTotalCustomers] = useState(0)
  const [loading, setLoading] = useState(true)
  const [editingCustomer, setEditingCustomer] = useState<Partial<Customer> | null>(null)
  const [isSaveModalOpen, setIsSaveModalOpen] = useState(false)
  const [isDeleteModalOpen, setIsDeleteModalOpen] = useState(false)
  const [currentPage, setCurrentPage] = useState(1)
  const [pageSize, setPageSize] = useState(10)

  // Updated fetchCustomers function with server-side pagination
  const fetchCustomers = useCallback(async (page: number, size: number) => {
    try {
      const offset = (page - 1) * size
      const [paginatedResult, [{ count }]] = await Promise.all([
        db.select()
          .from(customersTable)
          .limit(size)
          .offset(offset),
        db.select({ count: sql<number>`count(*)` }).from(customersTable)
      ])
      setCustomers(paginatedResult)
      setTotalCustomers(count)
    } catch (error) {
      console.error('Error fetching customers:', error)
    } finally {
      setLoading(false)
    }
  }, [db])

  // Fetch customers on component mount and when pagination changes
  useEffect(() => {
    fetchCustomers(currentPage, pageSize)
  }, [fetchCustomers, currentPage, pageSize])

  // Add customer
  const handleAddCustomer = async () => {
    if (!editingCustomer) return

    try {
      const newCustomer: NewCustomer = {
        id: uid(),
        name: editingCustomer.name || '',
        email: editingCustomer.email || null,
        phoneNumber: editingCustomer.phoneNumber || null,
        countryCode: editingCustomer.countryCode || null,
      }
      await db.insert(customersTable).values([newCustomer])
      setIsSaveModalOpen(false)
      setEditingCustomer(null)
      fetchCustomers(currentPage, pageSize) // Refresh the customer list
    } catch (error) {
      console.error('Error adding customer:', error)
    }
  }

  // Update customer
  const handleUpdateCustomer = async () => {
    if (!editingCustomer || !editingCustomer.id) return

    try {
      await db.update(customersTable)
        .set(editingCustomer as Customer)
        .where(eq(customersTable.id, editingCustomer.id))
      setIsSaveModalOpen(false)
      setEditingCustomer(null)
      fetchCustomers(currentPage, pageSize) // Refresh the customer list
    } catch (error) {
      console.error('Error updating customer:', error)
    }
  }

  // Delete customer
  const handleDeleteCustomer = async (id: string) => {
    try {
      await db.delete(customersTable).where(eq(customersTable.id, id))
      setIsDeleteModalOpen(false)
      setEditingCustomer(null)
      fetchCustomers(currentPage, pageSize) // Refresh the customer list
    } catch (error) {
      console.error('Error deleting customer:', error)
    }
  }

  const headers = [
    { key: 'name', header: 'Name' },
    { key: 'email', header: 'Email' },
    { key: 'phoneNumber', header: 'Phone Number' },
  ]

  return (
    <Content className='min-h-[calc(100dvh-3rem)] p-0 flex flex-col'>
      <div className="p-4 flex-grow flex flex-col" style={{ minHeight: 'calc(100vh - 12rem)' }}>
        <DataTable<Customer>
          title="Customers"
          description="Manage your customers here. You can add, edit, or delete customers as needed."
          headers={headers}
          tableRows={customers}
          loading={loading}
          totalItems={totalCustomers}
          currentPage={currentPage}
          pageSize={pageSize}
          pageSizes={[10, 20, 30, 40, 50]}
          onPageChange={(page, pageSize) => {
            setCurrentPage(page)
            setPageSize(pageSize)
            fetchCustomers(page, pageSize)
          }}
          onAddClick={() => {
            setEditingCustomer({})
            setIsSaveModalOpen(true)
          }}
          onEditClick={(customer: Customer) => {
            setEditingCustomer(customer)
            setIsSaveModalOpen(true)
          }}
          onDeleteClick={(customer: Customer) => {
            setEditingCustomer(customer)
            setIsDeleteModalOpen(true)
          }}
        />
      </div>
      <SaveCustomerModal
        isOpen={isSaveModalOpen}
        editingCustomer={editingCustomer}
        onClose={() => {
          setIsSaveModalOpen(false)
          setEditingCustomer({})
        }}
        setEditingCustomer={setEditingCustomer}
        onSave={editingCustomer?.id ? handleUpdateCustomer : handleAddCustomer}
      />
      <DeleteCustomerModal
        isOpen={isDeleteModalOpen}
        editingCustomer={editingCustomer}
        onClose={() => {
          setIsDeleteModalOpen(false)
          setEditingCustomer({})
        }}
        onDelete={() => editingCustomer?.id && handleDeleteCustomer(editingCustomer.id)}
      />
    </Content>
  )
}

export default CustomersOverview
