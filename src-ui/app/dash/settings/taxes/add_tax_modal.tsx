import React, { useState } from 'react'
import { Modal, TextInput, Form, ModalProps } from '@carbon/react'
import { CreateTaxDocument, TaxNewInput } from '@/lib/graphql/graphql'
import { gql } from '@/lib/graphql/execute'
import { sanitizeDecimalInput } from '@/lib/util/number_format'

const AddTaxModal: React.FC<ModalProps> = ({
    open,
    onRequestSubmit,
    onRequestClose,
}) => {
    const [newTax, setNewTax] = useState<TaxNewInput>({
        name: '',
        rate: '0',
        description: ''
    })

    const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const { name, value } = e.target
        if (name === 'rate') {
            const finalValue = sanitizeDecimalInput(value, 4)
            setNewTax(prev => ({ ...prev, rate: finalValue }))
        } else {
            setNewTax(prev => ({ ...prev, [name]: value }))
        }
    }

    const handleSaveTax = async (e: React.MouseEvent<HTMLElement>) => {
        try {
            await gql(CreateTaxDocument, { input: newTax })
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
