'use client'
import React, { useState, useEffect, useCallback } from 'react'
import {
    Modal,
    TextInput,
    Checkbox,
    Form,
    Stack,
    MultiSelect,
    NumberInput,
    Tag,
    Button,
} from '@carbon/react'
import { Add, TrashCan } from '@carbon/icons-react'
import { gql } from '@/lib/graphql/execute'
import {
    GetVariantValuesDocument,
    AssignVariantValueToItemVariantDocument,
    RemoveVariantValueFromItemVariantDocument,
    VariantType,
    VariantValue,
    ItemVariant,
    ItemVariantUpdateInput,
} from '@/lib/graphql/graphql'
import { sanitizeDecimalInput } from '@/lib/util/number_format'

interface EditItemVariantModalProps {
    open: boolean
    onRequestClose: () => void
    onSave: (input: ItemVariantUpdateInput) => void
    itemVariant: ItemVariant
    variantTypes: VariantType[]
}

interface VariantValuesByType {
    [typeId: string]: VariantValue[]
}

const EditItemVariantModal: React.FC<EditItemVariantModalProps> = ({
    open,
    onRequestClose,
    onSave,
    itemVariant,
    variantTypes,
}) => {
    const [sku, setSku] = useState('')
    const [priceAdjustment, setPriceAdjustment] = useState('')
    const [isDefault, setIsDefault] = useState(false)
    const [variantValuesByType, setVariantValuesByType] = useState<VariantValuesByType>({})
    const [loading, setLoading] = useState(false)

    const fetchVariantValues = useCallback(async () => {
        setLoading(true)
        try {
            const valuesByType: VariantValuesByType = {}

            for (const variantType of variantTypes) {
                const result = await gql(GetVariantValuesDocument, {
                    variantTypeId: variantType.id,
                    first: 100,
                    offset: 0
                })
                valuesByType[variantType.id] = result.variantValues as unknown as VariantValue[]
            }

            setVariantValuesByType(valuesByType)
        } catch (error) {
            console.error('Error fetching variant values:', error)
        } finally {
            setLoading(false)
        }
    }, [variantTypes])

    useEffect(() => {
        if (open) {
            fetchVariantValues()
            setSku(itemVariant.sku || '')
            setPriceAdjustment(itemVariant.priceAdjustment || '')
            setIsDefault(itemVariant.isDefault)
        }
    }, [open, itemVariant, fetchVariantValues])

    const handlePriceAdjustmentChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const sanitizedValue = sanitizeDecimalInput(e.target.value)
        setPriceAdjustment(sanitizedValue)
    }

    const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault()

        const input: ItemVariantUpdateInput = {
            id: itemVariant.id,
            sku: sku !== itemVariant.sku ? (sku.trim() || null) : undefined,
            priceAdjustment: priceAdjustment !== itemVariant.priceAdjustment ? (priceAdjustment || null) : undefined,
            isDefault: isDefault !== itemVariant.isDefault ? isDefault : undefined,
        }

        onSave(input)
    }

    const handleAssignVariantValue = async (variantValueId: string) => {
        try {
            await gql(AssignVariantValueToItemVariantDocument, {
                itemVariantId: itemVariant.id,
                variantValueId
            })
            // Refresh the parent component
            onRequestClose()
        } catch (error) {
            console.error('Error assigning variant value:', error)
        }
    }

    const handleRemoveVariantValue = async (variantValueId: string) => {
        try {
            await gql(RemoveVariantValueFromItemVariantDocument, {
                itemVariantId: itemVariant.id,
                variantValueId
            })
            // Refresh the parent component
            onRequestClose()
        } catch (error) {
            console.error('Error removing variant value:', error)
        }
    }

    // Group variant values by type
    const groupedVariantValues: { [typeId: string]: VariantValue[] } = {}
    itemVariant.variantValues.forEach(value => {
        const typeId = value.variantType.id
        if (!groupedVariantValues[typeId]) {
            groupedVariantValues[typeId] = []
        }
        groupedVariantValues[typeId].push(value)
    })

    // Get available variant values that are not already assigned
    const getAvailableVariantValues = (typeId: string) => {
        const allValues = variantValuesByType[typeId] || []
        const assignedValueIds = new Set(
            itemVariant.variantValues
                .filter(v => v.variantType.id === typeId)
                .map(v => v.id)
        )
        return allValues.filter(v => !assignedValueIds.has(v.id))
    }

    return (
        <Modal
            open={open}
            modalHeading="Edit Item Variant"
            primaryButtonText="Save"
            secondaryButtonText="Cancel"
            onRequestSubmit={handleSubmit}
            onRequestClose={onRequestClose}
        >
            <Form onSubmit={handleSubmit}>
                <Stack gap={5}>
                    <TextInput
                        id="edit-sku"
                        labelText="SKU"
                        placeholder="Optional SKU code"
                        value={sku}
                        onChange={(e) => setSku(e.target.value)}
                    />

                    <NumberInput
                        id="edit-priceAdjustment"
                        label="Price Adjustment"
                        helperText="Amount to add to or subtract from the base price"
                        value={priceAdjustment}
                        onChange={(e) => setPriceAdjustment(e.currentTarget.value)}
                        allowEmpty
                    />

                    <Checkbox
                        id="edit-isDefault"
                        labelText="Set as default variant"
                        checked={isDefault}
                        onChange={(_, { checked }) => setIsDefault(checked)}
                    />

                    <div>
                        <h4 className="mb-2">Variant Values</h4>

                        {variantTypes.map(type => (
                            <div key={type.id} className="mb-4">
                                <h5 className="text-sm font-medium mb-2">{type.name}</h5>

                                <div className="flex flex-wrap gap-2 mb-2">
                                    {(groupedVariantValues[type.id] || []).map(value => (
                                        <Tag
                                            key={value.id}
                                            type="blue"
                                            filter
                                            onClose={() => handleRemoveVariantValue(value.id)}
                                        >
                                            {value.value}
                                        </Tag>
                                    ))}
                                </div>

                                <div className="mt-2">
                                    <MultiSelect
                                        id={`add-value-${type.id}`}
                                        titleText={`Add ${type.name} Value`}
                                        label="Select value to add"
                                        items={getAvailableVariantValues(type.id).map(v => ({
                                            id: v.id,
                                            label: v.value
                                        }))}
                                        itemToString={(item) => item?.label || ''}
                                        onChange={({ selectedItems }) => {
                                            if (selectedItems && selectedItems.length > 0) {
                                                handleAssignVariantValue(selectedItems![0].id)
                                            }
                                        }}
                                        selectionFeedback="top-after-reopen"
                                    />
                                </div>
                            </div>
                        ))}
                    </div>
                </Stack>
            </Form>
        </Modal>
    )
}

export default EditItemVariantModal
