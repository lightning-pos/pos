import React from 'react'
import { Modal, ModalProps } from '@carbon/react'
import { Tax, taxesTable } from '@/lib/db/sqlite/schema'
import { useDb } from '@/components/providers/drizzle_provider'
import { eq } from 'drizzle-orm'

interface DeleteTaxModalProps extends ModalProps {
  tax: Tax
}

const DeleteTaxModal: React.FC<DeleteTaxModalProps> = ({
  open,
  onRequestClose,
  onRequestSubmit,
  tax
}) => {
  const db = useDb()

  const handleDeleteTax = async (e: React.FormEvent<HTMLFormElement>) => {
    if (!tax?.id) return
    try {
      await db.delete(taxesTable)
        .where(eq(taxesTable.id, tax.id))
      onRequestSubmit?.(e as React.FormEvent<HTMLFormElement>)
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
      <p>Are you sure you want to delete the tax &quot;{tax.name}&quot;? This action cannot be undone.</p>
    </Modal>
  )
}

export default DeleteTaxModal
