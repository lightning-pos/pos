'use client'
import React, { useState } from 'react'
import { TextInput } from '@carbon/react'
import { gql } from '@/lib/graphql/execute'
import {
    GetPosCustomerByPhoneDocument,
    CreatePosCustomerDocument,
    Customer
} from '@/lib/graphql/graphql'

interface CustomerSelectProps {
    selectedCustomer: Customer | null
    setSelectedCustomer: React.Dispatch<React.SetStateAction<Customer | null>>
}

const CustomerSelect: React.FC<CustomerSelectProps> = ({ selectedCustomer, setSelectedCustomer }) => {
    const [customerInput, setCustomerInput] = useState('')
    const [searchResults, setSearchResults] = useState<Customer[]>([])
    const [loading, setLoading] = useState(false)

    const searchCustomer = async (phone: string) => {
        try {
            const result = await gql(GetPosCustomerByPhoneDocument, {
                phone
            })

            if (result.customerByPhone) {
                // Transform to concrete type
                const customer: Customer = {
                    id: result.customerByPhone.id,
                    fullName: result.customerByPhone.fullName,
                    phone: result.customerByPhone.phone,
                    email: result.customerByPhone.email,
                    address: result.customerByPhone.address,
                    createdAt: result.customerByPhone.createdAt,
                    updatedAt: result.customerByPhone.updatedAt
                }
                setSearchResults([customer])
            } else {
                setSearchResults([])
            }
        } catch (error) {
            console.error('Error searching customer:', error)
            setSearchResults([])
        }
    }

    const createCustomer = async (phone: string) => {
        try {
            const result = await gql(CreatePosCustomerDocument, {
                fullName: "",
                phone
            })

            if (result.createCustomer) {
                // Transform to concrete type
                const customer: Customer = {
                    id: result.createCustomer.id,
                    fullName: result.createCustomer.fullName,
                    phone: result.createCustomer.phone,
                    email: result.createCustomer.email,
                    address: result.createCustomer.address,
                    createdAt: result.createCustomer.createdAt,
                    updatedAt: result.createCustomer.updatedAt
                }
                setSelectedCustomer(customer)
                setSearchResults([])
                setCustomerInput('')
            }
        } catch (error) {
            console.error('Error creating customer:', error)
        }
    }

    const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const value = e.target.value
        setCustomerInput(value)
        setLoading(true)

        if (value.length >= 10) {
            searchCustomer(value)
        } else {
            setSearchResults([])
        }
        setLoading(false)
    }

    const handleKeyUp = (e: React.KeyboardEvent<HTMLInputElement>) => {
        if (e.key === 'Enter' && customerInput.length >= 10) {
            createCustomer(customerInput)
        }
    }

    const clearSelectedCustomer = () => {
        setSelectedCustomer(null)
        setCustomerInput('')
        setSearchResults([])
    }

    return (
        <div className='mb-4'>
            {!selectedCustomer ? (
                <TextInput
                    id="customer-input"
                    labelText="Customer Phone Number"
                    placeholder="Enter phone number and press Enter"
                    value={customerInput}
                    onChange={handleInputChange}
                    onKeyUp={handleKeyUp}
                    disabled={loading}
                />
            ) : (
                <div className='flex items-center gap-2 mt-2 p-2'>
                    <span>Customer: {selectedCustomer.fullName || 'No Name'} ({selectedCustomer.phone})</span>
                    <span className='mr-2 cursor-pointer text-blue-500' onClick={clearSelectedCustomer}>Clear</span>
                </div>
            )}
            {!selectedCustomer && (
                <ul>
                    {searchResults.map(customer => (
                        <li
                            key={customer.id}
                            className="cursor-pointer p-2 hover:bg-gray-100"
                            onClick={() => setSelectedCustomer(customer)}
                        >
                            {customer.fullName || 'No Name'} ({customer.phone})
                        </li>
                    ))}
                </ul>
            )}
        </div>
    )
}

export default CustomerSelect
