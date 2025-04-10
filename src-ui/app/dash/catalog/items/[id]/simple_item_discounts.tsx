'use client'
import React, { useState, useEffect, useCallback } from 'react'
import {
    Button,
    Table,
    TableHead,
    TableRow,
    TableHeader,
    TableBody,
    TableCell,
    Tag,
    Modal,
    Loading,
    InlineNotification
} from '@carbon/react'
import { TrashCan, Add } from '@carbon/icons-react'
import { gql } from '@/lib/graphql/execute'
import {
    GetItemDiscountsDocument,
    GetDiscountsDocument,
    AddItemDiscountDocument,
    RemoveItemDiscountDocument,
    DiscountState,
    DiscountScope,
    DiscountType,
    Discount,
    ItemDiscountObject
} from '@/lib/graphql/graphql'
import { formatCurrency } from '@/lib/util/number_format'

interface SimpleItemDiscountsProps {
    itemId: string
    itemName?: string
}

// Define a type that combines ItemDiscountObject with Discount for easier rendering
type EnrichedItemDiscount = {
    id: string // Unique ID for React keys
    itemDiscountObject: ItemDiscountObject
    discount?: Discount
}

const SimpleItemDiscounts: React.FC<SimpleItemDiscountsProps> = ({ itemId, itemName }) => {
    const [itemDiscounts, setItemDiscounts] = useState<EnrichedItemDiscount[]>([])
    const [availableDiscounts, setAvailableDiscounts] = useState<Discount[]>([])
    const [loading, setLoading] = useState(true)
    const [error, setError] = useState<string | null>(null)
    const [isAddModalOpen, setIsAddModalOpen] = useState(false)
    const [selectedDiscountId, setSelectedDiscountId] = useState<string | null>(null)

    // Fetch item discounts and available discounts
    const fetchDiscounts = useCallback(async () => {
        try {
            setLoading(true)
            setError(null)

            // Fetch item discounts
            const result = await gql(GetItemDiscountsDocument, { itemId })

            // Fetch all discounts to get full discount details
            const discountsResult = await gql(GetDiscountsDocument, {
                first: 100,
                offset: 0,
                state: DiscountState.Active
            })

            if (discountsResult.discounts) {
                // Create a map for quick lookup
                const discountsMap = new Map<string, Discount>()

                // Store discounts in a map for quick lookup
                discountsResult.discounts.forEach(discount => {
                    discountsMap.set(discount.id, discount)
                })

                // Filter available discounts to only those with SPECIFIC_ITEMS scope
                // and not already applied to this item
                const appliedDiscountIds = new Set(
                    result.itemDiscounts ? result.itemDiscounts.map((d: any) => d.discountId) : []
                )

                // Filter available discounts to only those with SPECIFIC_ITEMS scope
                // and not already applied to this item
                const availableDiscountsList = discountsResult.discounts.filter(discount =>
                    discount.scope === DiscountScope.SpecificItems && !appliedDiscountIds.has(discount.id)
                )
                setAvailableDiscounts(availableDiscountsList)

                // Combine item discounts with full discount details
                if (result.itemDiscounts) {
                    const enrichedDiscounts: EnrichedItemDiscount[] = result.itemDiscounts.map((itemDiscountObj: ItemDiscountObject) => ({
                        id: `${itemDiscountObj.itemId}-${itemDiscountObj.discountId}`, // Create a unique ID
                        itemDiscountObject: itemDiscountObj,
                        discount: discountsMap.get(itemDiscountObj.discountId)
                    }))
                    setItemDiscounts(enrichedDiscounts)
                }
            }
        } catch (err: any) {
            setError(`Failed to fetch discounts: ${err.message}`)
        } finally {
            setLoading(false)
        }
    }, [itemId])

    // Initial data loading
    useEffect(() => {
        fetchDiscounts()
    }, [fetchDiscounts])

    const handleAddDiscount = () => {
        setIsAddModalOpen(true)
    }

    const handleSaveDiscount = async () => {
        if (!selectedDiscountId) return

        try {
            setLoading(true)
            await gql(AddItemDiscountDocument, {
                itemDiscount: {
                    itemId,
                    discountId: selectedDiscountId
                }
            })
            await fetchDiscounts()
            setIsAddModalOpen(false)
            setSelectedDiscountId(null)
        } catch (err: any) {
            setError(`Failed to add discount: ${err.message}`)
        } finally {
            setLoading(false)
        }
    }

    const handleRemoveDiscount = async (discountId: string) => {
        try {
            setLoading(true)
            await gql(RemoveItemDiscountDocument, { itemId, discountId })
            await fetchDiscounts()
        } catch (err: any) {
            setError(`Failed to remove discount: ${err.message}`)
        } finally {
            setLoading(false)
        }
    }

    const renderDiscountValue = (discount?: Discount) => {
        if (!discount) return '-'

        if (discount.discountType === DiscountType.Percentage) {
            return `${discount.value}%`
        }
        return formatCurrency(parseFloat(discount.value))
    }

    const renderDiscountType = (discount?: Discount) => {
        if (!discount) return '-'

        return discount.discountType === DiscountType.Percentage ? 'Percentage' : 'Fixed Amount'
    }

    const renderDiscountState = (discount?: Discount) => {
        if (!discount) return '-'

        // Map state to Carbon tag types
        let tagType: 'green' | 'gray' | 'blue' | 'red' = 'gray'

        switch (discount.state) {
        case DiscountState.Active:
            tagType = 'green'
            break
        case DiscountState.Inactive:
            tagType = 'gray'
            break
        case DiscountState.Scheduled:
            tagType = 'blue'
            break
        case DiscountState.Expired:
            tagType = 'red'
            break
        }

        return (
            <Tag type={tagType}>
                {discount.state}
            </Tag>
        )
    }

    if (loading && itemDiscounts.length === 0) {
        return <Loading description="Loading discounts" withOverlay={false} />
    }

    return (
        <div>
            {error && (
                <InlineNotification
                    kind="error"
                    title="Error"
                    subtitle={error}
                    className="mb-4"
                />
            )}

            <div className="flex justify-between items-center mb-4">
                <Button
                    kind="primary"
                    size="md"
                    renderIcon={Add}
                    onClick={handleAddDiscount}
                    disabled={availableDiscounts.length === 0}
                >
                    Add Discount
                </Button>
            </div>

            {availableDiscounts.length === 0 && (
                <div className="bg-blue-50 p-4 rounded mb-4">
                    <p className="text-blue-800 font-medium">No eligible discounts available</p>
                    <p className="text-blue-600">You need to create discounts with "Specific Items" scope first.</p>
                    <Button
                        kind="tertiary"
                        className="mt-2"
                        onClick={() => window.open('/dash/catalog/discounts', '_blank')}
                    >
                        Go to Discounts Page
                    </Button>
                </div>
            )}

            {itemDiscounts.length === 0 ? (
                <p className="text-gray-500">No discounts applied to this item.</p>
            ) : (
                <Table>
                    <TableHead>
                        <TableRow>
                            <TableHeader>Name</TableHeader>
                            <TableHeader>Value</TableHeader>
                            <TableHeader>Type</TableHeader>
                            <TableHeader>Status</TableHeader>
                            <TableHeader>Actions</TableHeader>
                        </TableRow>
                    </TableHead>
                    <TableBody>
                        {itemDiscounts.map(itemDiscount => (
                            <TableRow key={itemDiscount.id}>
                                <TableCell>{itemDiscount.discount?.name || '-'}</TableCell>
                                <TableCell>{renderDiscountValue(itemDiscount.discount)}</TableCell>
                                <TableCell>{renderDiscountType(itemDiscount.discount)}</TableCell>
                                <TableCell>{renderDiscountState(itemDiscount.discount)}</TableCell>
                                <TableCell>
                                    <Button
                                        kind="danger"
                                        size="sm"
                                        renderIcon={TrashCan}
                                        iconDescription="Remove discount"
                                        hasIconOnly
                                        tooltipPosition="left"
                                        onClick={() => handleRemoveDiscount(itemDiscount.itemDiscountObject.discountId)}
                                    />
                                </TableCell>
                            </TableRow>
                        ))}
                    </TableBody>
                </Table>
            )}

            {/* Add Discount Modal */}
            <Modal
                open={isAddModalOpen}
                onRequestClose={() => {
                    setIsAddModalOpen(false)
                    setSelectedDiscountId(null)
                }}
                modalHeading={`Add Discount to ${itemName || 'Item'}`}
                primaryButtonText="Add"
                secondaryButtonText="Cancel"
                onRequestSubmit={handleSaveDiscount}
                primaryButtonDisabled={!selectedDiscountId || loading}
            >
                {loading ? (
                    <Loading description="Loading discounts..." withOverlay={false} />
                ) : availableDiscounts.length === 0 ? (
                    <p>No eligible discounts available. Create discounts with "Specific Items" scope first.</p>
                ) : (
                    <div className="mt-4">
                        <Table>
                            <TableHead>
                                <TableRow>
                                    <TableHeader></TableHeader>
                                    <TableHeader>Name</TableHeader>
                                    <TableHeader>Value</TableHeader>
                                    <TableHeader>Type</TableHeader>
                                </TableRow>
                            </TableHead>
                            <TableBody>
                                {availableDiscounts.map(discount => (
                                    <TableRow
                                        key={discount.id}
                                        className={selectedDiscountId === discount.id ? 'bg-blue-50' : ''}
                                        onClick={() => setSelectedDiscountId(discount.id)}
                                    >
                                        <TableCell>
                                            <input
                                                type="radio"
                                                checked={selectedDiscountId === discount.id}
                                                onChange={() => setSelectedDiscountId(discount.id)}
                                            />
                                        </TableCell>
                                        <TableCell>{discount.name}</TableCell>
                                        <TableCell>{renderDiscountValue(discount)}</TableCell>
                                        <TableCell>{renderDiscountType(discount)}</TableCell>
                                    </TableRow>
                                ))}
                            </TableBody>
                        </Table>
                    </div>
                )}
            </Modal>
        </div>
    )
}

export default SimpleItemDiscounts
