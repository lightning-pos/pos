'use client'
import React, { useState, useEffect } from 'react'
import { ClickableTile } from '@carbon/react'
import { gql } from '@/lib/graphql/execute'
import {
    GetPosCategoriesDocument,
    ItemGroup,
    ItemGroupState
} from '@/lib/graphql/graphql'

interface CategoriesSectionProps {
    onCategorySelect: (categoryId: string) => void
}

const CategoriesSection: React.FC<CategoriesSectionProps> = ({ onCategorySelect }) => {
    const [categories, setCategories] = useState<ItemGroup[]>([])

    useEffect(() => {
        const fetchCategories = async () => {
            const result = await gql(GetPosCategoriesDocument, {
                first: 100
            })

            if (result.itemCategories) {
                // Transform categories to concrete types
                const transformedCategories = result.itemCategories.map((category): ItemGroup => ({
                    id: category.id,
                    name: category.name,
                    description: category.description,
                    state: category.state as ItemGroupState,
                    createdAt: category.createdAt,
                    updatedAt: category.updatedAt
                }))
                console.log(transformedCategories)
                setCategories(transformedCategories)
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
