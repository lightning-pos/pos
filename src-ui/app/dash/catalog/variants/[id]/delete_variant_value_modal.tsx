'use client'
import React from 'react'
import { Modal } from '@carbon/react'
import { VariantValue } from '@/lib/graphql/graphql'

interface DeleteVariantValueModalProps {
    open: boolean
    onRequestClose: () => void
    onDelete: () => void
    variantValue: VariantValue
}

const DeleteVariantValueModal: React.FC<DeleteVariantValueModalProps> = ({
    open,
    onRequestClose,
    onDelete,
    variantValue,
}) => {
    return (
        <Modal
            open={open}
            modalHeading="Delete Variant Value"
            primaryButtonText="Delete"
            secondaryButtonText="Cancel"
            danger
            onRequestSubmit={onDelete}
            onRequestClose={onRequestClose}
        >
            <p>
                Are you sure you want to delete the variant value &quot;{variantValue.value}&quot;?
                This action cannot be undone.
            </p>
            <p className="mt-4">
                Note: You cannot delete a variant value that is used by any item variants.
                Please remove the variant value from all item variants first.
            </p>
        </Modal>
    )
}

export default DeleteVariantValueModal
