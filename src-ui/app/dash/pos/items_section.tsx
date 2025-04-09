'use client'
import React, { useState, useEffect } from 'react'
import { AspectRatio, ClickableTile, Column, Grid, Loading, Tag } from '@carbon/react'
import { Corn, Catalog } from '@carbon/icons-react'
import { gql } from '@/lib/graphql/execute'
import {
    GetPosItemsDocument,
    Item,
    ItemNature,
    ItemState,
    ItemGroup,
    Tax,
    ItemVariant
} from '@/lib/graphql/graphql'
import { formatCurrency } from '@/lib/util/number_format'
import VariantSelectionModal from './variant_selection_modal'

interface ItemsSectionProps {
    selectedCategoryId: string | null
    addItemToCart: (item: Item, variant?: ItemVariant) => void
}

const ItemsSection: React.FC<ItemsSectionProps> = ({ selectedCategoryId, addItemToCart }) => {
    const [items, setItems] = useState<Item[]>([])
    const [loading, setLoading] = useState(false)
    const [selectedItem, setSelectedItem] = useState<Item | null>(null)
    const [isVariantModalOpen, setIsVariantModalOpen] = useState(false)

    useEffect(() => {
        const fetchItems = async () => {
            if (selectedCategoryId) {
                setLoading(true)
                try {
                    const result = await gql(GetPosItemsDocument, {
                        first: 100,
                        offset: 0
                    })

                    if (result.items) {
                        // Transform items to concrete types
                        const transformedItems = result.items.map((item: any) => ({
                            id: item.id,
                            name: item.name,
                            description: item.description,
                            nature: item.nature as ItemNature,
                            state: item.state as ItemState,
                            price: item.price,
                            createdAt: item.createdAt,
                            updatedAt: item.updatedAt,
                            hasVariants: item.hasVariants || false,
                            variants: item.variants || [],
                            category: {
                                id: item.category.id,
                                name: item.category.name,
                                description: item.category.description,
                                state: item.category.state as unknown as ItemState,
                                createdAt: item.category.createdAt,
                                updatedAt: item.category.updatedAt
                            } as unknown as ItemGroup,
                            taxes: item.taxes.map((tax: any): Tax => ({
                                id: tax.id,
                                name: tax.name,
                                rate: tax.rate,
                                description: tax.description,
                                createdAt: tax.createdAt,
                                updatedAt: tax.updatedAt
                            }))
                        }))

                        // Filter items by selected category and active state
                        const filteredItems = transformedItems.filter(
                            item => item.category.id === selectedCategoryId && item.state === 'ACTIVE'
                        )
                        setItems(filteredItems)
                    }
                } catch (error) {
                    console.error('Error fetching items:', error)
                } finally {
                    setLoading(false)
                }
            } else {
                setItems([])
            }
        }
        fetchItems()
    }, [selectedCategoryId])

    const handleItemClick = (item: Item) => {
        if (item.hasVariants && item.variants && item.variants.length > 0) {
            setSelectedItem(item)
            setIsVariantModalOpen(true)
        } else {
            // If no variants, add directly to cart
            addItemToCart(item)
        }
    }

    const handleVariantSelected = (item: Item, variant: ItemVariant) => {
        addItemToCart(item, variant)
    }

    if (loading && items.length === 0) {
        return <Loading description="Loading items" withOverlay={false} />
    }

    return (
        <>
            <Grid narrow className='mx-4'>
                {items.map((item) => (
                    <Column key={item.id} lg={2} className='mb-4'>
                        <ClickableTile
                            renderIcon={item.hasVariants ? Catalog : Corn}
                            onClick={() => handleItemClick(item)}
                        >
                            <AspectRatio ratio='3x2'>
                                <div className='flex flex-col justify-between h-full'>
                                    <span>{item.name}</span>
                                    <div className="flex justify-between items-center">
                                        <span>{formatCurrency(item.price)}</span>
                                        {item.hasVariants && (
                                            <Tag type="blue" size="sm">Variants</Tag>
                                        )}
                                    </div>
                                </div>
                            </AspectRatio>
                        </ClickableTile>
                    </Column>
                ))}
            </Grid>

            <VariantSelectionModal
                open={isVariantModalOpen}
                onClose={() => setIsVariantModalOpen(false)}
                item={selectedItem}
                onSelectVariant={handleVariantSelected}
            />
        </>
    )
}

export default ItemsSection
