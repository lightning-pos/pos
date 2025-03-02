'use client'
import React, { useState, useEffect } from 'react'
import { Button } from '@carbon/react'
import { ShoppingCart, Close } from '@carbon/icons-react'
import { gql } from '@/lib/graphql/execute'
import {
    GetPosTaxesDocument,
    Item,
    Tax,
    Customer,
    SalesOrderState,
    SalesOrderNewInput,
    CreateSalesOrderDocument
} from '@/lib/graphql/graphql'
import CheckoutModal from './checkout_modal'
import CustomerSelect from './customer_select'
import CartItem from './cart_item'
import { formatCurrency } from '@/lib/util/number_format'

export interface CartItem extends Omit<Item, 'category' | 'taxes'> {
    quantity: number
    taxIds?: string[]
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

    const handleCheckout = () => {
        if (!selectedCustomer) {
            alert('Please select a customer before checkout')
            return
        }
        setCheckoutModalKey(prev => prev + 1)
        setIsCheckoutModalOpen(true)
    }

    const handleCheckoutComplete = async () => {
        if (!selectedCustomer) return

        try {
            const now = new Date();
            const orderDate = now.getUTCFullYear() + '-' +
                String(now.getUTCMonth() + 1).padStart(2, '0') + '-' +
                String(now.getUTCDate()).padStart(2, '0') + ' ' +
                String(now.getUTCHours()).padStart(2, '0') + ':' +
                String(now.getUTCMinutes()).padStart(2, '0') + ':' +
                String(now.getUTCSeconds()).padStart(2, '0')

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
                state: SalesOrderState.Completed,
                items: cart.map(item => ({
                    itemId: item.id,
                    itemName: item.name,
                    quantity: item.quantity,
                    priceAmount: item.price.toString(),
                    taxAmount: item.taxIds
                        ? taxes
                            .filter(tax => item.taxIds?.includes(tax.id))
                            .reduce((sum, tax) => sum + (parseFloat(item.price) * parseFloat(tax.rate) / 100), 0)
                            .toString()
                        : '0',
                    totalAmount: ((parseFloat(item.price) + (item.taxIds
                        ? taxes
                            .filter(tax => item.taxIds?.includes(tax.id))
                            .reduce((sum, tax) => sum + (parseFloat(item.price) * parseFloat(tax.rate) / 100), 0)
                        : 0)) * item.quantity).toString(),
                }))
            }

            await gql(CreateSalesOrderDocument, { salesOrder: orderInput })
            setCart([])
            setIsCheckoutModalOpen(false)
            setSelectedCustomer(null)
        } catch (error) {
            console.error('Error creating order:', error)
            alert('Failed to create order. Please try again.')
        }
    }

    return (
        <div className='flex flex-col h-[calc(100dvh-4rem)]'>
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
