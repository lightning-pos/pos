import React from 'react'
import { Modal, TextInput, Form } from '@carbon/react'
import { Supplier } from '@/lib/graphql/graphql'

interface EditSupplierModalProps {
    isOpen: boolean
    supplier: Supplier | null
    onClose: () => void
    onSave: () => void
    setSupplier: React.Dispatch<React.SetStateAction<Supplier | null>>
}

const EditSupplierModal: React.FC<EditSupplierModalProps> = ({
    isOpen,
    supplier,
    onClose,
    onSave,
    setSupplier
}) => {
    return (
        <Modal
            open={isOpen}
            modalHeading="Edit Supplier"
            primaryButtonText="Update"
            secondaryButtonText="Cancel"
            onRequestSubmit={onSave}
            onRequestClose={onClose}
        >
            <Form>
                <TextInput
                    id="name"
                    labelText="Name"
                    value={supplier?.name || ''}
                    onChange={(e) => setSupplier(prev => prev ? { ...prev, name: e.target.value } : null)}
                    required
                />
                <TextInput
                    id="address"
                    labelText="Address"
                    value={supplier?.address || ''}
                    onChange={(e) => setSupplier(prev => prev ? { ...prev, address: e.target.value } : null)}
                />
                <TextInput
                    id="phone"
                    labelText="Phone"
                    value={supplier?.phone || ''}
                    onChange={(e) => setSupplier(prev => prev ? { ...prev, phone: e.target.value } : null)}
                />
            </Form>
        </Modal>
    )
}

export default EditSupplierModal
