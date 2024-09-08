import React, { useState, useEffect } from 'react'
import { Button, IconButton } from '@carbon/react'
import { Add, Subtract, ShoppingCart, Close } from '@carbon/icons-react'
import { Item, Tax } from '@/lib/powersync/app_schema'
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

  useEffect(() => {
    const fetchTaxes = async () => {
      const taxesResult = await db.selectFrom('taxes').selectAll().execute()
      setTaxes(taxesResult)
    }
    fetchTaxes()
  }, [])

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
    setCheckoutModalKey(prev => prev + 1) // Increment the key
    setIsCheckoutModalOpen(true)
  }

  const handleCheckoutComplete = () => {
    // Clear the cart after successful checkout
    setCart([])
    setIsCheckoutModalOpen(false)
  }

  return (
    <div className='flex flex-col h-[calc(100dvh-4rem)]'>
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
        <Button renderIcon={ShoppingCart} onClick={handleCheckout} kind='primary' className='flex-1 ml-0' disabled={cart.length === 0}>
          Checkout
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
      />
    </div>
  )
}

export default CartSection
