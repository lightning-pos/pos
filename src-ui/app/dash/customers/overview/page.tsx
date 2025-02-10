'use client'
import React, { useState, useEffect, useCallback } from 'react'
import { Content } from '@carbon/react'
import SaveCustomerModal from './save_customer_modal'
import DataTable from '@/components/ui/DataTable'
import DeleteCustomerModal from './delete_customer_modal'
import { invoke } from '@tauri-apps/api/core'

interface Customer {
    id: string
    fullName: string
    email?: string | null
    phone?: string | null
    address?: string | null
    createdAt: string
    updatedAt: string
}

const CustomersOverview = () => {
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
        setLoading(true)
        try {
            const offset = (page - 1) * size
            const result: Array<{ customers: Customer[], totalCustomers: number }> = await invoke('graphql', {
                query: `#graphql
                    query {
                        customers(first: ${size}, offset: ${offset}) {
                            id
                            fullName
                            email
                            phone
                            address
                            createdAt
                            updatedAt
                        }
                        totalCustomers
                    }
                `
            })
            setCustomers(result[0].customers)
            setTotalCustomers(result[0].totalCustomers)
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
        if (!editingCustomer) return

        try {
            const result: Array<{ createCustomer: Customer }> = await invoke('graphql', {
                query: `#graphql
                    mutation {
                        createCustomer(input: {
                            fullName: "${editingCustomer.fullName || ''}"
                            email: ${editingCustomer.email ? `"${editingCustomer.email}"` : 'null'}
                            phone: ${editingCustomer.phone ? `"${editingCustomer.phone}"` : 'null'}
                            address: ${editingCustomer.address ? `"${editingCustomer.address}"` : 'null'}
                        }) {
                            id
                            fullName
                            email
                            phone
                            address
                            createdAt
                            updatedAt
                        }
                    }
                `
            })
            setIsSaveModalOpen(false)
            setEditingCustomer(null)
            fetchCustomers(currentPage, pageSize)
        } catch (error) {
            console.error('Error adding customer:', error)
        }
    }

    // Update customer
    const handleUpdateCustomer = async () => {
        if (!editingCustomer || !editingCustomer.id) return

        try {
            const result: Array<{ updateCustomer: Customer }> = await invoke('graphql', {
                query: `#graphql
                    mutation {
                        updateCustomer(input: {
                            id: "${editingCustomer.id}"
                            fullName: ${editingCustomer.fullName ? `"${editingCustomer.fullName}"` : 'null'}
                            email: ${editingCustomer.email ? `"${editingCustomer.email}"` : 'null'}
                            phone: ${editingCustomer.phone ? `"${editingCustomer.phone}"` : 'null'}
                            address: ${editingCustomer.address ? `"${editingCustomer.address}"` : 'null'}
                        }) {
                            id
                            fullName
                            email
                            phone
                            address
                            createdAt
                            updatedAt
                        }
                    }
                `
            })
            setIsSaveModalOpen(false)
            setEditingCustomer(null)
            fetchCustomers(currentPage, pageSize)
        } catch (error) {
            console.error('Error updating customer:', error)
        }
    }

    // Delete customer
    const handleDeleteCustomer = async (id: string) => {
        try {
            await invoke('graphql', {
                query: `#graphql
                    mutation DeleteCustomer($id: DbUuid!) {
                        deleteCustomer(id: $id)
                    }
                `,
                variables: {
                    id
                }
            })
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
                    setEditingCustomer(null)
                }}
                onDelete={() => editingCustomer?.id && handleDeleteCustomer(editingCustomer.id)}
            />
        </Content>
    )
}

export default CustomersOverview
