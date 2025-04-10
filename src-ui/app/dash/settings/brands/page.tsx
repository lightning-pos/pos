'use client'
import React, { useState, useEffect, useCallback } from 'react'
import { Content } from '@carbon/react'
import DataTable from '@/components/ui/DataTable'
import AddBrandModal from './add_brand_modal'
import EditBrandModal from './edit_brand_modal'
import DeleteBrandModal from './delete_brand_modal'
import { gql } from '@/lib/graphql/execute'
import { GetBrandsDocument, CreateBrandDocument, UpdateBrandDocument, DeleteBrandDocument, Brand } from '@/lib/graphql/graphql'

// Define the table row structure
interface TableRow extends Brand {
    status: string
}

const BrandsPage = () => {
    // State declarations
    const [brands, setBrands] = useState<TableRow[]>([])
    const [totalBrands, setTotalBrands] = useState(0)
    const [loading, setLoading] = useState(true)
    const [newBrand, setNewBrand] = useState<Partial<Brand>>({})
    const [editingBrand, setEditingBrand] = useState<Brand | null>(null)
    const [isAddModalOpen, setIsAddModalOpen] = useState(false)
    const [isEditModalOpen, setIsEditModalOpen] = useState(false)
    const [isDeleteModalOpen, setIsDeleteModalOpen] = useState(false)
    const [currentPage, setCurrentPage] = useState(1)
    const [pageSize, setPageSize] = useState(10)

    // Format brand data for table
    const formatBrandData = (brands: Brand[]): TableRow[] => {
        return brands.map(brand => ({
            ...brand,
            status: brand.isActive ? 'Active' : 'Inactive'
        }))
    }

    // Fetch brands
    const fetchBrands = useCallback(async () => {
        setLoading(true)
        try {
            const result = await gql(GetBrandsDocument)
            if (result.brands) {
                setBrands(formatBrandData(result.brands as Brand[]))
                setTotalBrands(result.brands.length)
            }
        } catch (error) {
            console.error('Error fetching brands:', error)
        } finally {
            setLoading(false)
        }
    }, [])

    // Fetch brands on component mount
    useEffect(() => {
        fetchBrands()
    }, [fetchBrands])

    // Create brand
    const handleCreateBrand = async () => {
        try {
            await gql(CreateBrandDocument, {
                input: {
                    name: newBrand.name || '',
                    description: newBrand.description,
                    isActive: newBrand.isActive ?? true
                }
            })
            setIsAddModalOpen(false)
            setNewBrand({})
            fetchBrands()
        } catch (error) {
            console.error('Error creating brand:', error)
        }
    }

    // Update brand
    const handleUpdateBrand = async () => {
        if (!editingBrand?.id) return

        try {
            await gql(UpdateBrandDocument, {
                input: {
                    id: editingBrand.id,
                    name: editingBrand.name,
                    description: editingBrand.description,
                    isActive: editingBrand.isActive
                }
            })
            setIsEditModalOpen(false)
            setEditingBrand(null)
            fetchBrands()
        } catch (error) {
            console.error('Error updating brand:', error)
        }
    }

    // Delete brand
    const handleDeleteBrand = async (id: string) => {
        try {
            await gql(DeleteBrandDocument, { id })
            setIsDeleteModalOpen(false)
            setEditingBrand(null)
            fetchBrands()
        } catch (error) {
            console.error('Error deleting brand:', error)
        }
    }

    const headers = [
        { key: 'name', header: 'Name' },
        { key: 'description', header: 'Description' },
        { key: 'status', header: 'Status' }
    ]

    return (
        <Content className='min-h-[calc(100dvh-3rem)] p-0 flex flex-col'>
            <div className="p-4 flex-grow flex flex-col" style={{ minHeight: 'calc(100vh - 12rem)' }}>
                <DataTable<TableRow>
                    title="Brands"
                    description="Manage product brands for your inventory."
                    headers={headers}
                    tableRows={brands}
                    loading={loading}
                    totalItems={totalBrands}
                    currentPage={currentPage}
                    pageSize={pageSize}
                    pageSizes={[10, 20, 30, 40, 50]}
                    onPageChange={(page, size) => {
                        setCurrentPage(page)
                        setPageSize(size)
                    }}
                    onAddClick={() => {
                        setNewBrand({
                            isActive: true
                        })
                        setIsAddModalOpen(true)
                    }}
                    onEditClick={(brand: TableRow) => {
                        setEditingBrand(brand)
                        setIsEditModalOpen(true)
                    }}
                    onDeleteClick={(brand: TableRow) => {
                        setEditingBrand(brand)
                        setIsDeleteModalOpen(true)
                    }}
                />
            </div>

            <AddBrandModal
                isOpen={isAddModalOpen}
                brand={newBrand}
                onClose={() => {
                    setIsAddModalOpen(false)
                    setNewBrand({})
                }}
                setBrand={setNewBrand}
                onSave={handleCreateBrand}
            />

            {editingBrand && (
                <EditBrandModal
                    isOpen={isEditModalOpen}
                    brand={editingBrand as any}
                    onClose={() => {
                        setIsEditModalOpen(false)
                        setEditingBrand(null)
                    }}
                    setBrand={setEditingBrand as any}
                    onSave={handleUpdateBrand}
                />
            )}

            <DeleteBrandModal
                isOpen={isDeleteModalOpen}
                brandId={editingBrand?.id || ''}
                brandName={editingBrand?.name || ''}
                onClose={() => {
                    setIsDeleteModalOpen(false)
                    setEditingBrand(null)
                }}
                onDelete={() => editingBrand?.id && handleDeleteBrand(editingBrand.id)}
            />
        </Content>
    )
}

export default BrandsPage
