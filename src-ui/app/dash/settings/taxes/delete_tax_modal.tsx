import React from 'react'
import { Modal, ModalProps } from '@carbon/react'
import { DeleteTaxDocument, Tax } from '@/lib/graphql/graphql'
import { gql } from '@/lib/graphql/execute'

interface DeleteTaxModalProps extends ModalProps {
  tax: Tax
}

const DeleteTaxModal: React.FC<DeleteTaxModalProps> = ({
  open,
  onRequestClose,
  onRequestSubmit,
  tax
}) => {
  const handleDeleteTax = async (e: React.MouseEvent<HTMLElement>) => {
    try {
      await gql(DeleteTaxDocument, { id: tax.id })
      onRequestSubmit?.(e)
    } catch (error) {
      console.error('Error deleting tax:', error)
    }
  }

  if (!tax) return null

  return (
    <Modal
      open={open}
      onRequestClose={onRequestClose}
      modalHeading="Delete Tax"
      primaryButtonText="Delete"
      secondaryButtonText="Cancel"
      danger
      onRequestSubmit={handleDeleteTax}
    >
      <p>Are you sure you want to delete tax {tax.name}?</p>
      <p>This action cannot be undone.</p>
    </Modal>
  )
}

export default DeleteTaxModal
