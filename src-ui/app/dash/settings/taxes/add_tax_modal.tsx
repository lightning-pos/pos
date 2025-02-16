import React, { useState } from 'react'
import { Modal, TextInput, Form, ModalProps } from '@carbon/react'
import { CreateTaxDocument, TaxNewInput } from '@/lib/graphql/graphql'
import { gql } from '@/lib/graphql/execute'

const AddTaxModal: React.FC<ModalProps> = ({
    open,
    onRequestSubmit,
    onRequestClose,
}) => {
    const [newTax, setNewTax] = useState<TaxNewInput>({
        name: '',
        rate: 0,
        description: ''
    })

    const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const { name, value } = e.target
        setNewTax(prev => ({ ...prev, [name]: name === 'rate' ? parseFloat(value) : value }))
    }

    const handleSaveTax = async (e: React.MouseEvent<HTMLElement>) => {
        try {
            await gql(CreateTaxDocument, {
                input: {
                    ...newTax,
                    rate: Math.round(newTax.rate * 100) // Convert decimal to integer percentage
                }
            })
            onRequestSubmit?.(e)
        } catch (error) {
            console.error('Error creating tax:', error)
        }
    }

    return (
        <Modal
            open={open}
            modalHeading="Add Tax"
            primaryButtonText="Add"
            secondaryButtonText="Cancel"
            onRequestSubmit={handleSaveTax}
            onRequestClose={onRequestClose}
        >
            <Form>
                <TextInput
                    id="name"
                    name="name"
                    labelText="Name"
                    value={newTax.name}
                    onChange={handleInputChange}
                    required
                />
                <TextInput
                    id="rate"
                    name="rate"
                    labelText="Rate (%)"
                    type="number"
                    value={newTax.rate}
                    onChange={handleInputChange}
                    required
                />
                <TextInput
                    id="description"
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
