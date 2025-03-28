import React from 'react'
import { Modal, TextInput, Form } from '@carbon/react'
import { Supplier } from '@/lib/graphql/graphql'

interface AddSupplierModalProps {
    isOpen: boolean
    supplier: Partial<Supplier>
    onClose: () => void
    onSave: () => void
    setSupplier: React.Dispatch<React.SetStateAction<Partial<Supplier>>>
}

const AddSupplierModal: React.FC<AddSupplierModalProps> = ({
    isOpen,
    supplier,
    onClose,
    onSave,
    setSupplier
}) => {
    return (
        <Modal
            open={isOpen}
            modalHeading="Add New Supplier"
            primaryButtonText="Add"
            secondaryButtonText="Cancel"
            onRequestSubmit={onSave}
            onRequestClose={onClose}
        >
            <Form>
                <TextInput
                    id="name"
                    labelText="Name"
                    value={supplier?.name || ''}
                    onChange={(e) => setSupplier(prev => ({ ...prev, name: e.target.value }))}
                    required
                />
                <TextInput
                    id="address"
                    labelText="Address"
                    value={supplier?.address || ''}
                    onChange={(e) => setSupplier(prev => ({ ...prev, address: e.target.value }))}
                />
                <TextInput
                    id="phone"
                    labelText="Phone"
                    value={supplier?.phone || ''}
                    onChange={(e) => setSupplier(prev => ({ ...prev, phone: e.target.value }))}
                />
            </Form>
        </Modal>
    )
}

export default AddSupplierModal
