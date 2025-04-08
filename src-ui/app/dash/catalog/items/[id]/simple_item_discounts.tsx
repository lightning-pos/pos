'use client'
import React from 'react'
import { Button } from '@carbon/react'
import { Add } from '@carbon/icons-react'

interface SimpleItemDiscountsProps {
    itemId: string
    itemName?: string
}

const SimpleItemDiscounts: React.FC<SimpleItemDiscountsProps> = ({ itemId, itemName }) => {
    console.log('SimpleItemDiscounts component rendered with:', { itemId, itemName })

    return (
        <div className="mt-4">
            <div className="flex justify-between items-center mb-4">
                <h3 className="text-lg font-medium">Applied Discounts</h3>
                <Button
                    kind="primary"
                    size="lg"
                    renderIcon={Add}
                    onClick={() => alert(`Add discount clicked for item: ${itemId}`)}
                >
                    Add Discount
                </Button>
            </div>
            
            <p>This is a simplified discount management interface.</p>
            <p>Item ID: {itemId}</p>
            <p>Item Name: {itemName || 'Unknown'}</p>
            
            <div className="bg-blue-50 p-4 rounded mt-4">
                <p className="text-blue-800 font-medium">No eligible discounts available</p>
                <p className="text-blue-600">You need to create discounts with "Specific Items" scope first.</p>
                <Button 
                    kind="tertiary" 
                    className="mt-2"
                    onClick={() => window.open('/dash/catalog/discounts', '_blank')}
                >
                    Go to Discounts Page
                </Button>
            </div>
        </div>
    )
}

export default SimpleItemDiscounts
