import React, { useState } from 'react'
import { TextInput } from '@carbon/react'
import { invoke } from '@tauri-apps/api/core'

interface Customer {
    id: string
    fullName: string
    phone: string
}

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
            const result: Array<{ customerByPhone: Customer }> = await invoke('graphql', {
                query: `#graphql
                    query {
                        customerByPhone(phone: "${phone}") {
                            id fullName phone
                        }
                    }
                `,
            })
            if (result[0]?.customerByPhone) {
                setSearchResults([result[0].customerByPhone])
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
            const result: Array<{ createCustomer: Customer }> = await invoke('graphql', {
                query: `#graphql
                    mutation {
                        createCustomer(
                            customer: { fullName: "", phone: "${phone}" }
                        ) {
                            id fullName phone
                        }
                    }
                `,
            })
            debugger;
            if (result[0]?.createCustomer) {
                setSelectedCustomer(result[0].createCustomer)
            }
        } catch (error) {
            console.error('Error creating customer:', error)
            alert('Failed to create customer. Please try again.')
        }
    }

    const handleCustomerInput = async (e: React.KeyboardEvent<HTMLInputElement>) => {
        if (e.key === 'Enter') {
            e.preventDefault()
            const phoneNumber = customerInput.trim()
            if (!phoneNumber) return

            try {
                setLoading(true)
                if (searchResults.length > 0) {
                    setSelectedCustomer(searchResults[0])
                } else {
                    if (phoneNumber.length !== 10) {
                        return
                    }
                    await createCustomer(phoneNumber)
                }

                setCustomerInput('')
                setSearchResults([])
            } catch (error) {
                console.error('Error processing customer:', error)
                alert('Failed to process customer. Please try again.')
            } finally {
                setLoading(false)
            }
        } else {
            const value = e.currentTarget.value
            setCustomerInput(value)
            if (value.length > 2) {
                searchCustomer(value)
            } else {
                setSearchResults([])
            }
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
                    onChange={(e) => setCustomerInput(e.target.value)}
                    onKeyUp={handleCustomerInput}
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
