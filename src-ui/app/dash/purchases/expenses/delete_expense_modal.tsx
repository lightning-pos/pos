import React from 'react'
import { Modal } from '@carbon/react'
import { Scalars } from '@/lib/graphql/graphql'

interface DeleteExpenseModalProps {
    isOpen: boolean
    expenseId: Scalars['DbUuid']['input']
    expenseTitle: string
    onClose: () => void
    onDelete: () => void
}

const DeleteExpenseModal: React.FC<DeleteExpenseModalProps> = ({
    isOpen,
    expenseId,
    expenseTitle,
    onClose,
    onDelete
}) => {
    return (
        <Modal
            open={isOpen}
            modalHeading="Delete Expense"
            primaryButtonText="Delete"
            secondaryButtonText="Cancel"
            onRequestSubmit={onDelete}
            onRequestClose={onClose}
            danger
        >
            <p>Are you sure you want to delete expense "{expenseTitle}"?</p>
            <p>This action cannot be undone.</p>
        </Modal>
    )
}

export default DeleteExpenseModal
