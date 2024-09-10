import React, { useState } from 'react'
import { Modal, RadioButtonGroup, RadioButton } from '@carbon/react'
import { db } from '@/components/providers/system_provider'
import { CartItem } from './cart_section'
import { Customer } from '@/lib/powersync/app_schema'
import { uid } from 'uid'

interface CheckoutModalProps {
  isOpen: boolean
  onClose: () => void
  cart: CartItem[]
  onCheckoutComplete: () => void
  subtotal: number
  tax: number
  total: number
  customer: Customer | null
}

const CheckoutModal: React.FC<CheckoutModalProps> = ({
  isOpen,
  onClose,
  cart,
  onCheckoutComplete,
  subtotal,
  tax,
  total,
  customer
}) => {
  const [paymentMethod, setPaymentMethod] = useState('cash')

  const handleCheckout = async () => {
    if (!customer) {
      alert('No customer selected')
      return
    }

    const orderId = uid()
    const orderItems = cart.map(item => ({
      id: uid(),
      order_id: orderId,
      item_id: item.id,
      item_name: item.name,
      quantity: item.quantity,
      price: item.price,
      tax: item.taxes?.reduce((sum, tax) => sum + (item.price || 0) * item.quantity * ((tax.rate || 0) / 100), 0) || 0,
    }))

    try {
      // Insert the order
      await db.insertInto('orders').values({
        id: orderId,
        total_amount: total,
        payment_method: paymentMethod,
        created_at: Date.now(),
        status: 'completed',
        subtotal: subtotal,
        tax: tax,
        customer_id: customer.id,
        customer_name: customer.name,
        customer_phone_number: customer.phone_number,
      }).execute()

      // Insert order items
      for (const item of orderItems) {
        await db.insertInto('order_items').values(item).execute()
      }

      onCheckoutComplete()
    } catch (error) {
      console.error('Error during checkout:', error)
      alert('An error occurred during checkout. Please try again.')
    }
  }

  return (
    <Modal
      open={isOpen}
      onRequestClose={onClose}
      modalHeading="Checkout"
      primaryButtonText="Complete Order"
      secondaryButtonText="Cancel"
      onRequestSubmit={handleCheckout}
    >
      <div className='mb-4'>
        <strong>Customer:</strong> {customer?.name} ({customer?.phone_number})
      </div>
      <div className='mb-4'>
        <strong>Subtotal:</strong> Rs. {subtotal.toFixed(2)}
      </div>
      <div className='mb-4'>
        <strong>Tax:</strong> Rs. {tax.toFixed(2)}
      </div>
      <div className='mb-4'>
        <strong>Total:</strong> Rs. {total.toFixed(2)}
      </div>
      <RadioButtonGroup
        legendText="Payment Method"
        name="payment-method"
        valueSelected={paymentMethod}
        onChange={(value) => setPaymentMethod(value as string)}
      >
        <RadioButton labelText="Cash" value="cash" id="cash" />
        <RadioButton labelText="Card" value="card" id="card" />
        <RadioButton labelText="UPI" value="upi" id="upi" />
      </RadioButtonGroup>
    </Modal>
  )
}

export default CheckoutModal
