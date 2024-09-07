'use client'
import { db } from '@/components/providers/system_provider'
import { Category, Item } from '@/lib/powersync/app_schema'
import { AspectRatio, ClickableTile, Column, Content, Grid } from '@carbon/react'
import { useState, useEffect } from 'react'
import { Corn } from '@carbon/icons-react'

const POS = () => {
  const [categories, setCategories] = useState<Array<Category>>([])
  const [selectedCategory, setSelectedCategory] = useState<Category | null>(null)
  const [items, setItems] = useState<Array<Item>>([])

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

  return (
    <Content className='min-h-[calc(100dvh-3rem)] p-0 pt-4'>
      <Grid fullWidth className='p-0'>
        <Column lg={2} className='m-0 p-0'>
          {categories.map((category) => (
            <ClickableTile key={category.id} onClick={() => setSelectedCategory(category)}>
              {category.name}
            </ClickableTile>
          ))}
        </Column>
        <Column lg={10} className='px-4'>
          <Grid narrow>
            {items.map((item) => (
              <Column key={item.id} lg={2} className='mb-4'>
                <ClickableTile renderIcon={Corn}>
                  <AspectRatio ratio='3x2'>{item.name}</AspectRatio>
                </ClickableTile>
              </Column>
            ))}
          </Grid>
        </Column>
        <Column lg={4} className='m-0 p-0'>Cart</Column>
      </Grid>
    </Content>
  )
}

export default POS
