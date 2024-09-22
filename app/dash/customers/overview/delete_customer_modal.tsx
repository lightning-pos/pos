import React from 'react'
import { Modal } from '@carbon/react'
import { useCustomers } from './customers_context'

const DeleteCustomerModal: React.FC = () => {
  const {
    isDeleteModalOpen,
    editingCustomer,
    setIsDeleteModalOpen,
    setEditingCustomer,
    handleDeleteCustomer
  } = useCustomers()

  if (!editingCustomer) return null

  return (
    <Modal
      open={isDeleteModalOpen}
      onRequestClose={() => {
        setIsDeleteModalOpen(false)
        setEditingCustomer(null)
      }}
      modalHeading="Delete Customer"
      primaryButtonText="Delete"
      secondaryButtonText="Cancel"
      danger
      onSecondarySubmit={() => {
        setIsDeleteModalOpen(false)
        setEditingCustomer(null)
      }}
      onRequestSubmit={() => handleDeleteCustomer(editingCustomer.id as string)}
    >
      <p>Are you sure you want to delete the customer &quot;{editingCustomer.name}&quot;? This action cannot be undone.</p>
    </Modal>
  )
}

export default DeleteCustomerModal
