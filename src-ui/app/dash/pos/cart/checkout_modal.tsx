import React, { useState } from 'react'
import { Modal, RadioButtonGroup, RadioButton, ModalProps } from '@carbon/react'
import { invoke } from '@tauri-apps/api/core'

interface Tax {
    id: string
    name: string
    rate: number
    description?: string
}

interface Customer {
    id: string
    fullName: string
    email?: string
    phone?: string
    address?: string
}

interface CartItem {
    id: string
    name: string
    description?: string
    price: number
    quantity: number
    taxIds?: string[]
}

interface CheckoutModalProps extends ModalProps {
    cart: CartItem[]
    subtotal: number
    tax: number
    total: number
    customer: Customer | null
}

const formatPrice = (price: number): string => {
    return new Intl.NumberFormat('en-IN', {
        style: 'currency',
        currency: 'INR'
    }).format(price);
};

// Convert a number to base units (paise) for storage
const toBaseUnits = (amount: number): number => {
    return Math.round(amount * 100);
};

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
    const [paymentMethod, setPaymentMethod] = useState('cash')

    const handleCheckout = async (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault()
        if (!customer) {
            alert('No customer selected')
            return
        }

        try {
            const orderItems = cart.map(item => ({
                itemId: item.id,
                itemName: item.name,
                quantity: item.quantity,
                priceAmount: toBaseUnits(item.price).toString(),
                taxAmount: toBaseUnits(item.taxIds?.reduce((sum, taxId) => {
                    // We'll calculate tax amount in the backend to ensure consistency
                    return sum
                }, 0) || 0).toString()
            }))

            const result: Array<{ createSalesOrder: { id: string } }> = await invoke('graphql', {
                query: `#graphql
                    mutation {
                        createSalesOrder(
                            salesOrder: {
                                customerId: "${customer.id}",
                                customerName: "${customer.fullName}",
                                customerPhoneNumber: "${customer.phone || ''}",
                                orderDate: "${new Date().toISOString().split('.')[0].replace('T', ' ')}",
                                netAmount: "${toBaseUnits(subtotal)}",
                                discAmount: "0",
                                taxableAmount: "${toBaseUnits(subtotal)}",
                                taxAmount: "${toBaseUnits(tax)}",
                                totalAmount: "${toBaseUnits(total)}",
                                state: COMPLETED,
                                items: [${orderItems.map(item => `{
                                    itemId: "${item.itemId}",
                                    itemName: "${item.itemName}",
                                    quantity: ${item.quantity},
                                    priceAmount: "${item.priceAmount}",
                                    taxAmount: "${item.taxAmount}"
                                }`).join(',')}]
                            }
                        ) {
                            id
                        }
                    }
                `
            })

            console.log(result)

            if (result[0]?.createSalesOrder?.id) {
                onRequestSubmit?.(e)
            } else {
                throw new Error('Failed to create sales order')
            }
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
                <strong>Customer:</strong> {customer?.fullName} ({customer?.phone})
            </div>
            <div className='mb-4'>
                <strong>Subtotal:</strong> {formatPrice(subtotal)}
            </div>
            <div className='mb-4'>
                <strong>Tax:</strong> {formatPrice(tax)}
            </div>
            <div className='mb-4'>
                <strong>Total:</strong> {formatPrice(total)}
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
