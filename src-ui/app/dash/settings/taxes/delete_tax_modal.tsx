import React from 'react'
import { Modal, ModalProps } from '@carbon/react'
import { invoke } from '@tauri-apps/api/core'

interface Tax {
  id: string
  name: string
  rate: number
  description?: string
  createdAt: string
  updatedAt: string
}

interface DeleteTaxModalProps extends ModalProps {
  tax: Tax
}

const DeleteTaxModal: React.FC<DeleteTaxModalProps> = ({
  open,
  onRequestClose,
  onRequestSubmit,
  tax
}) => {
  const handleDeleteTax = async (e: React.FormEvent<HTMLFormElement>) => {
    if (!tax?.id) return
    try {
      await invoke('graphql', {
        query: `#graphql
          mutation {
            deleteTax(id: "${tax.id}")
          }
        `
      })
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
