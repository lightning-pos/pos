'use client'
import React, { useState } from 'react'
import { Modal } from '@carbon/react'
import { DeleteTaxGroupDocument } from '@/lib/graphql/graphql'
import type { TaxGroup } from '@/lib/graphql/graphql'
import { gql } from '@/lib/graphql/execute'

interface DeleteTaxGroupModalProps {
    open: boolean
    taxGroup: TaxGroup
    onRequestClose: () => void
    onRequestSubmit: () => void
}

const DeleteTaxGroupModal: React.FC<DeleteTaxGroupModalProps> = ({
    open,
    taxGroup,
    onRequestClose,
    onRequestSubmit
}) => {
    const [loading, setLoading] = useState(false)
    const [error, setError] = useState('')

    const handleDelete = async () => {
        setLoading(true)
        try {
            await gql(DeleteTaxGroupDocument, { id: taxGroup.id })
            onRequestSubmit()
        } catch (error) {
            console.error('Error deleting tax group:', error)
            setError('Failed to delete tax group. It may be in use by sales orders.')
        } finally {
            setLoading(false)
        }
    }

    return (
        <Modal
            open={open}
            modalHeading="Delete Tax Group"
            primaryButtonText="Delete"
            secondaryButtonText="Cancel"
            primaryButtonDisabled={loading}
            danger
            onRequestClose={onRequestClose}
            onRequestSubmit={handleDelete}
        >
            {error ? (
                <div className="text-red-500 mb-4">{error}</div>
            ) : (
                <p>
                    Are you sure you want to delete the tax group <strong>{taxGroup.name}</strong>?
                    This action cannot be undone.
                </p>
            )}
            {taxGroup.taxes && taxGroup.taxes.length > 0 && (
                <div className="mt-4">
                    <p className="text-amber-600">
                        Warning: This tax group contains {taxGroup.taxes.length} taxes.
                        Deleting this group will remove these tax associations, but the taxes themselves will not be deleted.
                    </p>
                </div>
            )}
        </Modal>
    )
}

export default DeleteTaxGroupModal
