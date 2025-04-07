'use client'
import React from 'react'
import { Modal } from '@carbon/react'
import { VariantType } from '@/lib/graphql/graphql'

interface DeleteVariantTypeModalProps {
    open: boolean
    onRequestClose: () => void
    onDelete: () => void
    variantType: VariantType
}

const DeleteVariantTypeModal: React.FC<DeleteVariantTypeModalProps> = ({
    open,
    onRequestClose,
    onDelete,
    variantType,
}) => {
    return (
        <Modal
            open={open}
            modalHeading="Delete Variant Type"
            primaryButtonText="Delete"
            secondaryButtonText="Cancel"
            danger
            onRequestSubmit={onDelete}
            onRequestClose={onRequestClose}
        >
            <p>
                Are you sure you want to delete the variant type &quot;{variantType.name}&quot;?
                This action cannot be undone.
            </p>
            <p className="mt-4">
                Note: You cannot delete a variant type that has variant values associated with it.
                Please delete all variant values first.
            </p>
        </Modal>
    )
}

export default DeleteVariantTypeModal
