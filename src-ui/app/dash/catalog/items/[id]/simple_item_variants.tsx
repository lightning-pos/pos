'use client'
import React from 'react'
import { Button } from '@carbon/react'
import { Add } from '@carbon/icons-react'

interface SimpleItemVariantsProps {
    itemId: string
    itemName?: string
}

const SimpleItemVariants: React.FC<SimpleItemVariantsProps> = ({ itemId, itemName }) => {
    console.log('SimpleItemVariants component rendered with:', { itemId, itemName })

    return (
        <div className="mt-4">
            <div className="flex justify-between items-center mb-4">
                <h3 className="text-lg font-medium">{itemName ? `${itemName} Variants` : 'Item Variants'}</h3>
                <Button
                    renderIcon={Add}
                    onClick={() => alert(`Add variant clicked for item: ${itemId}`)}
                    size="lg"
                    kind="primary"
                >
                    Add Variant
                </Button>
            </div>
            
            <p>This is a simplified variant management interface.</p>
            <p>Item ID: {itemId}</p>
            <p>Item Name: {itemName || 'Unknown'}</p>
        </div>
    )
}

export default SimpleItemVariants
