'use client'
import React, { useState, useEffect } from 'react'
import { Button, ToastNotification, InlineNotification } from '@carbon/react'
import { ShoppingCart, Close } from '@carbon/icons-react'
import { gql } from '@/lib/graphql/execute'
import {
    GetPosTaxesDocument,
    GetPosCustomerByPhoneDocument,
    CreatePosCustomerDocument,
    Item,
    Tax,
    Customer,
    SalesOrderState,
    SalesOrderNewInput,
    SalesOrderItemInput,
    PosCreateSalesOrderDocument,
    ItemNature,
    ItemState
} from '@/lib/graphql/graphql'
import CheckoutModal from './checkout_modal'
import CustomerSelect from './customer_select'
import CartItem from './cart_item'
import { formatCurrency } from '@/lib/util/number_format'

export interface CartItem {
    cartItemId: string
    id: string
    variantId?: string
    name: string
    description: string
    price: string
    quantity: number
    taxIds?: string[]
    nature: ItemNature
    state: ItemState
    createdAt: string
    updatedAt: string
}

interface CartSectionProps {
    cart: CartItem[]
    setCart: React.Dispatch<React.SetStateAction<CartItem[]>>
}

const CartSection: React.FC<CartSectionProps> = ({ cart, setCart }) => {
    const [isCheckoutModalOpen, setIsCheckoutModalOpen] = useState(false)
    const [checkoutModalKey, setCheckoutModalKey] = useState(0)
    const [taxes, setTaxes] = useState<Tax[]>([])
    const [selectedCustomer, setSelectedCustomer] = useState<Customer | null>(null)
    const [isSubmitting, setIsSubmitting] = useState(false)
    const [customerPhone, setCustomerPhone] = useState('')
    const [customerName, setCustomerName] = useState('')
    const [notification, setNotification] = useState<{
        type: 'success' | 'error',
        title: string,
        subtitle?: string,
        show: boolean
    } | null>(null)

    useEffect(() => {
        const fetchTaxes = async () => {
            const result = await gql(GetPosTaxesDocument)

            if (result.taxes) {
                // Transform taxes to concrete types
                const transformedTaxes = result.taxes.map((tax): Tax => ({
                    id: tax.id,
                    name: tax.name,
                    rate: tax.rate,
                    description: tax.description,
                    createdAt: tax.createdAt,
                    updatedAt: tax.updatedAt
                }))
                setTaxes(transformedTaxes)
            }
        }
        fetchTaxes()
    }, [])

    const updateQuantity = (cartItemId: string, change: number) => {
        setCart(prevCart => {
            return prevCart.map(item => {
                if (item.cartItemId === cartItemId) {
                    const newQuantity = Math.max(0, item.quantity + change)
                    return newQuantity === 0 ? null : { ...item, quantity: newQuantity }
                }
                return item
            }).filter(Boolean) as CartItem[]
        })
    }

    const calculateTotalTax = (): number => {
        return cart.reduce((sum, item) => {
            if (!item.taxIds) return sum
            const itemTaxes = taxes.filter(tax => item.taxIds?.includes(tax.id))
            const itemPrice = item.price
            const itemTax = itemTaxes.reduce((taxSum, tax) => {
                const taxRate = parseFloat(tax.rate)
                return taxSum + (parseFloat(itemPrice) * item.quantity * taxRate / 100)
            }, 0)
            return sum + itemTax
        }, 0)
    }

    const subtotal = cart.reduce((sum, item) => {
        return sum + (parseFloat(item.price) * item.quantity)
    }, 0)

    const totalTax = calculateTotalTax()
    const totalAmount = subtotal + totalTax

    const showNotification = (type: 'success' | 'error', title: string, subtitle?: string) => {
        setNotification({ type, title, subtitle, show: true })
        setTimeout(() => setNotification(null), 5000)
    }

    const handleCheckout = () => {
        if (cart.length === 0) return
        setIsCheckoutModalOpen(true)
    }

    const handleCheckoutComplete = async () => {
        if (!selectedCustomer) return

        try {
            setIsSubmitting(true)

            const now = new Date()
            // Format date as yyyy-MM-dd HH:mm:ss for LocalDateTime
            const orderDate = now.toISOString().replace('T', ' ').substring(0, 19)

            const orderItems: SalesOrderItemInput[] = cart.map(item => {
                const taxAmount = item.taxIds
                    ? taxes
                        .filter(tax => item.taxIds?.includes(tax.id))
                        .reduce((sum, tax) => sum + (parseFloat(item.price) * parseFloat(tax.rate) / 100), 0)
                        .toString()
                    : '0'

                return {
                    itemId: item.id,
                    itemName: item.name,
                    quantity: item.quantity,
                    priceAmount: item.price,
                    discAmount: "0", // No discount for now
                    taxableAmount: item.price, // Same as price amount since no discount
                    taxAmount: taxAmount,
                    totalAmount: ((parseFloat(item.price) + (item.taxIds
                        ? taxes
                            .filter(tax => item.taxIds?.includes(tax.id))
                            .reduce((sum, tax) => sum + (parseFloat(item.price) * parseFloat(tax.rate) / 100), 0)
                        : 0)) * item.quantity).toString(),
                    sku: item.variantId ? `variant-${item.variantId}` : undefined // Store variant ID in SKU field
                }
            })

            const orderInput: SalesOrderNewInput = {
                customerId: selectedCustomer.id,
                customerName: selectedCustomer.fullName,
                customerPhoneNumber: selectedCustomer.phone || '',
                orderDate: orderDate,
                netAmount: subtotal.toString(),
                discAmount: '0',
                taxableAmount: subtotal.toString(),
                taxAmount: totalTax.toString(),
                totalAmount: totalAmount.toString(),
                channelId: '00000000-0000-0000-0000-000000000000', // Required field
                locationId: '00000000-0000-0000-0000-000000000000', // Required field
                costCenterId: '00000000-0000-0000-0000-000000000000',
                items: orderItems
            }

            await gql(PosCreateSalesOrderDocument, { salesOrder: orderInput })
            setCart([])
            setIsCheckoutModalOpen(false)
            setSelectedCustomer(null)
            showNotification('success', 'Success', 'Order has been submitted successfully')
        } catch (error) {
            console.error('Error submitting order:', error)
            showNotification('error', 'Error', 'Failed to submit order')
        } finally {
            setIsSubmitting(false)
        }
    }

    const handleSubmitOrder = async () => {
        if (cart.length === 0) return

        try {
            setIsSubmitting(true)

            let customerId = null
            if (customerPhone) {
                try {
                    const customerResult = await gql(GetPosCustomerByPhoneDocument, { phone: customerPhone })
                    if (customerResult.customerByPhone) {
                        customerId = customerResult.customerByPhone.id
                    } else {
                        const createResult = await gql(CreatePosCustomerDocument, {
                            fullName: customerName || 'Guest',
                            phone: customerPhone
                        })
                        if (createResult.createCustomer) {
                            customerId = createResult.createCustomer.id
                        }
                    }
                } catch (error) {
                    console.error('Error with customer lookup/creation:', error)
                }
            }

            const orderItems: SalesOrderItemInput[] = cart.map(item => {
                const taxAmount = item.taxIds
                    ? taxes
                        .filter(tax => item.taxIds?.includes(tax.id))
                        .reduce((sum, tax) => sum + (parseFloat(item.price) * parseFloat(tax.rate) / 100), 0)
                        .toString()
                    : '0'

                return {
                    itemId: item.id,
                    itemName: item.name,
                    quantity: item.quantity,
                    priceAmount: item.price,
                    discAmount: "0", // No discount for now
                    taxableAmount: item.price, // Same as price amount since no discount
                    taxAmount: taxAmount,
                    totalAmount: ((parseFloat(item.price) + (item.taxIds
                        ? taxes
                            .filter(tax => item.taxIds?.includes(tax.id))
                            .reduce((sum, tax) => sum + (parseFloat(item.price) * parseFloat(tax.rate) / 100), 0)
                        : 0)) * item.quantity).toString(),
                    sku: undefined // Optional field
                }
            })

            const orderInput: SalesOrderNewInput = {
                customerId: customerId || '00000000-0000-0000-0000-000000000000',
                customerName: customerName || 'Walk-in Customer',
                customerPhoneNumber: customerPhone || '',
                orderDate: new Date().toISOString(),
                netAmount: subtotal.toString(),
                discAmount: '0',
                taxableAmount: subtotal.toString(),
                taxAmount: totalTax.toString(),
                totalAmount: totalAmount.toString(),
                channelId: '00000000-0000-0000-0000-000000000000', // Required field
                locationId: '00000000-0000-0000-0000-000000000000', // Required field
                costCenterId: '00000000-0000-0000-0000-000000000000',
                items: orderItems
            }

            await gql(PosCreateSalesOrderDocument, { salesOrder: orderInput })
            setCart([])
            setIsCheckoutModalOpen(false)
            setSelectedCustomer(null)
            showNotification('success', 'Success', 'Order has been submitted successfully')
        } catch (error) {
            console.error('Error submitting order:', error)
            showNotification('error', 'Error', 'Failed to submit order')
        } finally {
            setIsSubmitting(false)
        }
    }

    return (
        <div className='flex flex-col h-[calc(100dvh-4rem)]'>
            {notification && notification.show && (
                <div className="mb-4">
                    {notification.type === 'success' ? (
                        <InlineNotification
                            kind="success"
                            title={notification.title}
                            subtitle={notification.subtitle}
                            onClose={() => setNotification(null)}
                        />
                    ) : (
                        <InlineNotification
                            kind="error"
                            title={notification.title}
                            subtitle={notification.subtitle}
                            onClose={() => setNotification(null)}
                        />
                    )}
                </div>
            )}

            <CustomerSelect selectedCustomer={selectedCustomer} setSelectedCustomer={setSelectedCustomer} />

            <div className='flex-grow overflow-y-auto py-4'>
                {cart.map((item) => (
                    <CartItem
                        key={item.id}
                        item={item}
                        updateQuantity={updateQuantity}
                    />
                ))}
            </div>
            <div className='mt-4 py-4'>
                <div className='flex justify-between items-center'>
                    <span>Subtotal:</span>
                    <span>{formatCurrency(subtotal)}</span>
                </div>
                <div className='flex justify-between items-center'>
                    <span>Tax:</span>
                    <span>{formatCurrency(totalTax)}</span>
                </div>
                <div className='flex justify-between items-center font-bold'>
                    <span>Total:</span>
                    <span>{formatCurrency(totalAmount)}</span>
                </div>
            </div>
            <div className='flex flex-row items-center my-4 w-full p-0'>
                <Button renderIcon={Close} onClick={() => setCart([])} kind='secondary' className='flex-1 mr-0'>
                    Clear
                </Button>
                <Button
                    renderIcon={ShoppingCart}
                    onClick={handleCheckout}
                    kind='primary'
                    className='flex-1 ml-0'
                    disabled={cart.length === 0 || !selectedCustomer}
                >
                    Checkout ({cart.length})
                </Button>
            </div>
            <CheckoutModal
                key={checkoutModalKey}
                isOpen={isCheckoutModalOpen}
                onClose={() => setIsCheckoutModalOpen(false)}
                onComplete={handleCheckoutComplete}
                cart={cart}
                customer={selectedCustomer}
                taxes={taxes}
                subtotal={subtotal}
                totalTax={totalTax}
                totalAmount={totalAmount}
            />
        </div>
    )
}

export default CartSection
