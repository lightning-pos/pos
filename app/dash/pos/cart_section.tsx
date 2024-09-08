import React, { useState } from 'react'
import { Column, Button, IconButton } from '@carbon/react'
import { Add, Subtract, ShoppingCart, Close } from '@carbon/icons-react'
import { Item } from '@/lib/powersync/app_schema'
import CheckoutModal from './checkout_modal'

export interface CartItem extends Item {
  quantity: number
}

interface CartSectionProps {
  cart: CartItem[]
  setCart: React.Dispatch<React.SetStateAction<CartItem[]>>
}

const CartSection: React.FC<CartSectionProps> = ({ cart, setCart }) => {
  const [isCheckoutModalOpen, setIsCheckoutModalOpen] = useState(false)
  const [checkoutModalKey, setCheckoutModalKey] = useState(0)

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

  const totalAmount = cart.reduce((sum, item) => sum + (item.price || 0) * item.quantity, 0)

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
      <div className='flex-grow overflow-y-auto'>
        <h2 className='text-xl font-bold mb-4 p-4'>Cart</h2>
        {cart.map((item) => (
          <div key={item.id} className='flex justify-between items-center mb-2'>
            <span>{item.name}</span>
            <br />
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
        ))}
      </div>
      <div className='mt-4 p-4'>
        <div className='flex justify-between items-center'>
          <span className='font-bold'>Total:</span>
          <span className='font-bold'>Rs. {totalAmount.toFixed(2)}</span>
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
        key={checkoutModalKey} // Add this key prop
        isOpen={isCheckoutModalOpen}
        onClose={() => setIsCheckoutModalOpen(false)}
        cart={cart}
        onCheckoutComplete={handleCheckoutComplete}
      />
    </div>
  )
}

export default CartSection
