import React from 'react'
import { Modal } from '@carbon/react'

interface DeleteCategoryModalProps {
  isOpen: boolean
  onClose: () => void
  onDelete: () => Promise<void>
  categoryName: string
}

const DeleteCategoryModal: React.FC<DeleteCategoryModalProps> = ({
    isOpen,
    onClose,
    onDelete,
    categoryName
}) => {
    return (
        <Modal
            open={isOpen}
            onRequestClose={onClose}
            modalHeading="Delete Category"
            primaryButtonText="Delete"
            secondaryButtonText="Cancel"
            danger
            onRequestSubmit={onDelete}
        >
            <p>Are you sure you want to delete the category &quot;{categoryName}&quot;? This action cannot be undone.</p>
        </Modal>
    )
}

export default DeleteCategoryModal
