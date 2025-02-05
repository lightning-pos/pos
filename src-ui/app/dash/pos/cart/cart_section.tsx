import React, { useState, useEffect } from 'react'
import { Button } from '@carbon/react'
import { ShoppingCart, Close } from '@carbon/icons-react'
import CheckoutModal from './checkout_modal'
import { useDb } from '@/components/providers/drizzle_provider'
import { Customer, Item, Tax, taxesTable } from '@/lib/db/sqlite/schema'
import CustomerSelect from './customer_select'
import CartItem from './cart_item'
import { money, Money } from '@/lib/util/money'

export interface CartItem extends Item {
  quantity: number;
  taxIds?: string[];
}

interface CartSectionProps {
  cart: CartItem[]
  setCart: React.Dispatch<React.SetStateAction<CartItem[]>>
}

const CartSection: React.FC<CartSectionProps> = ({ cart, setCart }) => {
  const db = useDb()
  const [isCheckoutModalOpen, setIsCheckoutModalOpen] = useState(false)
  const [checkoutModalKey, setCheckoutModalKey] = useState(0)
  const [taxes, setTaxes] = useState<Tax[]>([])
  const [selectedCustomer, setSelectedCustomer] = useState<Customer | null>(null)

  useEffect(() => {
    const fetchTaxes = async () => {
      const taxesResult = await db.select().from(taxesTable).execute()
      setTaxes(taxesResult)
    }
    fetchTaxes()
  }, [db])

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

  const calculateTotalTax = (): Money => {
    return cart.reduce((sum, item) => {
      if (!item.taxIds) return sum
      const itemTaxes = taxes.filter(tax => item.taxIds?.some(t => t === tax.id))
      const itemPrice = money(item.price || 0, 'INR')
      const itemTax = itemTaxes.reduce((taxSum, tax) => {
        const taxRate = (tax.rate || 0) / 100
        return taxSum.add(itemPrice.multiply(item.quantity).multiply(taxRate))
      }, money(0, 'INR'))
      return sum.add(itemTax)
    }, money(0, 'INR'))
  }

  const subtotal = cart.reduce((sum, item) => {
    const itemPrice = money(item.price || 0, 'INR')
    return sum.add(itemPrice.multiply(item.quantity))
  }, money(0, 'INR'))

  const totalTax = calculateTotalTax()
  const totalAmount = subtotal.add(totalTax)

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
  }

  return (
    <div className='flex flex-col h-[calc(100dvh-4rem)]'>
      <CustomerSelect selectedCustomer={selectedCustomer} setSelectedCustomer={setSelectedCustomer} />

      <div className='flex-grow overflow-y-auto py-4'>
        {cart.map((item) => (
          <CartItem
            key={item.id}
            item={item}
            updateQuantity={updateQuantity}
          />
        ))}
      </div>
      <div className='mt-4 py-4'>
        <div className='flex justify-between items-center'>
          <span>Subtotal:</span>
          <span>{subtotal.format()}</span>
        </div>
        <div className='flex justify-between items-center'>
          <span>Tax:</span>
          <span>{totalTax.format()}</span>
        </div>
        <div className='flex justify-between items-center font-bold'>
          <span>Total:</span>
          <span>{totalAmount.format()}</span>
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
        open={isCheckoutModalOpen}
        onRequestClose={() => setIsCheckoutModalOpen(false)}
        onRequestSubmit={handleCheckoutComplete}
        cart={cart}
        subtotal={subtotal}
        tax={totalTax}
        total={totalAmount}
        customer={selectedCustomer}
      />
    </div>
  )
}

export default CartSection
