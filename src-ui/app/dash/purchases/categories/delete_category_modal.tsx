import React from 'react'
import { Modal } from '@carbon/react'

interface DeletePurchaseCategoryModalProps {
    isOpen: boolean
    onClose: () => void
    onDelete: () => void
    categoryName: string
}

const DeletePurchaseCategoryModal: React.FC<DeletePurchaseCategoryModalProps> = ({
    isOpen,
    onClose,
    onDelete,
    categoryName,
}) => {
    return (
        <Modal
            open={isOpen}
            modalHeading="Delete Purchase Category"
            primaryButtonText="Delete"
            secondaryButtonText="Cancel"
            danger
            onRequestClose={onClose}
            onRequestSubmit={onDelete}
        >
            <p>
                Are you sure you want to delete the purchase category "{categoryName}"? This action cannot be undone.
            </p>
        </Modal>
    )
}

export default DeletePurchaseCategoryModal
