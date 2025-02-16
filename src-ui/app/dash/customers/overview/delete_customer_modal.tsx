import React from 'react'
import { Modal } from '@carbon/react'
import { Scalars } from '@/lib/graphql/graphql'

interface DeleteCustomerModalProps {
    isOpen: boolean
    customerId: Scalars['DbUuid']['input']
    customerName: string
    onClose: () => void
    onDelete: () => void
}

const DeleteCustomerModal: React.FC<DeleteCustomerModalProps> = ({
    isOpen,
    customerId,
    customerName,
    onClose,
    onDelete
}) => {
    return (
        <Modal
            open={isOpen}
            modalHeading="Delete Customer"
            primaryButtonText="Delete"
            secondaryButtonText="Cancel"
            onRequestSubmit={onDelete}
            onRequestClose={onClose}
            danger
        >
            <p>Are you sure you want to delete customer {customerName}?</p>
            <p>This action cannot be undone.</p>
        </Modal>
    )
}

export default DeleteCustomerModal
