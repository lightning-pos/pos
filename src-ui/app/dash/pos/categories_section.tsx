import React, { useState, useEffect } from 'react'
import { ClickableTile, Column } from '@carbon/react'
import { useDb } from '@/components/providers/drizzle_provider'
import { itemCategoriesTable, ItemCategory } from '@/lib/db/sqlite/schema'

interface CategoriesSectionProps {
  onCategorySelect: (categoryId: string) => void
}

const CategoriesSection: React.FC<CategoriesSectionProps> = ({ onCategorySelect }) => {
  const db = useDb()
  const [categories, setCategories] = useState<Array<ItemCategory>>([])

  useEffect(() => {
    const fetchCategories = async () => {
      const result = await db.select().from(itemCategoriesTable).execute()
      setCategories(result)
    }
    fetchCategories()
  }, [db])

  return (
    <>
      {categories.map((category) => (
        <ClickableTile key={category.id} onClick={() => onCategorySelect(category.id)}>
          {category.name}
        </ClickableTile>
      ))}
    </>
  )
}

export default CategoriesSection
