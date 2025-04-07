'use client'
import React, { useState, useEffect } from 'react'
import { AspectRatio, ClickableTile, Column, Grid } from '@carbon/react'
import { Corn } from '@carbon/icons-react'
import { gql } from '@/lib/graphql/execute'
import {
    GetPosItemsDocument,
    Item,
    ItemNature,
    ItemState,
    ItemGroup,
    Tax
} from '@/lib/graphql/graphql'
import { formatCurrency } from '@/lib/util/number_format'

interface ItemsSectionProps {
    selectedCategoryId: string | null
    addItemToCart: (item: Item) => void
}

const ItemsSection: React.FC<ItemsSectionProps> = ({ selectedCategoryId, addItemToCart }) => {
    const [items, setItems] = useState<Item[]>([])

    useEffect(() => {
        const fetchItems = async () => {
            if (selectedCategoryId) {
                const result = await gql(GetPosItemsDocument, {
                    first: 100,
                    offset: 0
                })

                if (result.items) {
                    // Transform items to concrete types
                    const transformedItems = result.items.map((item) => ({
                        // @ts-ignore - Adding missing properties
                        hasVariants: false,
                        variants: [],
                        id: item.id,
                        name: item.name,
                        description: item.description,
                        nature: item.nature as ItemNature,
                        state: item.state as ItemState,
                        price: item.price,
                        createdAt: item.createdAt,
                        updatedAt: item.updatedAt,
                        category: {
                            id: item.category.id,
                            name: item.category.name,
                            description: item.category.description,
                            state: item.category.state as unknown as ItemState,
                            createdAt: item.category.createdAt,
                            updatedAt: item.category.updatedAt
                        } as unknown as ItemGroup,
                        taxes: item.taxes.map((tax): Tax => ({
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
            } else {
                setItems([])
            }
        }
        fetchItems()
    }, [selectedCategoryId])

    return (
        <Grid narrow className='mx-4'>
            {items.map((item) => (
                <Column key={item.id} lg={2} className='mb-4'>
                    <ClickableTile renderIcon={Corn} onClick={() => addItemToCart(item)}>
                        <AspectRatio ratio='3x2'>
                            <div className='flex flex-col justify-between h-full'>
                                <span>{item.name}</span>
                                <span>{formatCurrency(item.price)}</span>
                            </div>
                        </AspectRatio>
                    </ClickableTile>
                </Column>
            ))}
        </Grid>
    )
}

export default ItemsSection
