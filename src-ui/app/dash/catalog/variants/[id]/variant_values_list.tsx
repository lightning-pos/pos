'use client'
import React, { useState } from 'react'
import DataTable from '@/components/ui/DataTable'
import AddVariantValueModal from './add_variant_value_modal'
import EditVariantValueModal from './edit_variant_value_modal'
import DeleteVariantValueModal from './delete_variant_value_modal'
import { gql } from '@/lib/graphql/execute'
import {
    CreateVariantValueDocument,
    UpdateVariantValueDocument,
    DeleteVariantValueDocument,
    VariantValue,
    VariantValueNewInput,
    VariantValueUpdateInput,
} from '@/lib/graphql/graphql'

interface VariantValuesListProps {
    variantTypeId: string
    variantValues: VariantValue[]
    loading: boolean
    totalItems: number
    currentPage: number
    pageSize: number
    pageSizes: number[]
    onPageChange: (page: number, size: number) => void
    onRefresh: () => void
    variantTypeName?: string
}

const VariantValuesList: React.FC<VariantValuesListProps> = ({
    variantTypeId,
    variantValues,
    loading,
    totalItems,
    currentPage,
    pageSize,
    pageSizes,
    onPageChange,
    onRefresh,
    variantTypeName,
}) => {
    const [selectedVariantValue, setSelectedVariantValue] = useState<VariantValue | null>(null)
    const [isAddModalOpen, setIsAddModalOpen] = useState(false)
    const [isEditModalOpen, setIsEditModalOpen] = useState(false)
    const [isDeleteModalOpen, setIsDeleteModalOpen] = useState(false)

    const handleAddVariantValue = async (input: VariantValueNewInput) => {
        try {
            await gql(CreateVariantValueDocument, { input })
            onRefresh()
            setIsAddModalOpen(false)
        } catch (error) {
            console.error('Error adding variant value:', error)
        }
    }

    const handleEditVariantValue = async (input: VariantValueUpdateInput) => {
        try {
            await gql(UpdateVariantValueDocument, { input })
            onRefresh()
            setIsEditModalOpen(false)
            setSelectedVariantValue(null)
        } catch (error) {
            console.error('Error updating variant value:', error)
        }
    }

    const handleDeleteVariantValue = async (id: string) => {
        try {
            await gql(DeleteVariantValueDocument, { id })
            onRefresh()
            setIsDeleteModalOpen(false)
            setSelectedVariantValue(null)
        } catch (error) {
            console.error('Error deleting variant value:', error)
        }
    }

    const headers = [
        { key: 'value', header: 'Value' },
        { key: 'displayOrder', header: 'Display Order' },
    ]

    return (
        <>
            <DataTable<VariantValue>
                title={variantTypeName ? `${variantTypeName} Values` : 'Variant Values'}
                description={variantTypeName ? `Manage values for the ${variantTypeName} variant type` : 'Manage the values for this variant type. For example, if the variant type is \'Size\', values might be \'Small\', \'Medium\', \'Large\', etc.'}
                headers={headers}
                tableRows={variantValues}
                loading={loading}
                totalItems={totalItems}
                currentPage={currentPage}
                pageSize={pageSize}
                pageSizes={pageSizes}
                onPageChange={onPageChange}
                onAddClick={() => setIsAddModalOpen(true)}
                onEditClick={(row) => {
                    setSelectedVariantValue(row)
                    setIsEditModalOpen(true)
                }}
                onDeleteClick={(row) => {
                    setSelectedVariantValue(row)
                    setIsDeleteModalOpen(true)
                }}
            />

            <AddVariantValueModal
                open={isAddModalOpen}
                onRequestClose={() => setIsAddModalOpen(false)}
                onSave={handleAddVariantValue}
                variantTypeId={variantTypeId}
            />

            {selectedVariantValue && (
                <>
                    <EditVariantValueModal
                        open={isEditModalOpen}
                        onRequestClose={() => {
                            setIsEditModalOpen(false)
                            setSelectedVariantValue(null)
                        }}
                        onSave={handleEditVariantValue}
                        variantValue={selectedVariantValue}
                    />

                    <DeleteVariantValueModal
                        open={isDeleteModalOpen}
                        onRequestClose={() => {
                            setIsDeleteModalOpen(false)
                            setSelectedVariantValue(null)
                        }}
                        onDelete={() => handleDeleteVariantValue(selectedVariantValue.id)}
                        variantValue={selectedVariantValue}
                    />
                </>
            )}
        </>
    )
}

export default VariantValuesList
