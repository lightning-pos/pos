'use client'
import React, { useState } from 'react'
import { Modal, RadioButtonGroup, RadioButton } from '@carbon/react'
import { gql } from '@/lib/graphql/execute'
import {
    Tax,
    Customer
} from '@/lib/graphql/graphql'
import { CartItem } from './cart_section'

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

const formatPrice = (price: number): string => {
    return new Intl.NumberFormat('en-IN', {
        style: 'currency',
        currency: 'INR'
    }).format(price / 100)
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
                <strong>Subtotal:</strong> {formatPrice(subtotal)}
            </div>
            <div className='mb-4'>
                <strong>Tax:</strong> {formatPrice(totalTax)}
            </div>
            <div className='mb-4'>
                <strong>Total:</strong> {formatPrice(totalAmount)}
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
