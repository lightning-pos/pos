import React from 'react'
import { Modal, TextInput, Form } from '@carbon/react'
import { Customer } from '@/lib/powersync/app_schema'

interface SaveCustomerModalProps {
  isOpen: boolean
  editingCustomer: Partial<Customer> | null
  onClose: () => void
  onSave: () => void
  onCustomerChange: (customer: Partial<Customer>) => void
}

const SaveCustomerModal: React.FC<SaveCustomerModalProps> = ({
  isOpen,
  editingCustomer,
  onClose,
  onSave,
  onCustomerChange,
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
          onChange={(e) => onCustomerChange({ ...editingCustomer, name: e.target.value })}
          required
        />
        <TextInput
          id="email"
          labelText="Email"
          value={editingCustomer?.email || ''}
          onChange={(e) => onCustomerChange({ ...editingCustomer, email: e.target.value })}
          required
        />
        <TextInput
          id="phone_number"
          labelText="Phone Number"
          value={editingCustomer?.phone_number || ''}
          onChange={(e) => onCustomerChange({ ...editingCustomer, phone_number: e.target.value })}
          required
        />
        <TextInput
          id="country_code"
          labelText="Country Code"
          value={editingCustomer?.country_code || ''}
          onChange={(e) => onCustomerChange({ ...editingCustomer, country_code: e.target.value })}
          required
        />
      </Form>
    </Modal>
  )
}

export default SaveCustomerModal
