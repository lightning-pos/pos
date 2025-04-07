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
} from '@carbon/react'
import { gql } from '@/lib/graphql/execute'
import {
    GetVariantValuesDocument,
    VariantType,
    VariantValue,
    ItemVariantNewInput,
} from '@/lib/graphql/graphql'
import { sanitizeDecimalInput } from '@/lib/util/number_format'

interface AddItemVariantModalProps {
    open: boolean
    onRequestClose: () => void
    onSave: (input: ItemVariantNewInput) => void
    itemId: string
    variantTypes: VariantType[]
}

interface VariantValuesByType {
    [typeId: string]: VariantValue[]
}

const AddItemVariantModal: React.FC<AddItemVariantModalProps> = ({
    open,
    onRequestClose,
    onSave,
    itemId,
    variantTypes,
}) => {
    const [sku, setSku] = useState('')
    const [priceAdjustment, setPriceAdjustment] = useState('')
    const [isDefault, setIsDefault] = useState(false)
    const [selectedValues, setSelectedValues] = useState<string[]>([])
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
        }
    }, [open, fetchVariantValues])

    const handlePriceAdjustmentChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const sanitizedValue = sanitizeDecimalInput(e.target.value)
        setPriceAdjustment(sanitizedValue)
    }

    const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault()

        const input: ItemVariantNewInput = {
            itemId,
            sku: sku.trim() || null,
            priceAdjustment: priceAdjustment ? priceAdjustment : null,
            isDefault,
            variantValueIds: selectedValues,
        }

        onSave(input)
        resetForm()
    }

    const resetForm = () => {
        setSku('')
        setPriceAdjustment('')
        setIsDefault(false)
        setSelectedValues([])
    }

    const handleClose = () => {
        resetForm()
        onRequestClose()
    }

    // Prepare items for MultiSelect
    const multiSelectItems = variantTypes.map(type => {
        const values = variantValuesByType[type.id] || []
        return {
            id: type.id,
            label: type.name,
            items: values.map(value => ({
                id: value.id,
                label: value.value,
                groupId: type.id
            }))
        }
    })

    return (
        <Modal
            open={open}
            modalHeading="Add Item Variant"
            primaryButtonText="Save"
            secondaryButtonText="Cancel"
            onRequestSubmit={handleSubmit}
            onRequestClose={handleClose}
        >
            <Form onSubmit={handleSubmit}>
                <Stack gap={5}>
                    <TextInput
                        id="sku"
                        labelText="SKU"
                        placeholder="Optional SKU code"
                        value={sku}
                        onChange={(e) => setSku(e.target.value)}
                    />

                    <NumberInput
                        id="priceAdjustment"
                        label="Price Adjustment"
                        helperText="Amount to add to or subtract from the base price"
                        value={priceAdjustment}
                        onChange={(e) => setPriceAdjustment(e.currentTarget.value)}
                        allowEmpty
                    />

                    <Checkbox
                        id="isDefault"
                        labelText="Set as default variant"
                        checked={isDefault}
                        onChange={(_, { checked }) => setIsDefault(checked)}
                    />

                    <MultiSelect
                        id="variantValues"
                        titleText="Variant Values"
                        label="Select variant values"
                        items={multiSelectItems.flatMap(group => group.items)}
                        itemToString={(item) => {
                            if (!item) return ''
                            const group = multiSelectItems.find(g => g.id === item.groupId)
                            return `${group?.label}: ${item.label}`
                        }}
                        onChange={({ selectedItems }) => {
                            setSelectedValues(selectedItems?.map(item => item.id) || [])
                        }}
                        selectionFeedback="top-after-reopen"
                    />
                </Stack>
            </Form>
        </Modal>
    )
}

export default AddItemVariantModal
