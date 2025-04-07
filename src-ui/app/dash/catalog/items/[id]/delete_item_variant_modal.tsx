'use client'
import React from 'react'
import { Modal } from '@carbon/react'
import { ItemVariant } from '@/lib/graphql/graphql'

interface DeleteItemVariantModalProps {
    open: boolean
    onRequestClose: () => void
    onDelete: () => void
    itemVariant: ItemVariant
}

const DeleteItemVariantModal: React.FC<DeleteItemVariantModalProps> = ({
    open,
    onRequestClose,
    onDelete,
    itemVariant,
}) => {
    return (
        <Modal
            open={open}
            modalHeading="Delete Item Variant"
            primaryButtonText="Delete"
            secondaryButtonText="Cancel"
            danger
            onRequestSubmit={onDelete}
            onRequestClose={onRequestClose}
        >
            <p>
                Are you sure you want to delete this item variant
                {itemVariant.sku ? ` (${itemVariant.sku})` : ''}?
                This action cannot be undone.
            </p>
            {itemVariant.isDefault && (
                <p className="mt-4 text-red-600">
                    Warning: This is the default variant for this item. Deleting it may affect how this item appears in the system.
                </p>
            )}
        </Modal>
    )
}

export default DeleteItemVariantModal
