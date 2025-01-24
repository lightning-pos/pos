import React, { useState, useEffect } from 'react'
import { AspectRatio, ClickableTile, Column, Grid } from '@carbon/react'
import { Corn } from '@carbon/icons-react'
import { useDb } from '@/components/providers/drizzle_provider'
import { itemsTable, Item, ItemTax } from '@/lib/db/sqlite/schema'
import { eq } from 'drizzle-orm'
import { money } from '@/lib/util/money'

interface ItemsSectionProps {
  selectedCategoryId: string | null
  addItemToCart: (item: Item & { taxes: ItemTax[] }) => void
}

const ItemsSection: React.FC<ItemsSectionProps> = ({ selectedCategoryId, addItemToCart }) => {
  const db = useDb()
  const [items, setItems] = useState<Array<Item & { taxes: ItemTax[] }>>([])

  useEffect(() => {
    const fetchItems = async () => {
      if (selectedCategoryId) {
        const result = await db.query.itemsTable.findMany({
          where: eq(itemsTable.categoryId, selectedCategoryId),
          with: {
            taxes: true
          }
        })
        setItems(result)
      }
    }
    fetchItems()
  }, [selectedCategoryId, db.query.itemsTable])

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
