'use client'
import React, { useState } from 'react'
import { useRouter } from 'next/navigation'
import DataTable from '@/components/ui/DataTable'
import AddVariantTypeModal from '@/app/dash/catalog/variants/add_variant_type_modal'
import EditVariantTypeModal from '@/app/dash/catalog/variants/edit_variant_type_modal'
import DeleteVariantTypeModal from '@/app/dash/catalog/variants/delete_variant_type_modal'
import { gql } from '@/lib/graphql/execute'
import {
    CreateVariantTypeDocument,
    UpdateVariantTypeDocument,
    DeleteVariantTypeDocument,
    VariantType,
    VariantTypeNewInput,
    VariantTypeUpdateInput,
} from '@/lib/graphql/graphql'

interface VariantTypesListProps {
    variantTypes: VariantType[]
    loading: boolean
    totalItems: number
    currentPage: number
    pageSize: number
    pageSizes: number[]
    onPageChange: (page: number, size: number) => void
    onRefresh: () => void
}

const VariantTypesList: React.FC<VariantTypesListProps> = ({
    variantTypes,
    loading,
    totalItems,
    currentPage,
    pageSize,
    pageSizes,
    onPageChange,
    onRefresh,
}) => {
    const router = useRouter()
    const [selectedVariantType, setSelectedVariantType] = useState<VariantType | null>(null)
    const [isAddModalOpen, setIsAddModalOpen] = useState(false)
    const [isEditModalOpen, setIsEditModalOpen] = useState(false)
    const [isDeleteModalOpen, setIsDeleteModalOpen] = useState(false)

    const handleAddVariantType = async (input: VariantTypeNewInput) => {
        try {
            await gql(CreateVariantTypeDocument, { input })
            onRefresh()
            setIsAddModalOpen(false)
        } catch (error) {
            console.error('Error adding variant type:', error)
        }
    }

    const handleEditVariantType = async (input: VariantTypeUpdateInput) => {
        try {
            await gql(UpdateVariantTypeDocument, { input })
            onRefresh()
            setIsEditModalOpen(false)
            setSelectedVariantType(null)
        } catch (error) {
            console.error('Error updating variant type:', error)
        }
    }

    const handleDeleteVariantType = async (id: string) => {
        try {
            await gql(DeleteVariantTypeDocument, { id })
            onRefresh()
            setIsDeleteModalOpen(false)
            setSelectedVariantType(null)
        } catch (error) {
            console.error('Error deleting variant type:', error)
        }
    }

    const handleRowClick = (row: VariantType) => {
        router.push(`/dash/catalog/variants/${row.id}`)
    }

    const headers = [
        { key: 'name', header: 'Name' },
        { key: 'description', header: 'Description' },
    ]

    return (
        <>
            <DataTable<VariantType>
                title="Variant Types"
                description="Manage your variant types here. Variant types are categories of variations like Size, Color, etc."
                headers={headers}
                tableRows={variantTypes}
                loading={loading}
                totalItems={totalItems}
                currentPage={currentPage}
                pageSize={pageSize}
                pageSizes={pageSizes}
                onPageChange={onPageChange}
                onAddClick={() => setIsAddModalOpen(true)}
                onEditClick={(row) => {
                    setSelectedVariantType(row)
                    setIsEditModalOpen(true)
                }}
                onDeleteClick={(row) => {
                    setSelectedVariantType(row)
                    setIsDeleteModalOpen(true)
                }}
                onRowClick={handleRowClick}
            />

            <AddVariantTypeModal
                open={isAddModalOpen}
                onRequestClose={() => setIsAddModalOpen(false)}
                onSave={handleAddVariantType}
            />

            {selectedVariantType && (
                <>
                    <EditVariantTypeModal
                        open={isEditModalOpen}
                        onRequestClose={() => {
                            setIsEditModalOpen(false)
                            setSelectedVariantType(null)
                        }}
                        onSave={handleEditVariantType}
                        variantType={selectedVariantType}
                    />

                    <DeleteVariantTypeModal
                        open={isDeleteModalOpen}
                        onRequestClose={() => {
                            setIsDeleteModalOpen(false)
                            setSelectedVariantType(null)
                        }}
                        onDelete={() => handleDeleteVariantType(selectedVariantType.id)}
                        variantType={selectedVariantType}
                    />
                </>
            )}
        </>
    )
}

export default VariantTypesList
