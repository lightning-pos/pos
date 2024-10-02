import React, { useState } from 'react'
import { Modal, RadioButtonGroup, RadioButton } from '@carbon/react'
// import { db } from '@/components/providers/system_provider'
import { CartItem } from './cart/cart_section'
// import { Customer } from '@/lib/powersync/app_schema'
import { uid } from 'uid'
import { Customer, orderItemsTable } from '@/lib/db/sqlite/schema'
import { useDb } from '@/components/providers/drizzle_provider'
import { ordersTable } from '@/lib/pglite/schema'

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
  const db = useDb()

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
      await db.insert(ordersTable).values({
        id: orderId,
        totalAmount: total,
        paymentMethod: paymentMethod,
        createdAt: Date.now(),
        status: 'completed',
        subtotal: subtotal,
        tax: tax,
        customerId: customer.id,
        customerName: customer.name,
        customerPhoneNumber: customer.phoneNumber,
      }).execute()

      // Insert order items
      for (const item of orderItems) {
        await db.insert(orderItemsTable).values(item).execute()
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
        <strong>Customer:</strong> {customer?.name} ({customer?.phoneNumber})
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
