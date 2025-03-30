'use client'
import React from 'react'
import { Modal, Form, TextInput, TextArea, Toggle } from '@carbon/react'
import { Brand } from '@/lib/graphql/graphql'

interface AddBrandModalProps {
    isOpen: boolean
    brand: Partial<Brand>
    onClose: () => void
    setBrand: React.Dispatch<React.SetStateAction<Partial<Brand>>>
    onSave: () => void
}

const AddBrandModal: React.FC<AddBrandModalProps> = ({
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
            modalHeading="Add Brand"
            primaryButtonText="Create"
            secondaryButtonText="Cancel"
            onRequestSubmit={handleSubmit}
            onRequestClose={onClose}
        >
            <Form onSubmit={handleSubmit}>
                <div className="space-y-6">
                    <TextInput
                        id="name"
                        labelText="Brand Name"
                        value={brand.name || ''}
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
                        toggled={brand.isActive ?? true}
                        onToggle={() => setBrand({ ...brand, isActive: !(brand.isActive ?? true) })}
                    />
                </div>
            </Form>
        </Modal>
    )
}

export default AddBrandModal
