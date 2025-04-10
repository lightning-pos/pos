'use client'

import { useState } from 'react'
import { Modal, InlineLoading } from '@carbon/react'
import { gql } from '@/lib/graphql/execute'
import { DeletePaymentMethodDocument, PaymentMethod } from '@/lib/graphql/graphql'

interface DeletePaymentMethodModalProps {
    isOpen: boolean;
    paymentMethod: PaymentMethod;
    onClose: () => void;
    onDelete: () => void;
}

export default function DeletePaymentMethodModal({ isOpen, paymentMethod, onClose, onDelete }: DeletePaymentMethodModalProps) {
    const [loading, setLoading] = useState(false)
    const [error, setError] = useState<string | null>(null)

    const handleDelete = async () => {
        try {
            setLoading(true)
            setError(null)

            await gql(DeletePaymentMethodDocument, { id: paymentMethod.id })

            onDelete()
        } catch (err) {
            console.error('Error deleting payment method:', err)
            setError('Failed to delete payment method. It may be in use in sales or purchase transactions.')
            setLoading(false)
        }
    }

    return (
        <Modal
            open={isOpen}
            modalHeading="Delete Payment Method"
            primaryButtonText="Delete"
            secondaryButtonText="Cancel"
            primaryButtonDisabled={loading}
            danger
            onRequestClose={onClose}
            onRequestSubmit={handleDelete}
        >
            {error && (
                <div className="bg-red-100 border-l-4 border-red-500 text-red-700 p-4 mb-4">
                    {error}
                </div>
            )}

            <p>
                Are you sure you want to delete the payment method <strong>{paymentMethod.name}</strong>?
            </p>

            <p className="mt-4">
                This action cannot be undone. If this payment method is used in any transactions, this operation will fail.
            </p>

            {loading && (
                <div className="mt-4">
                    <InlineLoading description="Deleting payment method..." />
                </div>
            )}
        </Modal>
    )
}
