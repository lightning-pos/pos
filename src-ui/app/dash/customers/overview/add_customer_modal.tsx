import React from 'react'
import { Modal, TextInput, Form } from '@carbon/react'
import { Customer } from '@/lib/graphql/graphql'

interface AddCustomerModalProps {
    isOpen: boolean
    customer: Partial<Customer>
    onClose: () => void
    onSave: () => void
    setCustomer: React.Dispatch<React.SetStateAction<Partial<Customer>>>
}

const AddCustomerModal: React.FC<AddCustomerModalProps> = ({
    isOpen,
    customer,
    onClose,
    onSave,
    setCustomer
}) => {
    return (
        <Modal
            open={isOpen}
            modalHeading="Add New Customer"
            primaryButtonText="Add"
            secondaryButtonText="Cancel"
            onRequestSubmit={onSave}
            onRequestClose={onClose}
        >
            <Form>
                <TextInput
                    id="name"
                    labelText="Name"
                    value={customer?.fullName || ''}
                    onChange={(e) => setCustomer(prev => ({ ...prev, fullName: e.target.value }))}
                    required
                />
                <TextInput
                    id="email"
                    labelText="Email"
                    value={customer?.email || ''}
                    onChange={(e) => setCustomer(prev => ({ ...prev, email: e.target.value }))}
                />
                <TextInput
                    id="phone"
                    labelText="Phone"
                    value={customer?.phone || ''}
                    onChange={(e) => setCustomer(prev => ({ ...prev, phone: e.target.value }))}
                />
                <TextInput
                    id="address"
                    labelText="Address"
                    value={customer?.address || ''}
                    onChange={(e) => setCustomer(prev => ({ ...prev, address: e.target.value }))}
                />
            </Form>
        </Modal>
    )
}

export default AddCustomerModal
