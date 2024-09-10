import React, { useState, useEffect } from 'react'
import { Button, IconButton, Search } from '@carbon/react'
import { Add, Subtract, ShoppingCart, Close } from '@carbon/icons-react'
import { Item, Tax, Customer } from '@/lib/powersync/app_schema'
import { db } from '@/components/providers/system_provider'
import CheckoutModal from './checkout_modal'

export interface CartItem extends Item {
  quantity: number;
  taxes?: { id: string; name: string; rate: number }[];
}

interface CartSectionProps {
  cart: CartItem[]
  setCart: React.Dispatch<React.SetStateAction<CartItem[]>>
}

const CartSection: React.FC<CartSectionProps> = ({ cart, setCart }) => {
  const [isCheckoutModalOpen, setIsCheckoutModalOpen] = useState(false)
  const [checkoutModalKey, setCheckoutModalKey] = useState(0)
  const [taxes, setTaxes] = useState<Tax[]>([])
  const [customerSearch, setCustomerSearch] = useState('')
  const [searchResults, setSearchResults] = useState<Customer[]>([])
  const [selectedCustomer, setSelectedCustomer] = useState<Customer | null>(null)

  useEffect(() => {
    const fetchTaxes = async () => {
      const taxesResult = await db.selectFrom('taxes').selectAll().execute()
      setTaxes(taxesResult)
    }
    fetchTaxes()
  }, [])

  useEffect(() => {
    const searchCustomers = async () => {
      if (customerSearch.length > 2) {
        const results = await db
          .selectFrom('customers')
          .selectAll()
          .where((eb) => eb.or([
            eb('name', 'like', `%${customerSearch}%`),
            eb('phone_number', 'like', `%${customerSearch}%`),
          ]))
          .limit(3)
          .execute()
        setSearchResults(results)
      } else {
        setSearchResults([])
      }
    }
    searchCustomers()
  }, [customerSearch])

  const updateQuantity = (itemId: string, change: number) => {
    setCart(prevCart => {
      return prevCart.map(item => {
        if (item.id === itemId) {
          const newQuantity = Math.max(0, item.quantity + change)
          return newQuantity === 0 ? null : { ...item, quantity: newQuantity }
        }
        return item
      }).filter(Boolean) as CartItem[]
    })
  }

  const calculateItemTax = (item: CartItem) => {
    if (!item.tax_ids) return 0
    const itemTaxes = taxes.filter(tax => item.tax_ids?.includes(tax.id))
    return itemTaxes.reduce((sum, tax) => sum + (item.price || 0) * item.quantity * ((tax.rate || 0) / 100), 0)
  }

  const calculateTotalTax = () => {
    return cart.reduce((sum, item) => sum + calculateItemTax(item), 0)
  }

  const subtotal = cart.reduce((sum, item) => sum + (item.price || 0) * item.quantity, 0)
  const totalTax = calculateTotalTax()
  const totalAmount = subtotal + totalTax

  const handleCheckout = () => {
    if (!selectedCustomer) {
      alert('Please select a customer before checkout')
      return
    }
    setCheckoutModalKey(prev => prev + 1)
    setIsCheckoutModalOpen(true)
  }

  const handleCheckoutComplete = () => {
    setCart([])
    setIsCheckoutModalOpen(false)
    setSelectedCustomer(null)
    setCustomerSearch('')
  }

  return (
    <div className='flex flex-col h-[calc(100dvh-4rem)]'>
      <div className='mb-4'>
        <Search
          labelText="Search customers"
          placeholder="Search by name or phone"
          value={customerSearch}
          onChange={(e) => setCustomerSearch(e.target.value)}
        />
        {searchResults.length > 0 && (
          <ul className="mt-2 border border-gray-300 rounded">
            {searchResults.map((customer) => (
              <li
                key={customer.id}
                className="p-2 cursor-pointer"
                onClick={() => {
                  setSelectedCustomer(customer)
                  setCustomerSearch('')
                  setSearchResults([])
                }}
              >
                {customer.name} - {customer.phone_number}
              </li>
            ))}
          </ul>
        )}
        {selectedCustomer && (
          <div className='mt-2'>
            <strong>Selected Customer:</strong> {selectedCustomer.name} ({selectedCustomer.phone_number})
          </div>
        )}
      </div>

      <div className='flex-grow overflow-y-auto py-4'>
        {cart.map((item) => (
          <div key={item.id} className='flex flex-col mb-4'>
            <div className='flex justify-between items-center'>
              <span>{item.name}</span>
              <div className='flex items-center'>
                <IconButton size='sm' label="Decrease" onClick={() => updateQuantity(item.id, -1)}>
                  <Subtract size={16} />
                </IconButton>
                <span className='mx-2'>{item.quantity}</span>
                <IconButton size='sm' label="Increase" onClick={() => updateQuantity(item.id, 1)}>
                  <Add size={16} />
                </IconButton>
                <span className='ml-4'>Rs. {((item.price || 0) * item.quantity).toFixed(2)}</span>
              </div>
            </div>
            <div className='text-sm'>
              Tax: Rs. {calculateItemTax(item).toFixed(2)}
            </div>
          </div>
        ))}
      </div>
      <div className='mt-4 p-4'>
        <div className='flex justify-between items-center'>
          <span>Subtotal:</span>
          <span>Rs. {subtotal.toFixed(2)}</span>
        </div>
        <div className='flex justify-between items-center'>
          <span>Tax:</span>
          <span>Rs. {totalTax.toFixed(2)}</span>
        </div>
        <div className='flex justify-between items-center font-bold'>
          <span>Total:</span>
          <span>Rs. {totalAmount.toFixed(2)}</span>
        </div>
      </div>
      <div className='flex flex-row items-center my-4 w-full p-0'>
        <Button renderIcon={Close} onClick={() => setCart([])} kind='secondary' className='flex-1 mr-0'>
          Clear
        </Button>
        <Button
          renderIcon={ShoppingCart}
          onClick={handleCheckout}
          kind='primary'
          className='flex-1 ml-0'
          disabled={cart.length === 0 || !selectedCustomer}
        >
          Checkout ({cart.length})
        </Button>
      </div>
      <CheckoutModal
        key={checkoutModalKey}
        isOpen={isCheckoutModalOpen}
        onClose={() => setIsCheckoutModalOpen(false)}
        cart={cart}
        onCheckoutComplete={handleCheckoutComplete}
        subtotal={subtotal}
        tax={totalTax}
        total={totalAmount}
        customer={selectedCustomer}
      />
    </div>
  )
}

export default CartSection
