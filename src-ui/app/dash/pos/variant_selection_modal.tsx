'use client'
import React, { useState, useEffect } from 'react'
import {
    Modal,
    Tile,
    Button,
    Tag,
    Loading,
    InlineNotification
} from '@carbon/react'
import { formatCurrency } from '@/lib/util/number_format'
import { Item, ItemVariant } from '@/lib/graphql/graphql'

interface VariantSelectionModalProps {
    open: boolean
    onClose: () => void
    item: Item | null
    onSelectVariant: (item: Item, variant: ItemVariant) => void
}

interface GroupedVariantValues {
    [typeId: string]: {
        typeName: string
        values: {
            id: string
            value: string
            variantIds: string[]
        }[]
    }
}

const VariantSelectionModal: React.FC<VariantSelectionModalProps> = ({
    open,
    onClose,
    item,
    onSelectVariant
}) => {
    const [selectedValues, setSelectedValues] = useState<Record<string, string>>({})
    const [matchingVariant, setMatchingVariant] = useState<ItemVariant | null>(null)
    const [groupedVariantValues, setGroupedVariantValues] = useState<GroupedVariantValues>({})
    const [error, setError] = useState<string | null>(null)

    // Group variant values by type
    useEffect(() => {
        if (!item || !item.variants) return

        const grouped: GroupedVariantValues = {}

        // Process all variants to extract and group values by type
        item.variants.forEach(variant => {
            variant.variantValues.forEach(value => {
                const typeId = value.variantType.id
                const typeName = value.variantType.name

                if (!grouped[typeId]) {
                    grouped[typeId] = {
                        typeName,
                        values: []
                    }
                }

                // Check if this value is already in the group
                const existingValue = grouped[typeId].values.find(v => v.value === value.value)
                if (existingValue) {
                    // Add this variant ID to the existing value
                    if (!existingValue.variantIds.includes(variant.id)) {
                        existingValue.variantIds.push(variant.id)
                    }
                } else {
                    // Add new value
                    grouped[typeId].values.push({
                        id: value.id,
                        value: value.value,
                        variantIds: [variant.id]
                    })
                }
            })
        })

        setGroupedVariantValues(grouped)

        // If there's a default variant, pre-select its values
        const defaultVariant = item.variants.find(v => v.isDefault)
        if (defaultVariant) {
            const defaultValues: Record<string, string> = {}
            defaultVariant.variantValues.forEach(value => {
                defaultValues[value.variantType.id] = value.value
            })
            setSelectedValues(defaultValues)
        } else {
            setSelectedValues({})
        }
    }, [item])

    // Find matching variant based on selected values
    useEffect(() => {
        if (!item || !item.variants || Object.keys(selectedValues).length === 0) {
            setMatchingVariant(null)
            return
        }

        // Check if we have selected a value for each variant type
        const variantTypeIds = Object.keys(groupedVariantValues)
        const allTypesSelected = variantTypeIds.every(typeId => selectedValues[typeId])

        if (!allTypesSelected) {
            setMatchingVariant(null)
            return
        }

        // Find the variant that matches all selected values
        const matching = item.variants.find(variant => {
            // For each variant type, check if this variant has the selected value
            return variantTypeIds.every(typeId => {
                const selectedValue = selectedValues[typeId]
                return variant.variantValues.some(value => 
                    value.variantType.id === typeId && value.value === selectedValue
                )
            })
        }) || null

        setMatchingVariant(matching)
    }, [selectedValues, item, groupedVariantValues])

    const handleSelectValue = (typeId: string, value: string) => {
        setSelectedValues(prev => ({
            ...prev,
            [typeId]: value
        }))
    }

    const handleAddToCart = () => {
        if (!item || !matchingVariant) {
            setError('Please select a valid variant combination')
            return
        }

        onSelectVariant(item, matchingVariant)
        onClose()
    }

    if (!item) return null

    return (
        <Modal
            open={open}
            onRequestClose={onClose}
            modalHeading={`Select Variant for ${item.name}`}
            primaryButtonText="Add to Cart"
            secondaryButtonText="Cancel"
            onRequestSubmit={handleAddToCart}
            primaryButtonDisabled={!matchingVariant}
        >
            {error && (
                <InlineNotification
                    kind="error"
                    title="Error"
                    subtitle={error}
                    onCloseButtonClick={() => setError(null)}
                    className="mb-4"
                />
            )}

            <div className="mb-4">
                {Object.entries(groupedVariantValues).map(([typeId, { typeName, values }]) => (
                    <div key={typeId} className="mb-4">
                        <h4 className="mb-2">{typeName}</h4>
                        <div className="flex flex-wrap gap-2">
                            {values.map(value => (
                                <Button
                                    key={value.id}
                                    kind={selectedValues[typeId] === value.value ? 'primary' : 'tertiary'}
                                    size="sm"
                                    onClick={() => handleSelectValue(typeId, value.value)}
                                >
                                    {value.value}
                                </Button>
                            ))}
                        </div>
                    </div>
                ))}
            </div>

            {matchingVariant && (
                <Tile className="mt-4">
                    <div className="flex justify-between items-center">
                        <div>
                            <h4>Selected Variant</h4>
                            <div className="flex flex-wrap gap-2 mt-2">
                                {matchingVariant.variantValues.map(value => (
                                    <Tag key={value.id} type="blue">
                                        {value.variantType.name}: {value.value}
                                    </Tag>
                                ))}
                            </div>
                            {matchingVariant.sku && (
                                <p className="mt-2">SKU: {matchingVariant.sku}</p>
                            )}
                        </div>
                        <div className="text-right">
                            <p className="text-lg font-bold">{formatCurrency(parseFloat(matchingVariant.finalPrice))}</p>
                            {matchingVariant.priceAdjustment && parseFloat(matchingVariant.priceAdjustment) !== 0 && (
                                <p className="text-sm">
                                    {parseFloat(matchingVariant.priceAdjustment) > 0 ? '+' : ''}
                                    {formatCurrency(parseFloat(matchingVariant.priceAdjustment))}
                                </p>
                            )}
                        </div>
                    </div>
                </Tile>
            )}
        </Modal>
    )
}

export default VariantSelectionModal
