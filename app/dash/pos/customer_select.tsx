import React, { useState, useEffect } from 'react'
import { TextInput } from '@carbon/react'
import { useDb } from '@/components/providers/drizzle_provider'
import { Customer, customersTable } from '@/lib/db/sqlite/schema'
import { eq, like } from 'drizzle-orm'
import { uid } from 'uid'

interface CustomerSelectProps {
  selectedCustomer: Customer | null
  setSelectedCustomer: React.Dispatch<React.SetStateAction<Customer | null>>
}

const CustomerSelect: React.FC<CustomerSelectProps> = ({ selectedCustomer, setSelectedCustomer }) => {
  const db = useDb()
  const [customerSearch, setCustomerSearch] = useState('')
  const [searchResults, setSearchResults] = useState<Customer[]>([])
  const [customerInput, setCustomerInput] = useState('')

  useEffect(() => {
    const searchCustomers = async () => {
      if (customerSearch.length > 2) {
        const results = await db
          .select()
          .from(customersTable)
          .where(like(customersTable.phoneNumber, `%${customerSearch}%`))
          .limit(3)
          .execute()
        setSearchResults(results)
      } else {
        setSearchResults([])
      }
    }
    searchCustomers()
  }, [customerSearch, db])


  const handleCustomerInput = async (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter') {
      e.preventDefault()
      const phoneNumber = customerInput.trim()
      if (!phoneNumber) return

      try {
        if (searchResults.length > 0) {
          setSelectedCustomer(searchResults[0])
        } else {
          if (phoneNumber.length !== 10) {
            return
          }

          await db
            .insert(customersTable)
            .values({ id: uid(), name: '', phoneNumber: phoneNumber })
            .execute()

          const newCustomer = await db.query.customersTable.findFirst({
            where: eq(customersTable.phoneNumber, phoneNumber),
          })

          if (!newCustomer) { return }

          setSelectedCustomer(newCustomer)
        }

        setCustomerInput('')
        setSearchResults([])
      } catch (error) {
        console.error('Error processing customer:', error)
        alert('Failed to process customer. Please try again.')
      }
    } else {
      setCustomerSearch(e.currentTarget.value)
    }
  }

  const clearSelectedCustomer = () => {
    setSelectedCustomer(null);
    setCustomerInput('');
    setSearchResults([]);
  };

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
        />
      ) : (
        <div className='flex items-center gap-2 mt-2 p-2'>
          <span>Customer: {selectedCustomer.name || 'No Name'} ({selectedCustomer.phoneNumber})</span>
          <span className='mr-2 cursor-pointer text-blue-500' onClick={clearSelectedCustomer}>Clear</span>
        </div>
      )}
      {!selectedCustomer && (
        <ul>
          {searchResults.map(customer => (
            <li key={customer.id} onClick={() => setSelectedCustomer(customer)} className='px-2 cursor-pointer'>
              {customer.phoneNumber}
            </li>
          ))}
        </ul>
      )}
    </div>
  )
}

export default CustomerSelect
