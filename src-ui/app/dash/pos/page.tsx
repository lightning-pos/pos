'use client'
import { Column, Content, Grid } from '@carbon/react'
import { useState } from 'react'
import CartSection, { CartItem } from './cart/cart_section'
import CategoriesSection from './categories_section'
import ItemsSection from './items_section'
import { Item, Tax, ItemNature, ItemState } from '@/lib/graphql/graphql'

const POS = () => {
    const [selectedCategoryId, setSelectedCategoryId] = useState<string | null>(null)
    const [cart, setCart] = useState<CartItem[]>([])

    const addToCart = (item: Item) => {
        setCart(prevCart => {
            const existingItem = prevCart.find(cartItem => cartItem.id === item.id)
            const taxIds = item.taxes?.map(tax => tax.id) ?? []

            if (existingItem) {
                return prevCart.map(cartItem =>
                    cartItem.id === item.id
                        ? {
                            ...cartItem,
                            quantity: cartItem.quantity + 1,
                            taxIds
                        }
                        : cartItem
                )
            }

            const newCartItem = {
                // @ts-ignore - Adding missing properties
                hasVariants: false,
                variants: [],
                id: item.id,
                name: item.name,
                description: item.description ?? '',
                price: item.price,
                quantity: 1,
                taxIds,
                nature: item.nature ?? ItemNature.Goods,
                state: item.state ?? ItemState.Active,
                createdAt: item.createdAt ?? new Date().toISOString(),
                updatedAt: item.updatedAt ?? new Date().toISOString()
            }

            return [...prevCart, newCartItem]
        })
    }

    return (
        <Content className='min-h-[calc(100dvh-3rem)] p-0 pt-4'>
            <Grid className='m-auto p-0'>
                <Column lg={2} className='m-0 p-0 max-h-[calc(100dvh-4rem)]'>
                    <CategoriesSection onCategorySelect={setSelectedCategoryId} />
                </Column>
                <Column lg={10} className='m-0 p-0 max-h-[calc(100dvh-4rem)]'>
                    <ItemsSection selectedCategoryId={selectedCategoryId} addItemToCart={addToCart} />
                </Column>
                <Column lg={4} className='m-0 p-0 max-h-[calc(100dvh-4rem)]'>
                    <CartSection cart={cart} setCart={setCart} />
                </Column>
            </Grid>
        </Content>
    )
}

export default POS
