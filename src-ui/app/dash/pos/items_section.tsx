import React, { useState, useEffect } from 'react'
import { AspectRatio, ClickableTile, Column, Grid } from '@carbon/react'
import { Corn } from '@carbon/icons-react'
import { invoke } from '@tauri-apps/api/core'
import { money } from '@/lib/util/money'

interface Tax {
    id: string
    name: string
    rate: number
    description?: string
}

interface Item {
    id: string
    name: string
    description?: string
    nature: 'GOODS' | 'SERVICE'
    state: 'ACTIVE' | 'INACTIVE' | 'DELETED'
    price: number
    category: {
        id: string
        name: string
    }
    taxes: Tax[]
}

interface ItemsSectionProps {
    selectedCategoryId: string | null
    addItemToCart: (item: Item) => void
}

const ItemsSection: React.FC<ItemsSectionProps> = ({ selectedCategoryId, addItemToCart }) => {
    const [items, setItems] = useState<Array<Item>>([])

    useEffect(() => {
        const fetchItems = async () => {
            if (selectedCategoryId) {
                try {
                    const result: Array<{ items: Item[] }> = await invoke('graphql', {
                        query: `#graphql
                            query GetItems {
                                items(first: 100) {
                                    id
                                    name
                                    description
                                    nature
                                    state
                                    price
                                    category {
                                        id
                                        name
                                    }
                                    taxes {
                                        id
                                        name
                                        rate
                                        description
                                    }
                                }
                            }
                        `,
                    })

                    if (result[0]?.items) {
                        const filteredItems = result[0].items.filter(
                            item => item.category.id === selectedCategoryId && item.state === 'ACTIVE'
                        )
                        setItems(filteredItems)
                    }
                } catch (error) {
                    console.error('Error fetching items:', error)
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
                                <span>{money(item.price, 'INR').format()}</span>
                            </div>
                        </AspectRatio>
                    </ClickableTile>
                </Column>
            ))}
        </Grid>
    )
}

export default ItemsSection
