'use client'
import React from 'react'
import { Modal, Form, TextInput, TextArea, Toggle } from '@carbon/react'
import { Brand } from '@/lib/graphql/graphql'

interface EditBrandModalProps {
    isOpen: boolean
    brand: Brand
    onClose: () => void
    setBrand: React.Dispatch<React.SetStateAction<Brand | null>>
    onSave: () => void
}

const EditBrandModal: React.FC<EditBrandModalProps> = ({
    isOpen,
    brand,
    onClose,
    setBrand,
    onSave
}) => {
    const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault()
        onSave()
    }

    return (
        <Modal
            open={isOpen}
            modalHeading="Edit Brand"
            primaryButtonText="Save"
            secondaryButtonText="Cancel"
            onRequestSubmit={handleSubmit}
            onRequestClose={onClose}
        >
            <Form onSubmit={handleSubmit}>
                <div className="space-y-6">
                    <TextInput
                        id="name"
                        labelText="Brand Name"
                        value={brand.name}
                        onChange={(e) => setBrand({ ...brand, name: e.target.value })}
                        required
                    />

                    <TextArea
                        id="description"
                        labelText="Description"
                        value={brand.description || ''}
                        onChange={(e) => setBrand({ ...brand, description: e.target.value || null })}
                    />

                    <Toggle
                        id="is_active"
                        labelText="Status"
                        labelA="Inactive"
                        labelB="Active"
                        toggled={brand.isActive}
                        onToggle={() => setBrand({ ...brand, isActive: !brand.isActive })}
                    />
                </div>
            </Form>
        </Modal>
    )
}

export default EditBrandModal
