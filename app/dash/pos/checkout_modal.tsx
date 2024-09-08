import React, { useState, useEffect } from 'react'
import { Modal, NumberInput, Button } from '@carbon/react'
import { CartItem } from './cart_section'
import { db } from '@/components/providers/system_provider'
import { uid } from 'uid'

interface CheckoutModalProps {
  isOpen: boolean
  onClose: () => void
  cart: CartItem[]
  onCheckoutComplete: () => void
  subtotal: number
  tax: number
  total: number
}

interface PaymentMethod {
  method: string
  amount: number
}

const paymentMethods = ['Cash', 'Card', 'UPI']

const CheckoutModal: React.FC<CheckoutModalProps> = ({ isOpen, onClose, cart, onCheckoutComplete, subtotal, tax, total }) => {
  const [payments, setPayments] = useState<PaymentMethod[]>(paymentMethods.map(method => ({ method, amount: 0 })))

  useEffect(() => {
    if (isOpen) {
      setPayments(paymentMethods.map(method => ({ method, amount: method === 'Cash' ? total : 0 })))
    }
  }, [isOpen, total])

  const remainingAmount = total - payments.reduce((sum, payment) => sum + payment.amount, 0)

  const handlePaymentChange = (method: string, amount: number) => {
    setPayments(prevPayments =>
      prevPayments.map(payment =>
        payment.method === method ? { ...payment, amount } : payment
      )
    )
  }

  const handleCheckout = async () => {
    try {
      const orderId = uid()
      await db.insertInto('orders').values({
        id: orderId,
        total_amount: total,
        payment_method: JSON.stringify(payments.filter(p => p.amount > 0)),
        created_at: Date.now(),
        status: 'completed',
        subtotal: subtotal,
        tax: tax,
      }).execute()

      for (const item of cart) {
        await db.insertInto('order_items').values({
          id: uid(),
          order_id: orderId,
          item_id: item.id,
          item_name: item.name,
          quantity: item.quantity,
          price: item.price || 0,
          tax: item.taxes?.reduce((sum, tax) => sum + (item.price || 0) * item.quantity * (tax.rate / 100), 0) || 0,
        }).execute()
      }

      onCheckoutComplete()
    } catch (error) {
      console.error('Error during checkout:', error)
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
      primaryButtonDisabled={remainingAmount > 0}
    >
      <div className="mb-4">
        <p>Subtotal: Rs. {subtotal.toFixed(2)}</p>
        <p>Tax: Rs. {tax.toFixed(2)}</p>
        <p className="font-bold">Total: Rs. {total.toFixed(2)}</p>
        <p className="mt-2">Remaining: Rs. {remainingAmount.toFixed(2)}</p>
      </div>
      <div className="flex flex-col gap-4">
        {payments.map((payment) => (
          <NumberInput
            key={payment.method}
            id={`payment-${payment.method.toLowerCase()}`}
            label={payment.method}
            value={payment.amount}
            onChange={(e) => handlePaymentChange(payment.method, Number((e.target as HTMLInputElement).value))}
            step={0.01}
            min={0}
            max={total}
          />
        ))}
      </div>
    </Modal>
  )
}

export default CheckoutModal
