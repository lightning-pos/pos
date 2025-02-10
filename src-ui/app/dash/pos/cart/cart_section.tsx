import React, { useState, useEffect } from 'react'
import { Button } from '@carbon/react'
import { ShoppingCart, Close } from '@carbon/icons-react'
import { invoke } from '@tauri-apps/api/core'
import CheckoutModal from './checkout_modal'
import CustomerSelect from './customer_select'
import CartItem from './cart_item'

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

interface Item {
    id: string
    name: string
    description?: string
    price: number
    nature: 'GOODS' | 'SERVICE'
    state: 'ACTIVE' | 'INACTIVE' | 'DELETED'
}

export interface CartItem extends Item {
    quantity: number;
    taxIds?: string[];
}

interface CartSectionProps {
    cart: CartItem[]
    setCart: React.Dispatch<React.SetStateAction<CartItem[]>>
}

const formatPrice = (price: number): string => {
    return new Intl.NumberFormat('en-IN', {
        style: 'currency',
        currency: 'INR'
    }).format(price / 100);
};

const CartSection: React.FC<CartSectionProps> = ({ cart, setCart }) => {
    const [isCheckoutModalOpen, setIsCheckoutModalOpen] = useState(false)
    const [checkoutModalKey, setCheckoutModalKey] = useState(0)
    const [taxes, setTaxes] = useState<Tax[]>([])
    const [selectedCustomer, setSelectedCustomer] = useState<Customer | null>(null)

    useEffect(() => {
        const fetchTaxes = async () => {
            try {
                const result = await invoke('graphql', {
                    query: `#graphql
            query {
              taxes {
                id
                name
                rate
                description
              }
            }
          `
                }) as { data: { taxes: Tax[] } };
                setTaxes(result[0].taxes)
            } catch (error) {
                console.error('Error fetching taxes:', error)
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
            const itemTaxes = taxes.filter(tax => item.taxIds?.some(t => t === tax.id))
            const itemPrice = item.price || 0
            const itemTax = itemTaxes.reduce((taxSum, tax) => {
                const taxRate = (tax.rate || 0) / 10000 // Convert from basis points to decimal
                return taxSum + (itemPrice * item.quantity * taxRate)
            }, 0)
            return sum + itemTax
        }, 0)
    }

    const subtotal = cart.reduce((sum, item) => {
        return sum + ((item.price || 0) * item.quantity)
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

    const handleCheckoutComplete = () => {
        setCart([])
        setIsCheckoutModalOpen(false)
        setSelectedCustomer(null)
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
                    <span>{formatPrice(subtotal)}</span>
                </div>
                <div className='flex justify-between items-center'>
                    <span>Tax:</span>
                    <span>{formatPrice(totalTax)}</span>
                </div>
                <div className='flex justify-between items-center font-bold'>
                    <span>Total:</span>
                    <span>{formatPrice(totalAmount)}</span>
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
                open={isCheckoutModalOpen}
                onRequestClose={() => setIsCheckoutModalOpen(false)}
                onRequestSubmit={handleCheckoutComplete}
                cart={cart}
                subtotal={subtotal}
                tax={totalTax}
                total={totalAmount}
                customer={selectedCustomer}
            />
        </div>
    )
}

export default CartSection
