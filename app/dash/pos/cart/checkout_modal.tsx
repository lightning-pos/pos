import React, { useState } from 'react'
import { Modal, RadioButtonGroup, RadioButton, ModalProps } from '@carbon/react'
import { CartItem } from './cart_section'
import { uid } from 'uid'
import { Customer, OrderItem, orderItemsTable, ordersTable, taxesTable } from '@/lib/db/sqlite/schema'
import { useDb } from '@/components/providers/drizzle_provider'

interface CheckoutModalProps extends ModalProps {
  cart: CartItem[]
  subtotal: number
  tax: number
  total: number
  customer: Customer | null
}

const CheckoutModal: React.FC<CheckoutModalProps> = ({
  open,
  onRequestClose,
  onRequestSubmit,
  cart,
  subtotal,
  tax,
  total,
  customer
}) => {
  const db = useDb()

  const [paymentMethod, setPaymentMethod] = useState('cash')

  const handleCheckout = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()
    if (!customer) {
      alert('No customer selected')
      return
    }

    const orderId = uid()
    const taxes = await db.select().from(taxesTable)
    const orderItems: OrderItem[] = cart.map(item => {
      const itemTaxAmount = item.taxIds?.reduce((sum, taxId) => {
        const tax = taxes.find(t => t.id === taxId)
        return sum + (item.price || 0) * item.quantity * ((tax?.rate || 0) / 100)
      }, 0) || 0

      return {
        id: uid(),
        orderId: orderId,
        itemId: item.id,
        itemName: item.name,
        quantity: item.quantity,
        priceAmount: item.price,
        taxAmount: itemTaxAmount,
        createdAt: new Date(),
        updatedAt: new Date(),
      }
    })

    try {
      // Insert the order
      await db.insert(ordersTable).values({
        id: orderId,
        customerId: customer.id,
        customerName: customer.name,
        customerPhoneNumber: customer.phoneNumber,
        orderDate: new Date(),
        netAmount: subtotal,
        discAmount: 0, // Assuming no discount for now
        taxableAmount: subtotal,
        taxAmount: tax,
        totalAmount: total,
        state: 'completed',
        createdAt: new Date(),
        updatedAt: new Date(),
      }).execute()

      // Insert order items
      await db.insert(orderItemsTable).values(orderItems).execute()

      onRequestSubmit?.(e)
    } catch (error) {
      console.error('Error during checkout:', error)
      alert('An error occurred during checkout. Please try again.')
    }
  }

  return (
    <Modal
      open={open}
      onRequestClose={onRequestClose}
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
