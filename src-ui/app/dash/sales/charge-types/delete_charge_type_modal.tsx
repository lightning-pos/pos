'use client'
import React from 'react'
import { Modal } from '@carbon/react'
import { Scalars } from '@/lib/graphql/graphql'

interface DeleteChargeTypeModalProps {
  isOpen: boolean
  chargeTypeId: Scalars['DbUuid']['input']
  chargeTypeName: string
  onClose: () => void
  onDelete: () => void
}

const DeleteChargeTypeModal: React.FC<DeleteChargeTypeModalProps> = ({
    isOpen,
    chargeTypeId,
    chargeTypeName,
    onClose,
    onDelete
}) => {
    return (
        <Modal
            open={isOpen}
            modalHeading="Delete Charge Type"
            primaryButtonText="Delete"
            secondaryButtonText="Cancel"
            onRequestSubmit={onDelete}
            onRequestClose={onClose}
            danger
        >
            <p className="mb-2">
        Are you sure you want to delete the charge type <strong>{chargeTypeName}</strong>?
            </p>
            <p>
        This action cannot be undone. If this charge type is used in any sales orders, the deletion will fail.
            </p>
        </Modal>
    )
}

export default DeleteChargeTypeModal
