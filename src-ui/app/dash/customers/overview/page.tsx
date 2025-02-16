'use client'
import React, { useState, useEffect, useCallback } from 'react'
import { Content } from '@carbon/react'
import DataTable from '@/components/ui/DataTable'
import DeleteCustomerModal from './delete_customer_modal'
import AddCustomerModal from './add_customer_modal'
import EditCustomerModal from './edit_customer_modal'
import { gql } from '@/lib/graphql/execute'
import { GetCustomersDocument, CreateCustomerDocument, UpdateCustomerDocument, DeleteCustomerDocument, Customer } from '@/lib/graphql/graphql'

const CustomersOverview = () => {
    // State declarations
    const [customers, setCustomers] = useState<Customer[]>([])
    const [totalCustomers, setTotalCustomers] = useState(0)
    const [loading, setLoading] = useState(true)
    const [newCustomer, setNewCustomer] = useState<Partial<Customer>>({})
    const [editingCustomer, setEditingCustomer] = useState<Customer | null>(null)
    const [isAddModalOpen, setIsAddModalOpen] = useState(false)
    const [isEditModalOpen, setIsEditModalOpen] = useState(false)
    const [isDeleteModalOpen, setIsDeleteModalOpen] = useState(false)
    const [currentPage, setCurrentPage] = useState(1)
    const [pageSize, setPageSize] = useState(10)

    // Updated fetchCustomers function with server-side pagination
    const fetchCustomers = useCallback(async (page: number, size: number) => {
        setLoading(true)
        try {
            const offset = (page - 1) * size
            const result = await gql(GetCustomersDocument, { first: size, offset })
            setCustomers(result.customers)
            setTotalCustomers(result.totalCustomers)
        } catch (error) {
            console.error('Error fetching customers:', error)
        } finally {
            setLoading(false)
        }
    }, [])

    // Fetch customers on component mount and when pagination changes
    useEffect(() => {
        fetchCustomers(currentPage, pageSize)
    }, [fetchCustomers, currentPage, pageSize])

    // Add customer
    const handleAddCustomer = async () => {
        try {
            await gql(CreateCustomerDocument, {
                input: {
                    fullName: newCustomer.fullName || '',
                    email: newCustomer.email || null,
                    phone: newCustomer.phone || null,
                    address: newCustomer.address || null
                }
            })
            setIsAddModalOpen(false)
            setNewCustomer({})
            fetchCustomers(currentPage, pageSize)
        } catch (error) {
            console.error('Error adding customer:', error)
        }
    }

    // Update customer
    const handleUpdateCustomer = async () => {
        if (!editingCustomer?.id) return

        try {
            await gql(UpdateCustomerDocument, {
                input: {
                    id: editingCustomer.id,
                    fullName: editingCustomer.fullName,
                    email: editingCustomer.email,
                    phone: editingCustomer.phone,
                    address: editingCustomer.address
                }
            })
            setIsEditModalOpen(false)
            setEditingCustomer(null)
            fetchCustomers(currentPage, pageSize)
        } catch (error) {
            console.error('Error updating customer:', error)
        }
    }

    // Delete customer
    const handleDeleteCustomer = async (id: string) => {
        try {
            await gql(DeleteCustomerDocument, { id })
            setIsDeleteModalOpen(false)
            setEditingCustomer(null)
            fetchCustomers(currentPage, pageSize)
        } catch (error) {
            console.error('Error deleting customer:', error)
        }
    }

    const headers = [
        { key: 'fullName', header: 'Name' },
        { key: 'email', header: 'Email' },
        { key: 'phone', header: 'Phone' },
        { key: 'address', header: 'Address' }
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
                        setNewCustomer({})
                        setIsAddModalOpen(true)
                    }}
                    onEditClick={(customer: Customer) => {
                        setEditingCustomer(customer)
                        setIsEditModalOpen(true)
                    }}
                    onDeleteClick={(customer: Customer) => {
                        setEditingCustomer(customer)
                        setIsDeleteModalOpen(true)
                    }}
                />
            </div>

            <AddCustomerModal
                isOpen={isAddModalOpen}
                customer={newCustomer}
                onClose={() => {
                    setIsAddModalOpen(false)
                    setNewCustomer({})
                }}
                setCustomer={setNewCustomer}
                onSave={handleAddCustomer}
            />

            {editingCustomer && (
                <EditCustomerModal
                    isOpen={isEditModalOpen}
                    customer={editingCustomer}
                    onClose={() => {
                        setIsEditModalOpen(false)
                        setEditingCustomer(null)
                    }}
                    setCustomer={setEditingCustomer}
                    onSave={handleUpdateCustomer}
                />
            )}

            <DeleteCustomerModal
                isOpen={isDeleteModalOpen}
                customerId={editingCustomer?.id || ''}
                customerName={editingCustomer?.fullName || ''}
                onClose={() => {
                    setIsDeleteModalOpen(false)
                    setEditingCustomer(null)
                }}
                onDelete={() => editingCustomer?.id && handleDeleteCustomer(editingCustomer.id)}
            />
        </Content>
    )
}

export default CustomersOverview
