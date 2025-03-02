'use client'
import React, { useState } from 'react'
import { Modal, RadioButtonGroup, RadioButton } from '@carbon/react'
import {
    Tax,
    Customer
} from '@/lib/graphql/graphql'
import { CartItem } from './cart_section'
import { formatCurrency } from '@/lib/util/number_format'

enum PaymentMethod {
    CASH = 'cash',
    CARD = 'card',
    UPI = 'upi'
}

interface CheckoutModalProps {
    isOpen: boolean
    onClose: () => void
    onComplete: () => void
    cart: CartItem[]
    customer: Customer | null
    taxes: Tax[]
    subtotal: number
    totalTax: number
    totalAmount: number
}

const CheckoutModal: React.FC<CheckoutModalProps> = ({
    isOpen,
    onClose,
    onComplete,
    cart,
    customer,
    taxes,
    subtotal,
    totalTax,
    totalAmount
}) => {
    const [paymentMethod, setPaymentMethod] = useState<PaymentMethod>(PaymentMethod.CASH)
    const [processing, setProcessing] = useState(false)

    const handleSubmit = async () => {
        if (!customer) return

        try {
            setProcessing(true)
            await onComplete()
        } catch (error) {
            console.error('Error creating order:', error)
        } finally {
            setProcessing(false)
            onClose()
        }
    }

    return (
        <Modal
            open={isOpen}
            onRequestClose={onClose}
            modalHeading="Checkout"
            primaryButtonText="Complete Order"
            secondaryButtonText="Cancel"
            onRequestSubmit={handleSubmit}
            primaryButtonDisabled={processing}
        >
            <div className='mb-4'>
                <strong>Customer:</strong> {customer?.fullName} ({customer?.phone})
            </div>
            <div className='mb-4'>
                <strong>Subtotal:</strong> {formatCurrency(subtotal)}
            </div>
            <div className='mb-4'>
                <strong>Tax:</strong> {formatCurrency(totalTax)}
            </div>
            <div className='mb-4'>
                <strong>Total:</strong> {formatCurrency(totalAmount)}
            </div>
            <RadioButtonGroup
                legendText="Payment Method"
                name="payment-method"
                valueSelected={paymentMethod}
                onChange={(value: string | number | undefined) => {
                    if (Object.values(PaymentMethod).includes(value as PaymentMethod)) {
                        setPaymentMethod(value as PaymentMethod)
                    }
                }}
            >
                <RadioButton labelText="Cash" value={PaymentMethod.CASH} id={PaymentMethod.CASH} />
                <RadioButton labelText="Card" value={PaymentMethod.CARD} id={PaymentMethod.CARD} />
                <RadioButton labelText="UPI" value={PaymentMethod.UPI} id={PaymentMethod.UPI} />
            </RadioButtonGroup>
        </Modal>
    )
}

export default CheckoutModal
