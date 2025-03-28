import React from 'react'
import { Modal } from '@carbon/react'
import { Scalars } from '@/lib/graphql/graphql'

interface DeleteSupplierModalProps {
    isOpen: boolean
    supplierId: Scalars['DbUuid']['input']
    supplierName: string
    onClose: () => void
    onDelete: () => void
}

const DeleteSupplierModal: React.FC<DeleteSupplierModalProps> = ({
    isOpen,
    supplierId,
    supplierName,
    onClose,
    onDelete
}) => {
    return (
        <Modal
            open={isOpen}
            modalHeading="Delete Supplier"
            primaryButtonText="Delete"
            secondaryButtonText="Cancel"
            onRequestSubmit={onDelete}
            onRequestClose={onClose}
            danger
        >
            <p>Are you sure you want to delete supplier {supplierName}?</p>
            <p>This action cannot be undone.</p>
        </Modal>
    )
}

export default DeleteSupplierModal
