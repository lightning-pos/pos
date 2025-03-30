'use client'
import React from 'react'
import { Modal } from '@carbon/react'

interface DeleteBrandModalProps {
    isOpen: boolean
    brandId: string
    brandName: string
    onClose: () => void
    onDelete: () => void
}

const DeleteBrandModal: React.FC<DeleteBrandModalProps> = ({
    isOpen,
    brandId,
    brandName,
    onClose,
    onDelete
}) => {
    return (
        <Modal
            open={isOpen}
            modalHeading="Delete Brand"
            primaryButtonText="Delete"
            secondaryButtonText="Cancel"
            danger
            onRequestSubmit={onDelete}
            onRequestClose={onClose}
        >
            <p>
                Are you sure you want to delete the brand "{brandName}"? This action cannot be undone.
            </p>
        </Modal>
    )
}

export default DeleteBrandModal
