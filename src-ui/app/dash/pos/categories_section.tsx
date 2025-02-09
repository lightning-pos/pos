import React, { useState, useEffect } from 'react'
import { ClickableTile } from '@carbon/react'
import { invoke } from '@tauri-apps/api/core'

interface ItemGroup {
    id: string
    name: string
    description?: string
    state: 'ACTIVE' | 'INACTIVE' | 'DELETED'
}

interface CategoriesSectionProps {
    onCategorySelect: (categoryId: string) => void
}

const CategoriesSection: React.FC<CategoriesSectionProps> = ({ onCategorySelect }) => {
    const [categories, setCategories] = useState<Array<ItemGroup>>([])

    useEffect(() => {
        const fetchCategories = async () => {
            try {
                const result: Array<{ itemCategories: ItemGroup[] }> = await invoke('graphql', {
                    query: `#graphql
            query {
              itemCategories(first: 100) {
                id
                name
                description
                state
              }
            }
          `,
                })

                if (result[0]?.itemCategories) {
                    setCategories(result[0].itemCategories)
                }
            } catch (error) {
                console.error('Error fetching categories:', error)
            }
        }
        fetchCategories()
    }, [])

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
