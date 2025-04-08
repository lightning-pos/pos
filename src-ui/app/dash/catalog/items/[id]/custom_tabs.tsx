'use client'
import React, { useState } from 'react'
import { Button } from '@carbon/react'
import SimpleItemVariants from './simple_item_variants'
import SimpleItemDiscounts from './simple_item_discounts'

interface CustomTabsProps {
    itemId: string
    itemName?: string
    taxes: any[]
}

const CustomTabs: React.FC<CustomTabsProps> = ({ itemId, itemName, taxes }) => {
    const [activeTab, setActiveTab] = useState(0)

    const tabs = [
        { id: 0, name: 'Variants' },
        { id: 1, name: 'Discounts' },
        { id: 2, name: 'Taxes' }
    ]

    return (
        <div>
            {/* Tab Navigation */}
            <div className="flex border-b border-gray-200 mb-4">
                {tabs.map(tab => (
                    <button
                        key={tab.id}
                        className={`py-2 px-4 font-medium text-sm focus:outline-none ${
                            activeTab === tab.id
                                ? 'text-blue-600 border-b-2 border-blue-600'
                                : 'text-gray-500 hover:text-gray-700'
                        }`}
                        onClick={() => setActiveTab(tab.id)}
                    >
                        {tab.name}
                    </button>
                ))}
            </div>

            {/* Tab Content */}
            <div className="mt-4">
                {activeTab === 0 && (
                    <div className="p-4 bg-gray-100 rounded">
                        <SimpleItemVariants itemId={itemId} itemName={itemName} />
                    </div>
                )}

                {activeTab === 1 && (
                    <div className="p-4 bg-blue-100 rounded">
                        <SimpleItemDiscounts itemId={itemId} itemName={itemName} />
                    </div>
                )}

                {activeTab === 2 && (
                    <div className="p-4 bg-green-100 rounded">
                        <h3 className="text-lg font-medium mb-4">Applied Taxes</h3>
                        {taxes.length === 0 ? (
                            <p className="text-gray-500">No taxes applied to this item.</p>
                        ) : (
                            <ul className="list-disc pl-5">
                                {taxes.map(tax => (
                                    <li key={tax.id} className="mb-2">
                                        <span className="font-medium">{tax.name}</span>
                                        <span className="ml-2 text-gray-600">({tax.rate}%)</span>
                                        {tax.description && (
                                            <p className="text-sm text-gray-500">{tax.description}</p>
                                        )}
                                    </li>
                                ))}
                            </ul>
                        )}
                    </div>
                )}
            </div>
        </div>
    )
}

export default CustomTabs
