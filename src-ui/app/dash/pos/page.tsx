'use client'
import { Column, Content, Grid } from '@carbon/react'
import { useState } from 'react'
import CartSection, { CartItem } from './cart/cart_section'
import CategoriesSection from './categories_section'
import ItemsSection from './items_section'
import { Item, ItemVariant, ItemNature, ItemState } from '@/lib/graphql/graphql'

const POS = () => {
    const [selectedCategoryId, setSelectedCategoryId] = useState<string | null>(null)
    const [cart, setCart] = useState<CartItem[]>([])

    const addToCart = (item: Item, variant?: ItemVariant) => {
        setCart(prevCart => {
            const taxIds = item.taxes?.map(tax => tax.id) ?? []

            // Create a unique ID for the cart item
            // If it's a variant, use item ID + variant ID to make it unique
            const cartItemId = variant ? `${item.id}-${variant.id}` : item.id

            // Check if this exact item/variant is already in the cart
            const existingItem = prevCart.find(cartItem => cartItem.cartItemId === cartItemId)

            if (existingItem) {
                // Update quantity if already in cart
                return prevCart.map(cartItem =>
                    cartItem.cartItemId === cartItemId
                        ? {
                            ...cartItem,
                            quantity: cartItem.quantity + 1,
                            taxIds
                        }
                        : cartItem
                )
            }

            // Determine the price to use (variant price or item price)
            const price = variant ? variant.finalPrice : item.price

            // Create a name that includes variant information if applicable
            let displayName = item.name
            if (variant && variant.variantValues && variant.variantValues.length > 0) {
                const variantInfo = variant.variantValues
                    .map(v => `${v.variantType.name}: ${v.value}`)
                    .join(', ')
                displayName = `${item.name} (${variantInfo})`
            }

            // Create new cart item
            const newCartItem = {
                cartItemId,
                id: item.id,
                variantId: variant?.id,
                name: displayName,
                description: item.description ?? '',
                price,
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
