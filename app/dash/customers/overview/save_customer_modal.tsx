import React from 'react'
import { Modal, TextInput, Form } from '@carbon/react'
import { useCustomers } from './customers_context'

const SaveCustomerModal: React.FC = () => {
  const {
    isModalOpen,
    editingCustomer,
    setIsModalOpen,
    setEditingCustomer,
    handleAddOrUpdateCustomer
  } = useCustomers()

  return (
    <Modal
      open={isModalOpen}
      modalHeading={editingCustomer?.id ? "Edit Customer" : "Add New Customer"}
      primaryButtonText={editingCustomer?.id ? "Update" : "Add"}
      secondaryButtonText="Cancel"
      onRequestSubmit={handleAddOrUpdateCustomer}
      onRequestClose={() => {
        setIsModalOpen(false)
        setEditingCustomer(null)
      }}
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
