import React, { useState, useEffect } from 'react'
import { AspectRatio, ClickableTile, Column, Grid } from '@carbon/react'
import { Corn } from '@carbon/icons-react'
import { useDb } from '@/components/providers/drizzle_provider'
import { itemsTable, Item } from '@/lib/db/sqlite/schema'
import { eq } from 'drizzle-orm'

interface ItemsSectionProps {
  selectedCategoryId: string | null
  onAddToCart: (item: Item) => void
}

const ItemsSection: React.FC<ItemsSectionProps> = ({ selectedCategoryId, onAddToCart }) => {
  const db = useDb()
  const [items, setItems] = useState<Array<Item>>([])

  useEffect(() => {
    const fetchItems = async () => {
      if (selectedCategoryId) {
        const result = await db.query.itemsTable.findMany({
          where: eq(itemsTable.categoryId, selectedCategoryId),
        })
        setItems(result)
      }
    }
    fetchItems()
  }, [selectedCategoryId])

  return (
    <Grid narrow className='mx-4'>
      {items.map((item) => (
        <Column key={item.id} lg={2} className='mb-4'>
          <ClickableTile renderIcon={Corn} onClick={() => onAddToCart(item)}>
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
  )
}

export default ItemsSection
