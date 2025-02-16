import React from 'react'
import { Modal, TextInput, Form } from '@carbon/react'
import { Customer } from '@/lib/graphql/graphql'

interface EditCustomerModalProps {
    isOpen: boolean
    customer: Customer | null
    onClose: () => void
    onSave: () => void
    setCustomer: React.Dispatch<React.SetStateAction<Customer | null>>
}

const EditCustomerModal: React.FC<EditCustomerModalProps> = ({
    isOpen,
    customer,
    onClose,
    onSave,
    setCustomer
}) => {
    return (
        <Modal
            open={isOpen}
            modalHeading="Edit Customer"
            primaryButtonText="Update"
            secondaryButtonText="Cancel"
            onRequestSubmit={onSave}
            onRequestClose={onClose}
        >
            <Form>
                <TextInput
                    id="name"
                    labelText="Name"
                    value={customer?.fullName || ''}
                    onChange={(e) => setCustomer(prev => prev ? { ...prev, fullName: e.target.value } : null)}
                    required
                />
                <TextInput
                    id="email"
                    labelText="Email"
                    value={customer?.email || ''}
                    onChange={(e) => setCustomer(prev => prev ? { ...prev, email: e.target.value } : null)}
                />
                <TextInput
                    id="phone"
                    labelText="Phone"
                    value={customer?.phone || ''}
                    onChange={(e) => setCustomer(prev => prev ? { ...prev, phone: e.target.value } : null)}
                />
                <TextInput
                    id="address"
                    labelText="Address"
                    value={customer?.address || ''}
                    onChange={(e) => setCustomer(prev => prev ? { ...prev, address: e.target.value } : null)}
                />
            </Form>
        </Modal>
    )
}

export default EditCustomerModal
