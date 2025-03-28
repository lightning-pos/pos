'use client'
import React, { useState, useEffect, useCallback } from 'react'
import { Content } from '@carbon/react'
import DataTable from '@/components/ui/DataTable'
import DeleteSupplierModal from './delete_supplier_modal'
import AddSupplierModal from './add_supplier_modal'
import EditSupplierModal from './edit_supplier_modal'
import { gql } from '@/lib/graphql/execute'
import { GetSuppliersDocument, CreateSupplierDocument, UpdateSupplierDocument, DeleteSupplierDocument, Supplier } from '@/lib/graphql/graphql'

const SuppliersOverview = () => {
    // State declarations
    const [suppliers, setSuppliers] = useState<Supplier[]>([])
    const [totalSuppliers, setTotalSuppliers] = useState(0)
    const [loading, setLoading] = useState(true)
    const [newSupplier, setNewSupplier] = useState<Partial<Supplier>>({})
    const [editingSupplier, setEditingSupplier] = useState<Supplier | null>(null)
    const [isAddModalOpen, setIsAddModalOpen] = useState(false)
    const [isEditModalOpen, setIsEditModalOpen] = useState(false)
    const [isDeleteModalOpen, setIsDeleteModalOpen] = useState(false)
    const [currentPage, setCurrentPage] = useState(1)
    const [pageSize, setPageSize] = useState(10)

    // Fetch suppliers with server-side pagination
    const fetchSuppliers = useCallback(async (page: number, size: number) => {
        setLoading(true)
        try {
            const offset = (page - 1) * size
            const result = await gql(GetSuppliersDocument, { first: size, offset })
            setSuppliers(result.suppliers)
            setTotalSuppliers(result.totalSuppliers)
        } catch (error) {
            console.error('Error fetching suppliers:', error)
        } finally {
            setLoading(false)
        }
    }, [])

    // Fetch suppliers on component mount and when pagination changes
    useEffect(() => {
        fetchSuppliers(currentPage, pageSize)
    }, [fetchSuppliers, currentPage, pageSize])

    // Add supplier
    const handleAddSupplier = async () => {
        try {
            await gql(CreateSupplierDocument, {
                input: {
                    name: newSupplier.name || '',
                    address: newSupplier.address || null,
                    phone: newSupplier.phone || null
                }
            })
            setIsAddModalOpen(false)
            setNewSupplier({})
            fetchSuppliers(currentPage, pageSize)
        } catch (error) {
            console.error('Error adding supplier:', error)
        }
    }

    // Update supplier
    const handleUpdateSupplier = async () => {
        if (!editingSupplier?.id) return

        try {
            await gql(UpdateSupplierDocument, {
                input: {
                    id: editingSupplier.id,
                    name: editingSupplier.name,
                    address: editingSupplier.address,
                    phone: editingSupplier.phone
                }
            })
            setIsEditModalOpen(false)
            setEditingSupplier(null)
            fetchSuppliers(currentPage, pageSize)
        } catch (error) {
            console.error('Error updating supplier:', error)
        }
    }

    // Delete supplier
    const handleDeleteSupplier = async (id: string) => {
        try {
            await gql(DeleteSupplierDocument, { id })
            setIsDeleteModalOpen(false)
            setEditingSupplier(null)
            fetchSuppliers(currentPage, pageSize)
        } catch (error) {
            console.error('Error deleting supplier:', error)
        }
    }

    const headers = [
        { key: 'name', header: 'Name' },
        { key: 'address', header: 'Address' },
        { key: 'phone', header: 'Phone' }
    ]

    return (
        <Content className='min-h-[calc(100dvh-3rem)] p-0 flex flex-col'>
            <div className="p-4 flex-grow flex flex-col" style={{ minHeight: 'calc(100vh - 12rem)' }}>
                <DataTable<Supplier>
                    title="Supplier Directory"
                    description="Manage your supplier information here. You can add, edit, or delete supplier details as needed."
                    headers={headers}
                    tableRows={suppliers}
                    loading={loading}
                    totalItems={totalSuppliers}
                    currentPage={currentPage}
                    pageSize={pageSize}
                    pageSizes={[10, 20, 30, 40, 50]}
                    onPageChange={(page, pageSize) => {
                        setCurrentPage(page)
                        setPageSize(pageSize)
                        fetchSuppliers(page, pageSize)
                    }}
                    onAddClick={() => {
                        setNewSupplier({})
                        setIsAddModalOpen(true)
                    }}
                    onEditClick={(supplier: Supplier) => {
                        setEditingSupplier(supplier)
                        setIsEditModalOpen(true)
                    }}
                    onDeleteClick={(supplier: Supplier) => {
                        setEditingSupplier(supplier)
                        setIsDeleteModalOpen(true)
                    }}
                />
            </div>

            <AddSupplierModal
                isOpen={isAddModalOpen}
                supplier={newSupplier}
                onClose={() => {
                    setIsAddModalOpen(false)
                    setNewSupplier({})
                }}
                setSupplier={setNewSupplier}
                onSave={handleAddSupplier}
            />

            {editingSupplier && (
                <EditSupplierModal
                    isOpen={isEditModalOpen}
                    supplier={editingSupplier}
                    onClose={() => {
                        setIsEditModalOpen(false)
                        setEditingSupplier(null)
                    }}
                    setSupplier={setEditingSupplier}
                    onSave={handleUpdateSupplier}
                />
            )}

            <DeleteSupplierModal
                isOpen={isDeleteModalOpen}
                supplierId={editingSupplier?.id || ''}
                supplierName={editingSupplier?.name || ''}
                onClose={() => {
                    setIsDeleteModalOpen(false)
                    setEditingSupplier(null)
                }}
                onDelete={() => editingSupplier?.id && handleDeleteSupplier(editingSupplier.id)}
            />
        </Content>
    )
}

export default SuppliersOverview
