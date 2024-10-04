import React from 'react'
import { Modal, TextInput, Form } from '@carbon/react'
import { Customer } from '@/lib/db/sqlite/schema'

interface SaveCustomerModalProps {
  isOpen: boolean
  editingCustomer: Partial<Customer> | null
  onClose: () => void
  onSave: () => void
  setEditingCustomer: React.Dispatch<React.SetStateAction<Partial<Customer> | null>>
}

const SaveCustomerModal: React.FC<SaveCustomerModalProps> = ({
  isOpen,
  editingCustomer,
  onClose,
  onSave,
  setEditingCustomer
}) => {
  return (
    <Modal
      open={isOpen}
      modalHeading={editingCustomer?.id ? "Edit Customer" : "Add New Customer"}
      primaryButtonText={editingCustomer?.id ? "Update" : "Add"}
      secondaryButtonText="Cancel"
      onRequestSubmit={onSave}
      onRequestClose={onClose}
    >
      <Form>
        <TextInput
          id="name"
          labelText="Name"
          value={editingCustomer?.name || ''}
          onChange={(e) => setEditingCustomer(prev => ({ ...prev, name: e.target.value }))}
          required
        />
        <TextInput
          id="email"
          labelText="Email"
          value={editingCustomer?.email || ''}
          onChange={(e) => setEditingCustomer(prev => ({ ...prev, email: e.target.value }))}
          required
        />
        <TextInput
          id="phone_number"
          labelText="Phone Number"
          value={editingCustomer?.phoneNumber || ''}
          onChange={(e) => setEditingCustomer(prev => ({ ...prev, phoneNumber: e.target.value }))}
          required
        />
        <TextInput
          id="country_code"
          labelText="Country Code"
          value={editingCustomer?.countryCode || ''}
          onChange={(e) => setEditingCustomer(prev => ({ ...prev, countryCode: e.target.value }))}
          required
        />
      </Form>
    </Modal>
  )
}

export default SaveCustomerModal
