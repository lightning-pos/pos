'use client'
import { Column, Content, Grid } from '@carbon/react'
import { useState } from 'react'
import CartSection, { CartItem } from './cart/cart_section'
import { Item, ItemTax } from '@/lib/db/sqlite/schema'
import CategoriesSection from './categories_section'
import ItemsSection from './items_section'

const POS = () => {
  const [selectedCategoryId, setSelectedCategoryId] = useState<string | null>(null)
  const [cart, setCart] = useState<Array<CartItem>>([])

  const addToCart = (item: Item & { taxes: ItemTax[] }) => {
    setCart(prevCart => {
      const existingItem = prevCart.find(cartItem => cartItem.id === item.id)
      const taxIds = item.taxes.map(tax => tax.taxId)
      if (existingItem) {
        return prevCart.map(cartItem =>
          cartItem.id === item.id ? { ...cartItem, quantity: cartItem.quantity + 1, taxIds } : cartItem
        )
      } else {
        return [...prevCart, { ...item, quantity: 1, taxIds }]
      }
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
