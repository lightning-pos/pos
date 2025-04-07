'use client'
import React, { useState, useEffect, useCallback } from 'react'
import { useRouter } from 'next/navigation'
import {
    DataTable,
    Table,
    TableHead,
    TableRow,
    TableHeader,
    TableBody,
    TableCell,
    Button,
    Tag,
    Pagination,
    Loading,
    InlineNotification,
} from '@carbon/react'
import { Add, Edit, TrashCan } from '@carbon/icons-react'
import AddItemVariantModal from './add_item_variant_modal'
import EditItemVariantModal from './edit_item_variant_modal'
import DeleteItemVariantModal from './delete_item_variant_modal'
import { gql } from '@/lib/graphql/execute'
import {
    GetItemVariantsDocument,
    GetVariantTypesDocument,
    GetVariantValuesDocument,
    CreateItemVariantDocument,
    UpdateItemVariantDocument,
    DeleteItemVariantDocument,
    AssignVariantValueToItemVariantDocument,
    RemoveVariantValueFromItemVariantDocument,
    ItemVariant,
    VariantType,
    VariantValue,
    ItemVariantNewInput,
    ItemVariantUpdateInput,
} from '@/lib/graphql/graphql'
import { formatCurrency } from '@/lib/util/number_format'

interface ItemVariantsProps {
    itemId: string
}

const ItemVariants: React.FC<ItemVariantsProps> = ({ itemId }) => {
    const router = useRouter()
    const [itemVariants, setItemVariants] = useState<ItemVariant[]>([])
    const [variantTypes, setVariantTypes] = useState<VariantType[]>([])
    const [loading, setLoading] = useState(false)
    const [error, setError] = useState<string | null>(null)
    const [selectedVariant, setSelectedVariant] = useState<ItemVariant | null>(null)
    const [isAddModalOpen, setIsAddModalOpen] = useState(false)
    const [isEditModalOpen, setIsEditModalOpen] = useState(false)
    const [isDeleteModalOpen, setIsDeleteModalOpen] = useState(false)
    const [currentPage, setCurrentPage] = useState(1)
    const [pageSize, setPageSize] = useState(10)

    const fetchItemVariants = useCallback(async () => {
        setLoading(true)
        setError(null)
        try {
            const result = await gql(GetItemVariantsDocument, {
                itemId,
                first: 100, // Get all variants for this item
                offset: 0
            })
            setItemVariants(result.itemVariants as unknown as ItemVariant[])
        } catch (err) {
            console.error('Error fetching item variants:', err)
            setError('Failed to load item variants. Please try again.')
        } finally {
            setLoading(false)
        }
    }, [itemId])

    const fetchVariantTypes = useCallback(async () => {
        try {
            const result = await gql(GetVariantTypesDocument, {
                first: 100, // Get all variant types
                offset: 0
            })
            setVariantTypes(result.variantTypes as unknown as VariantType[])
        } catch (err) {
            console.error('Error fetching variant types:', err)
        }
    }, [])

    useEffect(() => {
        fetchItemVariants()
        fetchVariantTypes()
    }, [fetchItemVariants, fetchVariantTypes])

    const handleAddVariant = async (input: ItemVariantNewInput) => {
        try {
            await gql(CreateItemVariantDocument, { input })
            fetchItemVariants()
            setIsAddModalOpen(false)
        } catch (err) {
            console.error('Error adding item variant:', err)
            setError('Failed to add item variant. Please try again.')
        }
    }

    const handleUpdateVariant = async (input: ItemVariantUpdateInput) => {
        try {
            await gql(UpdateItemVariantDocument, { input })
            fetchItemVariants()
            setIsEditModalOpen(false)
            setSelectedVariant(null)
        } catch (err) {
            console.error('Error updating item variant:', err)
            setError('Failed to update item variant. Please try again.')
        }
    }

    const handleDeleteVariant = async (id: string) => {
        try {
            await gql(DeleteItemVariantDocument, { id })
            fetchItemVariants()
            setIsDeleteModalOpen(false)
            setSelectedVariant(null)
        } catch (err) {
            console.error('Error deleting item variant:', err)
            setError('Failed to delete item variant. Please try again.')
        }
    }

    // Calculate pagination
    const totalItems = itemVariants.length
    const totalPages = Math.ceil(totalItems / pageSize)
    const startIndex = (currentPage - 1) * pageSize
    const endIndex = Math.min(startIndex + pageSize, totalItems)
    const paginatedVariants = itemVariants.slice(startIndex, endIndex)

    if (loading && itemVariants.length === 0) {
        return <Loading description="Loading item variants" withOverlay={false} />
    }

    return (
        <div className="mt-4">
            {error && (
                <InlineNotification
                    kind="error"
                    title="Error"
                    subtitle={error}
                    onCloseButtonClick={() => setError(null)}
                    className="mb-4"
                />
            )}

            <div className="flex justify-between items-center mb-4">
                <h3 className="text-lg font-medium">Item Variants</h3>
                <Button
                    renderIcon={Add}
                    onClick={() => setIsAddModalOpen(true)}
                    disabled={variantTypes.length === 0}
                >
                    Add Variant
                </Button>
            </div>

            {variantTypes.length === 0 && (
                <InlineNotification
                    kind="info"
                    title="No Variant Types"
                    subtitle="You need to create variant types before you can add variants to this item."
                    // @ts-ignore - actionButton is a valid prop but TypeScript doesn't recognize it
                    actionButton={
                        <Button
                            kind="ghost"
                            onClick={() => router.push('/dash/catalog/variants')}
                        >
                            Go to Variant Types
                        </Button>
                    }
                    className="mb-4"
                />
            )}

            {itemVariants.length === 0 ? (
                <p className="text-gray-500">No variants found for this item.</p>
            ) : (
                <>
                    <DataTable rows={paginatedVariants} headers={[
                        { key: 'sku', header: 'SKU' },
                        { key: 'variantValues', header: 'Variant Values' },
                        { key: 'priceAdjustment', header: 'Price Adjustment' },
                        { key: 'finalPrice', header: 'Final Price' },
                        { key: 'isDefault', header: 'Default' },
                        { key: 'actions', header: 'Actions' },
                    ]}>
                        {({ rows, headers, getHeaderProps, getTableProps }) => (
                            <Table {...getTableProps()}>
                                <TableHead>
                                    <TableRow>
                                        {headers.map((header) => (
                                            <TableHeader {...getHeaderProps({ header })}>
                                                {header.header}
                                            </TableHeader>
                                        ))}
                                    </TableRow>
                                </TableHead>
                                <TableBody>
                                    {paginatedVariants.map((variant) => (
                                        <TableRow key={variant.id}>
                                            <TableCell>{variant.sku || '-'}</TableCell>
                                            <TableCell>
                                                <div className="flex flex-wrap gap-2">
                                                    {variant.variantValues.map((value) => (
                                                        <Tag key={value.id} type="blue">
                                                            {value.variantType.name}: {value.value}
                                                        </Tag>
                                                    ))}
                                                </div>
                                            </TableCell>
                                            <TableCell>
                                                {variant.priceAdjustment
                                                    ? formatCurrency(parseFloat(variant.priceAdjustment))
                                                    : '-'}
                                            </TableCell>
                                            <TableCell>
                                                {formatCurrency(parseFloat(variant.finalPrice))}
                                            </TableCell>
                                            <TableCell>
                                                {variant.isDefault && <Tag type="green">Default</Tag>}
                                            </TableCell>
                                            <TableCell>
                                                <div className="flex gap-2">
                                                    <Button
                                                        kind="ghost"
                                                        size="sm"
                                                        renderIcon={Edit}
                                                        iconDescription="Edit"
                                                        hasIconOnly
                                                        onClick={() => {
                                                            setSelectedVariant(variant)
                                                            setIsEditModalOpen(true)
                                                        }}
                                                    />
                                                    <Button
                                                        kind="ghost"
                                                        size="sm"
                                                        renderIcon={TrashCan}
                                                        iconDescription="Delete"
                                                        hasIconOnly
                                                        onClick={() => {
                                                            setSelectedVariant(variant)
                                                            setIsDeleteModalOpen(true)
                                                        }}
                                                    />
                                                </div>
                                            </TableCell>
                                        </TableRow>
                                    ))}
                                </TableBody>
                            </Table>
                        )}
                    </DataTable>

                    {totalPages > 1 && (
                        <Pagination
                            page={currentPage}
                            pageSize={pageSize}
                            pageSizes={[10, 20, 30, 40, 50]}
                            totalItems={totalItems}
                            onChange={({ page, pageSize }) => {
                                setCurrentPage(page)
                                setPageSize(pageSize)
                            }}
                            className="mt-4"
                        />
                    )}
                </>
            )}

            <AddItemVariantModal
                open={isAddModalOpen}
                onRequestClose={() => setIsAddModalOpen(false)}
                onSave={handleAddVariant}
                itemId={itemId}
                variantTypes={variantTypes}
            />

            {selectedVariant && (
                <>
                    <EditItemVariantModal
                        open={isEditModalOpen}
                        onRequestClose={() => {
                            setIsEditModalOpen(false)
                            setSelectedVariant(null)
                        }}
                        onSave={handleUpdateVariant}
                        itemVariant={selectedVariant}
                        variantTypes={variantTypes}
                    />

                    <DeleteItemVariantModal
                        open={isDeleteModalOpen}
                        onRequestClose={() => {
                            setIsDeleteModalOpen(false)
                            setSelectedVariant(null)
                        }}
                        onDelete={() => handleDeleteVariant(selectedVariant.id)}
                        itemVariant={selectedVariant}
                    />
                </>
            )}
        </div>
    )
}

export default ItemVariants
