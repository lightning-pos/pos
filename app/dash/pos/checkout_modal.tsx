import React, { useState, useEffect } from 'react'
import {
  Modal,
  NumberInput,
  Tile,
  Grid,
  Column,
  DismissibleTag,
} from '@carbon/react'
import { CartItem } from './cart_section'
import { db } from '@/components/providers/system_provider'
import { uid } from 'uid'

interface CheckoutModalProps {
  isOpen: boolean
  onClose: () => void
  cart: CartItem[]
  onCheckoutComplete: () => void
}

const paymentMethods = ['Cash', 'UPI', 'Card']

const CheckoutModal: React.FC<CheckoutModalProps> = ({ isOpen, onClose, cart, onCheckoutComplete }) => {
  const [payments, setPayments] = useState<{ [method: string]: number }>({})

  // Reset payments when the modal is opened
  useEffect(() => {
    if (isOpen) {
      setPayments({})
    }
  }, [isOpen])

  const totalAmount = cart.reduce((sum, item) => sum + (item.price || 0) * item.quantity, 0)
  const remainingAmount = totalAmount - Object.values(payments).reduce((sum, amount) => sum + amount, 0)

  const updatePayment = (method: string, amount: number) => {
    if (amount > 0 && amount <= totalAmount) {
      setPayments(prevPayments => ({
        ...prevPayments,
        [method]: amount
      }))
    } else if (amount === 0) {
      setPayments(prevPayments => {
        const { [method]: _, ...rest } = prevPayments
        return rest
      })
    }
  }

  const handleCheckout = async () => {
    const orderId = uid();
    const totalAmount = cart.reduce((sum, item) => sum + (item.price || 0) * item.quantity, 0);
    const paymentMethod = Object.keys(payments).join(', ');

    await db.insertInto('orders').values({
      id: orderId,
      total_amount: totalAmount,
      payment_method: paymentMethod,
      created_at: new Date().getTime(),
      status: 'completed',
    }).execute();

    // Insert order items
    const orderItems = cart.map(item => ({
      id: uid(), // Add a unique id for each order item
      order_id: orderId,
      item_id: item.id,
      item_name: item.name, // Add this line
      quantity: item.quantity,
      price: item.price || 0,
    }));

    await db.insertInto('order_items').values(orderItems).execute();

    console.log('Order created:', { id: orderId, cart, payments });
    onCheckoutComplete();
    onClose();
  }

  return (
    <Modal
      modalHeading="Checkout"
      open={isOpen}
      size="md"
      primaryButtonText="Complete Checkout"
      primaryButtonDisabled={remainingAmount > 0}
      onRequestSubmit={handleCheckout}
      onRequestClose={onClose}
      secondaryButtonText="Cancel"
      onSecondarySubmit={onClose}
    >
      <div className="mb-4">
        <h3 className="text-md font-bold mb-2">Order Summary</h3>
        {cart.map((item) => (
          <div key={item.id} className="flex justify-between">
            <span>{item.name} x {item.quantity}</span>
            <span>Rs. {((item.price || 0) * item.quantity).toFixed(2)}</span>
          </div>
        ))}
        <div className="flex justify-between font-bold mt-2">
          <span>Total:</span>
          <span>Rs. {totalAmount.toFixed(2)}</span>
        </div>
      </div>

      <div className="mb-4">
        <h3 className="text-lg font-bold mb-2">Payments</h3>
        <Grid className="mb-2">
          {paymentMethods.map((method) => (
            <Column key={method}>
              <Tile>
                <h4>{method}</h4>
                <NumberInput
                  id={`payment-amount-${method}`}
                  label="Amount"
                  value={payments[method] || 0}
                  onChange={(e, { value }) => updatePayment(method, Number(value))}
                  min={0}
                  max={remainingAmount + (payments[method] || 0)}
                />
              </Tile>
            </Column>
          ))}
        </Grid>
        <div className="flex flex-wrap gap-2">
          {Object.entries(payments).map(([method, amount]) => (
            <DismissibleTag
              key={method}
              type="blue"
              onClose={() => updatePayment(method, 0)}
              title={'Remove Payment'}
              text={`${method}: Rs. ${amount.toFixed(2)}`}
            />
          ))}
        </div>
        <div className="font-bold mt-4">
          Remaining: Rs. {remainingAmount.toFixed(2)}
        </div>
      </div>
    </Modal>
  )
}

export default CheckoutModal
