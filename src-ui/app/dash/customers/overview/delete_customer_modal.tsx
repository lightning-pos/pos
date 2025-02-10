import React from 'react'
import { Modal } from '@carbon/react'

interface Customer {
    id: string
    name: string
    email?: string | null
    phoneNumber?: string | null
    countryCode?: string | null
    createdAt: string
    updatedAt: string
}

interface DeleteCustomerModalProps {
    isOpen: boolean
    editingCustomer: Partial<Customer> | null
    onClose: () => void
    onDelete: () => void
}

const DeleteCustomerModal: React.FC<DeleteCustomerModalProps> = ({
    isOpen,
    editingCustomer,
    onClose,
    onDelete
}) => {
    if (!editingCustomer) return null

    return (
        <Modal
            open={isOpen}
            onRequestClose={onClose}
            modalHeading="Delete Customer"
            primaryButtonText="Delete"
            secondaryButtonText="Cancel"
            danger
            onSecondarySubmit={onClose}
            onRequestSubmit={onDelete}
        >
            <p>Are you sure you want to delete the customer &quot;{editingCustomer.name}&quot;? This action cannot be undone.</p>
        </Modal>
    )
}

export default DeleteCustomerModal
