'use client'

import { useState } from 'react'
import { Modal, InlineLoading } from '@carbon/react'
import { gql } from '@/lib/graphql/execute'
import { DeleteCostCenterDocument, CostCenterState, CostCenter } from '@/lib/graphql/graphql'

interface DeleteCostCenterModalProps {
    isOpen: boolean;
    costCenter: CostCenter;
    onClose: () => void;
    onDelete: () => void;
}

export default function DeleteCostCenterModal({
    isOpen,
    costCenter,
    onClose,
    onDelete,
}: DeleteCostCenterModalProps) {
    const [loading, setLoading] = useState(false)
    const [error, setError] = useState<string | null>(null)

    const handleDelete = async () => {
        try {
            setLoading(true)
            setError(null)

            await gql(DeleteCostCenterDocument, {
                id: costCenter.id,
            })

            // Notify parent component
            onDelete()
        } catch (err) {
            console.error('Error deleting cost center:', err)
            setError('Failed to delete cost center. Please try again.')
            setLoading(false)
        }
    }

    return (
        <Modal
            open={isOpen}
            modalHeading="Delete Cost Center"
            primaryButtonText="Delete"
            secondaryButtonText="Cancel"
            onRequestClose={onClose}
            onRequestSubmit={handleDelete}
            primaryButtonDisabled={loading}
            danger
        >
            {error && (
                <div className="bg-red-100 border-l-4 border-red-500 text-red-700 p-4 mb-4">
                    {error}
                </div>
            )}

            <p className="mb-4">
                Are you sure you want to delete the cost center <strong>{costCenter.name}</strong> ({costCenter.code})?
            </p>

            <p className="mb-4">
                This action cannot be undone. All data associated with this cost center will be permanently removed.
            </p>

            {loading && (
                <div className="mt-4">
                    <InlineLoading description="Deleting cost center..." />
                </div>
            )}
        </Modal>
    )
}
