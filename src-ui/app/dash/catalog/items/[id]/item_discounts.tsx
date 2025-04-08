'use client'
import React, { useState, useEffect, useCallback } from 'react'
import { Button, DataTable, Table, TableHead, TableRow, TableHeader, TableBody, TableCell, Tag, Modal, Loading, InlineNotification } from '@carbon/react'
import { TrashCan, Add } from '@carbon/icons-react'
import { gql } from '@/lib/graphql/execute'
import {
    GetItemDiscountsDocument,
    GetDiscountsDocument,
    AddItemDiscountDocument,
    RemoveItemDiscountDocument,
    Discount,
    DiscountScope,
    DiscountState,
    DiscountType
} from '@/lib/graphql/graphql'
import { formatCurrency } from '@/lib/util/number_format'

interface ItemDiscountsProps {
    itemId: string
    itemName?: string
}

interface ItemDiscount {
    id: string // Required for DataTable
    itemId: string
    discountId: string
    discount?: Discount
}

const ItemDiscounts: React.FC<ItemDiscountsProps> = ({ itemId, itemName }) => {
    const [itemDiscounts, setItemDiscounts] = useState<ItemDiscount[]>([])
    const [availableDiscounts, setAvailableDiscounts] = useState<Discount[]>([])
    const [loading, setLoading] = useState(true)
    const [error, setError] = useState<string | null>(null)
    const [isAddModalOpen, setIsAddModalOpen] = useState(false)
    const [selectedDiscountId, setSelectedDiscountId] = useState<string | null>(null)

    const fetchItemDiscounts = useCallback(async () => {
        try {
            setLoading(true)
            setError(null)

            // Fetch item discounts
            const result = await gql(GetItemDiscountsDocument, { itemId })

            if (result.itemDiscounts) {
                // Fetch all discounts to get full discount details
                const discountsResult = await gql(GetDiscountsDocument, { first: 100, offset: 0 })
                const discountsMap = new Map<string, Discount>()

                discountsResult.discounts.forEach((discount: Discount) => {
                    discountsMap.set(discount.id, discount)
                })

                // Combine item discounts with full discount details
                const enrichedDiscounts = result.itemDiscounts.map((itemDiscount: any, index: number) => ({
                    id: `${itemDiscount.itemId}-${itemDiscount.discountId}`, // Create a unique ID for DataTable
                    itemId: itemDiscount.itemId,
                    discountId: itemDiscount.discountId,
                    discount: discountsMap.get(itemDiscount.discountId)
                }))

                setItemDiscounts(enrichedDiscounts)

                // Filter available discounts (those not already assigned to this item)
                const assignedDiscountIds = new Set(result.itemDiscounts.map((d: any) => d.discountId))
                const filteredDiscounts = discountsResult.discounts.filter(
                    (d: Discount) => !assignedDiscountIds.has(d.id) &&
                        (d.scope === DiscountScope.SpecificItems) &&
                        (d.state === DiscountState.Active)
                )
                setAvailableDiscounts(filteredDiscounts)
            }
        } catch (err) {
            console.error('Error fetching item discounts:', err)
            setError('Failed to load discounts. Please try again.')
        } finally {
            setLoading(false)
        }
    }, [itemId])

    useEffect(() => {
        fetchItemDiscounts()
    }, [fetchItemDiscounts])

    const handleAddDiscount = async () => {
        if (!selectedDiscountId) return

        try {
            setLoading(true)
            await gql(AddItemDiscountDocument, {
                itemDiscount: {
                    itemId,
                    discountId: selectedDiscountId
                }
            })

            setIsAddModalOpen(false)
            setSelectedDiscountId(null)
            await fetchItemDiscounts()
        } catch (err) {
            console.error('Error adding discount to item:', err)
            setError('Failed to add discount. Please try again.')
        } finally {
            setLoading(false)
        }
    }

    const handleRemoveDiscount = async (discountId: string) => {
        try {
            setLoading(true)
            await gql(RemoveItemDiscountDocument, { itemId, discountId })
            await fetchItemDiscounts()
        } catch (err) {
            console.error('Error removing discount from item:', err)
            setError('Failed to remove discount. Please try again.')
        } finally {
            setLoading(false)
        }
    }

    const renderDiscountValue = (discount?: Discount) => {
        if (!discount) return '-'

        if (discount.discountType === DiscountType.Percentage) {
            return `${discount.value}%`
        }
        return formatCurrency(parseFloat(discount.value.toString()))
    }

    const renderDiscountType = (discount?: Discount) => {
        if (!discount) return '-'

        return discount.discountType === DiscountType.Percentage ? 'Percentage' : 'Fixed Amount'
    }

    const renderDiscountState = (discount?: Discount) => {
        if (!discount) return '-'

        const stateColors = {
            [DiscountState.Active]: 'green',
            [DiscountState.Inactive]: 'gray',
            [DiscountState.Scheduled]: 'blue',
            [DiscountState.Expired]: 'red'
        }

        return (
            <Tag type={stateColors[discount.state] as any}>
                {discount.state}
            </Tag>
        )
    }

    return (
        <div className="mt-4">
            <div className="flex justify-between items-center mb-4">
                <h3 className="text-lg font-medium">Applied Discounts</h3>
                <Button
                    kind="primary"
                    size="lg"
                    renderIcon={Add}
                    onClick={() => setIsAddModalOpen(true)}
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

            {error && (
                <InlineNotification
                    kind="error"
                    title="Error"
                    subtitle={error}
                    className="mb-4"
                />
            )}

            {loading ? (
                <Loading description="Loading discounts..." withOverlay={false} />
            ) : itemDiscounts.length === 0 ? (
                <p className="text-gray-500">No discounts applied to this item.</p>
            ) : (
                <table className="w-full border-collapse">
                    <thead>
                        <tr className="bg-gray-100">
                            <th className="p-2 text-left">Name</th>
                            <th className="p-2 text-left">Value</th>
                            <th className="p-2 text-left">Type</th>
                            <th className="p-2 text-left">Status</th>
                            <th className="p-2 text-left">Actions</th>
                        </tr>
                    </thead>
                    <tbody>
                        {itemDiscounts.map(itemDiscount => (
                            <tr key={itemDiscount.id} className="border-b hover:bg-gray-50">
                                <td className="p-2">{itemDiscount.discount?.name || '-'}</td>
                                <td className="p-2">{renderDiscountValue(itemDiscount.discount)}</td>
                                <td className="p-2">{renderDiscountType(itemDiscount.discount)}</td>
                                <td className="p-2">{renderDiscountState(itemDiscount.discount)}</td>
                                <td className="p-2">
                                    <Button
                                        kind="danger"
                                        size="sm"
                                        renderIcon={TrashCan}
                                        iconDescription="Remove discount"
                                        hasIconOnly
                                        tooltipPosition="left"
                                        onClick={() => handleRemoveDiscount(itemDiscount.discountId)}
                                    />
                                </td>
                            </tr>
                        ))}
                    </tbody>
                </table>
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
                onRequestSubmit={handleAddDiscount}
                primaryButtonDisabled={!selectedDiscountId}
            >
                {availableDiscounts.length === 0 ? (
                    <p>No eligible discounts available. Create discounts with "Specific Items" scope first.</p>
                ) : (
                    <div className="mt-4">
                        <table className="w-full border-collapse">
                            <thead>
                                <tr className="bg-gray-100">
                                    <th className="p-2 text-left"></th>
                                    <th className="p-2 text-left">Name</th>
                                    <th className="p-2 text-left">Value</th>
                                    <th className="p-2 text-left">Type</th>
                                </tr>
                            </thead>
                            <tbody>
                                {availableDiscounts.map(discount => (
                                    <tr
                                        key={discount.id}
                                        className={`border-b hover:bg-gray-50 cursor-pointer ${selectedDiscountId === discount.id ? 'bg-blue-50' : ''}`}
                                        onClick={() => setSelectedDiscountId(discount.id)}
                                    >
                                        <td className="p-2">
                                            <input
                                                type="radio"
                                                checked={selectedDiscountId === discount.id}
                                                onChange={() => setSelectedDiscountId(discount.id)}
                                            />
                                        </td>
                                        <td className="p-2">{discount.name}</td>
                                        <td className="p-2">{renderDiscountValue(discount)}</td>
                                        <td className="p-2">{renderDiscountType(discount)}</td>
                                    </tr>
                                ))}
                            </tbody>
                        </table>
                    </div>
                )}
            </Modal>
        </div>
    )
}

export default ItemDiscounts
