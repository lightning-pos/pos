'use client'
import { db } from '@/components/providers/system_provider'
import { Category, Item } from '@/lib/powersync/app_schema'
import { AspectRatio, ClickableTile, Column, Content, Grid } from '@carbon/react'
import { useState, useEffect } from 'react'
import { Corn } from '@carbon/icons-react'
import CartSection, { CartItem } from './cart_section'

const POS = () => {
  const [categories, setCategories] = useState<Array<Category>>([])
  const [selectedCategory, setSelectedCategory] = useState<Category | null>(null)
  const [items, setItems] = useState<Array<Item>>([])
  const [cart, setCart] = useState<Array<CartItem>>([])

  useEffect(() => {
    const fetchCategories = async () => {
      const result = await db.selectFrom('item_categories').selectAll().execute()
      setCategories(result)
    }
    fetchCategories()
  }, [])

  useEffect(() => {
    const fetchItems = async () => {
      if (selectedCategory) {
        const result = await db
          .selectFrom('items')
          .selectAll()
          .where('item_category_id', '=', selectedCategory.id)
          .execute()
        setItems(result)
      }
    }
    fetchItems()
  }, [selectedCategory])

  const addToCart = (item: Item) => {
    setCart(prevCart => {
      const existingItem = prevCart.find(cartItem => cartItem.id === item.id)
      if (existingItem) {
        return prevCart.map(cartItem =>
          cartItem.id === item.id ? { ...cartItem, quantity: cartItem.quantity + 1 } : cartItem
        )
      } else {
        return [...prevCart, { ...item, quantity: 1 }]
      }
    })
  }

  return (
    <Content className='min-h-[calc(100dvh-3rem)] p-0 pt-4'>
      <Grid className='m-auto p-0'>
        <Column lg={2} className='m-0 p-0 max-h-[calc(100dvh-4rem)]'>
          {categories.map((category) => (
            <ClickableTile key={category.id} onClick={() => setSelectedCategory(category)}>
              {category.name}
            </ClickableTile>
          ))}
        </Column>
        <Column lg={10} className='m-0 p-0  max-h-[calc(100dvh-4rem)]'>
          <Grid narrow className='mx-4'>
            {items.map((item) => (
              <Column key={item.id} lg={2} className='mb-4'>
                <ClickableTile renderIcon={Corn} onClick={() => addToCart(item)}>
                  <AspectRatio ratio='3x2'>
                    <div className='flex flex-col justify-between h-full'>
                      <span>{item.name}</span>
                      <span>Rs. {(item.price || 0).toFixed(2)}</span>
                    </div>
                  </AspectRatio>
                </ClickableTile>
              </Column>
            ))}
          </Grid>
        </Column>
        <Column lg={4} className='m-0 p-0 max-h-[calc(100dvh-4rem)]'>
          <CartSection cart={cart} setCart={setCart} />
        </Column>
      </Grid>
    </Content>
  )
}

export default POS
