import React from 'react'
import { IconButton } from '@carbon/react'
import { Add, Subtract } from '@carbon/icons-react'
import { CartItem as CartItemType } from './cart_section'
import { formatCurrency } from '@/lib/util/number_format'

interface CartItemProps {
    item: CartItemType
    updateQuantity: (itemId: string, change: number) => void
}

const CartItem: React.FC<CartItemProps> = ({ item, updateQuantity }) => {
    const itemTotal = (parseFloat(item.price) || 0) * item.quantity

    return (
        <div className='flex flex-col mb-4'>
            <div className='flex justify-between items-center'>
                <span>{item.name}</span>
                <div className='flex items-center'>
                    <IconButton size='sm' label="Decrease" onClick={() => updateQuantity(item.id, -1)}>
                        <Subtract size={16} />
                    </IconButton>
                    <span className='mx-2'>{item.quantity}</span>
                    <IconButton size='sm' label="Increase" onClick={() => updateQuantity(item.id, 1)}>
                        <Add size={16} />
                    </IconButton>
                    <span className='ml-4'>{formatCurrency(itemTotal)}</span>
                </div>
            </div>
        </div>
    )
}

export default CartItem
