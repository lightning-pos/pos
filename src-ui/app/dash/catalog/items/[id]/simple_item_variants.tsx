'use client'
import React, { useState, useEffect, useCallback } from 'react'
import {
    Button,
    DataTable,
    Table,
    TableHead,
    TableRow,
    TableHeader,
    TableBody,
    TableCell,
    Tag,
    Loading,
    InlineNotification,
    Modal,
    TextInput,
    Checkbox,
    Form,
    Stack,
    MultiSelect,
    NumberInput,
} from '@carbon/react'
import { Add, Edit, TrashCan } from '@carbon/icons-react'
import { gql } from '@/lib/graphql/execute'
import { formatCurrency } from '@/lib/util/number_format'
import {
    GetVariantTypesDocument,
    GetItemVariantsDocument,
    CreateItemVariantDocument,
    UpdateItemVariantDocument,
    DeleteItemVariantDocument,
    GetVariantValuesDocument,
    ItemVariantNewInput,
    ItemVariantUpdateInput
} from '@/lib/graphql/graphql'

interface SimpleItemVariantsProps {
    itemId: string
    itemName?: string
}



// Define types that match the structure returned by GraphQL queries
// These are compatible with the generated types but don't include properties not returned by the queries
type VariantTypeData = {
    id: string
    name: string
    description?: string | null
    createdAt: string
    updatedAt: string
}

type VariantValueData = {
    id: string
    value: string
    displayOrder: number
    variantType: {
        id: string
        name: string
    }
}

type ItemVariantData = {
    id: string
    sku?: string | null
    priceAdjustment?: string | null
    isDefault: boolean
    createdAt: string
    updatedAt: string
    finalPrice: string
    variantValues: VariantValueData[]
}

// Define a type for form data
interface VariantFormData {
    sku: string
    priceAdjustment: string
    isDefault: boolean
    variantValueIds: string[]
}

const SimpleItemVariants: React.FC<SimpleItemVariantsProps> = ({ itemId }) => {
    const [loading, setLoading] = useState(false)
    const [error, setError] = useState<string | null>(null)
    const [isAddModalOpen, setIsAddModalOpen] = useState(false)
    const [isEditModalOpen, setIsEditModalOpen] = useState(false)
    const [isDeleteModalOpen, setIsDeleteModalOpen] = useState(false)
    const [variants, setVariants] = useState<ItemVariantData[]>([])
    const [variantTypes, setVariantTypes] = useState<VariantTypeData[]>([])
    const [variantValues, setVariantValues] = useState<VariantValueData[]>([])
    const [selectedVariant, setSelectedVariant] = useState<ItemVariantData | null>(null)
    const [formData, setFormData] = useState<VariantFormData>({
        sku: '',
        priceAdjustment: '',
        isDefault: false,
        variantValueIds: []
    })

    // Fetch variant types
    const fetchVariantTypes = useCallback(async () => {
        try {
            setLoading(true)
            const result = await gql(GetVariantTypesDocument, { first: 100, offset: 0 })
            if (result.variantTypes) {
                setVariantTypes(result.variantTypes)

                // Fetch variant values for each type
                const allValues: VariantValueData[] = []
                for (const type of result.variantTypes) {
                    const valuesResult = await gql(GetVariantValuesDocument, {
                        variantTypeId: type.id,
                        first: 100,
                        offset: 0
                    })
                    if (valuesResult.variantValues) {
                        allValues.push(...valuesResult.variantValues)
                    }
                }
                setVariantValues(allValues)
            }
        } catch (err: any) {
            setError(`Failed to fetch variant types: ${err.message}`)
        } finally {
            setLoading(false)
        }
    }, [])

    // Fetch item variants
    const fetchVariants = useCallback(async () => {
        try {
            setLoading(true)
            const result = await gql(GetItemVariantsDocument, {
                itemId,
                first: 100,
                offset: 0
            })
            if (result.itemVariants) {
                setVariants(result.itemVariants)
            }
        } catch (err: any) {
            setError(`Failed to fetch variants: ${err.message}`)
        } finally {
            setLoading(false)
        }
    }, [itemId])

    // Initial data loading
    useEffect(() => {
        fetchVariantTypes()
        fetchVariants()
    }, [fetchVariantTypes, fetchVariants])

    const handleAddVariant = () => {
        setFormData({
            sku: '',
            priceAdjustment: '',
            isDefault: false,
            variantValueIds: []
        })
        setIsAddModalOpen(true)
    }

    const handleEditVariant = (variant: ItemVariantData) => {
        setSelectedVariant(variant)
        setFormData({
            sku: variant.sku || '',
            priceAdjustment: variant.priceAdjustment || '',
            isDefault: variant.isDefault,
            variantValueIds: variant.variantValues.map(v => v.id)
        })
        setIsEditModalOpen(true)
    }

    const handleDeleteVariant = (variant: ItemVariantData) => {
        setSelectedVariant(variant)
        setIsDeleteModalOpen(true)
    }

    const handleSaveVariant = async () => {
        try {
            setLoading(true)
            const input: ItemVariantNewInput = {
                itemId,
                sku: formData.sku || null,
                priceAdjustment: formData.priceAdjustment ? formData.priceAdjustment : null,
                isDefault: formData.isDefault,
                variantValueIds: formData.variantValueIds
            }

            await gql(CreateItemVariantDocument, { input })
            await fetchVariants()
            setIsAddModalOpen(false)
        } catch (err: any) {
            setError(`Failed to save variant: ${err.message}`)
        } finally {
            setLoading(false)
        }
    }

    const handleUpdateVariant = async () => {
        if (!selectedVariant) return

        try {
            setLoading(true)
            const input: ItemVariantUpdateInput = {
                id: selectedVariant.id,
                sku: formData.sku ? formData.sku : null,
                priceAdjustment: formData.priceAdjustment ? formData.priceAdjustment : null,
                isDefault: formData.isDefault,
                updatedAt: null
            }

            await gql(UpdateItemVariantDocument, { input })
            await fetchVariants()
            setIsEditModalOpen(false)
        } catch (err: any) {
            setError(`Failed to update variant: ${err.message}`)
        } finally {
            setLoading(false)
        }
    }

    const handleConfirmDelete = async () => {
        if (!selectedVariant) return

        try {
            setLoading(true)
            await gql(DeleteItemVariantDocument, { id: selectedVariant.id })
            await fetchVariants()
            setIsDeleteModalOpen(false)
        } catch (err: any) {
            setError(`Failed to delete variant: ${err.message}`)
        } finally {
            setLoading(false)
        }
    }

    if (loading && variants.length === 0) {
        return <Loading description="Loading variants" withOverlay={false} />
    }

    return (
        <div>
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
                <Button
                    renderIcon={Add}
                    onClick={handleAddVariant}
                    disabled={variantTypes.length === 0}
                    size="md"
                    kind="primary"
                >
                    Add Variant
                </Button>
            </div>

            {variantTypes.length === 0 && (
                <InlineNotification
                    kind="info"
                    title="No Variant Types"
                    subtitle="You need to create variant types before you can add variants to this item."
                    className="mb-4"
                />
            )}

            {variants.length === 0 ? (
                <p className="text-gray-500">No variants found for this item.</p>
            ) : (
                <DataTable rows={variants} headers={[
                    { key: 'sku', header: 'SKU' },
                    { key: 'variantValues', header: 'Variant Values' },
                    { key: 'priceAdjustment', header: 'Price Adjustment' },
                    { key: 'finalPrice', header: 'Final Price' },
                    { key: 'isDefault', header: 'Default' },
                    { key: 'actions', header: 'Actions' },
                ]}>
                    {({ headers, getHeaderProps, getTableProps }) => (
                        <Table {...getTableProps()}>
                            <TableHead>
                                <TableRow>
                                    {headers.map((header) => (
                                        <TableHeader {...getHeaderProps({ header })} key={header.key}>
                                            {header.header}
                                        </TableHeader>
                                    ))}
                                </TableRow>
                            </TableHead>
                            <TableBody>
                                {variants.map((variant) => (
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
                                                    onClick={() => handleEditVariant(variant)}
                                                />
                                                <Button
                                                    kind="ghost"
                                                    size="sm"
                                                    renderIcon={TrashCan}
                                                    iconDescription="Delete"
                                                    hasIconOnly
                                                    onClick={() => handleDeleteVariant(variant)}
                                                />
                                            </div>
                                        </TableCell>
                                    </TableRow>
                                ))}
                            </TableBody>
                        </Table>
                    )}
                </DataTable>
            )}

            {/* Add Variant Modal */}
            <Modal
                open={isAddModalOpen}
                modalHeading="Add Item Variant"
                primaryButtonText="Save"
                secondaryButtonText="Cancel"
                onRequestSubmit={handleSaveVariant}
                onRequestClose={() => setIsAddModalOpen(false)}
                primaryButtonDisabled={loading || formData.variantValueIds.length === 0}
            >
                <Form>
                    <Stack gap={5}>
                        <TextInput
                            id="sku"
                            labelText="SKU"
                            placeholder="Optional SKU code"
                            value={formData.sku}
                            onChange={(e) => setFormData({ ...formData, sku: e.target.value })}
                        />

                        <NumberInput
                            id="priceAdjustment"
                            label="Price Adjustment"
                            helperText="Amount to add to or subtract from the base price"
                            allowEmpty
                            value={formData.priceAdjustment}
                            onChange={(_e, { value }) => setFormData({ ...formData, priceAdjustment: value ? value.toString() : '' })}
                        />

                        <Checkbox
                            id="isDefault"
                            labelText="Set as default variant"
                            checked={formData.isDefault}
                            onChange={(_, { checked }) => setFormData({ ...formData, isDefault: checked })}
                        />

                        <MultiSelect
                            id="variantValues"
                            titleText="Variant Values"
                            label="Select variant values"
                            items={variantValues.map(value => ({
                                id: value.id,
                                text: `${value.variantType.name}: ${value.value}`
                            }))}
                            selectedItems={formData.variantValueIds.map(id => ({
                                id,
                                text: variantValues.find(v => v.id === id) ?
                                    `${variantValues.find(v => v.id === id)?.variantType.name}: ${variantValues.find(v => v.id === id)?.value}` :
                                    ''
                            }))}
                            onChange={({ selectedItems }) =>
                                setFormData({ ...formData, variantValueIds: selectedItems ? selectedItems.map(item => item.id) : [] })
                            }
                        />
                    </Stack>
                </Form>
            </Modal>

            {/* Edit Variant Modal */}
            <Modal
                open={isEditModalOpen}
                modalHeading="Edit Item Variant"
                primaryButtonText="Save"
                secondaryButtonText="Cancel"
                onRequestSubmit={handleUpdateVariant}
                onRequestClose={() => setIsEditModalOpen(false)}
                primaryButtonDisabled={loading}
            >
                <Form>
                    <Stack gap={5}>
                        <TextInput
                            id="edit-sku"
                            labelText="SKU"
                            placeholder="Optional SKU code"
                            value={formData.sku}
                            onChange={(e) => setFormData({ ...formData, sku: e.target.value })}
                        />

                        <NumberInput
                            id="edit-priceAdjustment"
                            label="Price Adjustment"
                            helperText="Amount to add to or subtract from the base price"
                            allowEmpty
                            value={formData.priceAdjustment}
                            onChange={(_e, { value }) => setFormData({ ...formData, priceAdjustment: value ? value.toString() : '' })}
                        />

                        <Checkbox
                            id="edit-isDefault"
                            labelText="Set as default variant"
                            checked={formData.isDefault}
                            onChange={(_, { checked }) => setFormData({ ...formData, isDefault: checked })}
                        />
                    </Stack>
                </Form>
            </Modal>

            {/* Delete Variant Modal */}
            <Modal
                open={isDeleteModalOpen}
                modalHeading="Delete Item Variant"
                primaryButtonText="Delete"
                secondaryButtonText="Cancel"
                onRequestSubmit={handleConfirmDelete}
                onRequestClose={() => setIsDeleteModalOpen(false)}
                primaryButtonDisabled={loading}
                danger
            >
                <p>Are you sure you want to delete this variant? This action cannot be undone.</p>
                {selectedVariant && (
                    <div className="mt-4">
                        <p><strong>SKU:</strong> {selectedVariant.sku || 'N/A'}</p>
                        <p><strong>Variant Values:</strong></p>
                        <div className="flex flex-wrap gap-2 mt-2">
                            {selectedVariant.variantValues.map(value => (
                                <Tag key={value.id} type="blue">
                                    {value.variantType.name}: {value.value}
                                </Tag>
                            ))}
                        </div>
                    </div>
                )}
            </Modal>
        </div>
    )
}

export default SimpleItemVariants
