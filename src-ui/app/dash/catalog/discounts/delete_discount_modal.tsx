import React from 'react'
import { Modal, ModalProps } from '@carbon/react'

interface DeleteDiscountModalProps {
    isOpen: boolean
    onClose: () => void
    onDelete: () => void
    discountName: string
}

const DeleteDiscountModal: React.FC<DeleteDiscountModalProps> = ({
    isOpen,
    onClose,
    onDelete,
    discountName,
}) => {
    return (
        <Modal
            open={isOpen}
            onRequestClose={onClose}
            modalHeading="Delete Discount"
            primaryButtonText="Delete"
            secondaryButtonText="Cancel"
            onRequestSubmit={onDelete}
            onSecondarySubmit={onClose}
            danger
        >
            <p className="mb-5">
                Are you sure you want to delete the discount <strong>{discountName}</strong>?
                This action cannot be undone.
            </p>
        </Modal>
    )
}

export default DeleteDiscountModal
