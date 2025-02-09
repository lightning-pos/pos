import React, { useState } from 'react'
import { Modal, TextInput, Form, ModalProps } from '@carbon/react'
import { invoke } from '@tauri-apps/api/core'

interface NewTax {
    name: string;
    rate: number;
    description?: string;
}

const AddTaxModal: React.FC<ModalProps> = ({
    open,
    onRequestSubmit,
    onRequestClose,
}) => {
    const [newTax, setNewTax] = useState<NewTax>({
        name: '',
        rate: 0,
        description: ''
    })

    const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const { name, value } = e.target
        setNewTax(prev => ({ ...prev, [name]: name === 'rate' ? parseFloat(value) : value }))
    }

    const handleSaveTax = async (e: React.FormEvent) => {
        e.preventDefault()
        try {
            await invoke('graphql', {
                query: `#graphql
                mutation {
                    createTax(
                        input: {
                            name: "${newTax.name}",
                            rate: ${Math.round(newTax.rate * 100)},
                            description: "${newTax.description || ''}"
                        }
                    ) {
                        id
                        name
                        rate
                        description
                    }
                }`
            })
            onRequestSubmit?.(e as React.FormEvent<HTMLFormElement>)
        } catch (error) {
            console.error('Error saving tax:', error)
        }
    }

    return (
        <Modal
            open={open}
            onRequestClose={onRequestClose}
            modalHeading="Add New Tax"
            primaryButtonText="Save"
            onRequestSubmit={handleSaveTax}
        >
            <Form onSubmit={handleSaveTax} className='flex flex-col gap-4'>
                <TextInput
                    id="tax-name"
                    name="name"
                    labelText="Tax Name"
                    value={newTax.name}
                    onChange={handleInputChange}
                    required
                />
                <TextInput
                    id="tax-rate"
                    name="rate"
                    labelText="Tax Rate (%)"
                    type="number"
                    value={newTax.rate || ''}
                    onChange={handleInputChange}
                    required
                />
                <TextInput
                    id="tax-description"
                    name="description"
                    labelText="Description"
                    value={newTax.description || ''}
                    onChange={handleInputChange}
                />
            </Form>
        </Modal>
    )
}

export default AddTaxModal
