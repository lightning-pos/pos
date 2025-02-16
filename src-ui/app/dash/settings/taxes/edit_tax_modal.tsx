import React, { useState, useEffect } from 'react'
import { Modal, TextInput, Form, ModalProps } from '@carbon/react'
import { UpdateTaxDocument, Tax, TaxUpdateInput } from '@/lib/graphql/graphql'
import { gql } from '@/lib/graphql/execute'

interface EditTaxModalProps extends ModalProps {
    tax: Tax
}

const EditTaxModal: React.FC<EditTaxModalProps> = ({
    open,
    onRequestClose,
    onRequestSubmit,
    tax
}) => {
    const [editingTax, setEditingTax] = useState<Tax | null>(null)

    useEffect(() => {
        setEditingTax(tax)
    }, [tax])

    const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const { name, value } = e.target
        setEditingTax(prev => prev ? { ...prev, [name]: name === 'rate' ? parseFloat(value) : value } : null)
    }

    const handleSaveTax = async (e: React.MouseEvent<HTMLElement>) => {
        if (!editingTax) return

        try {
            const input: TaxUpdateInput = {
                id: editingTax.id,
                name: editingTax.name,
                rate: Math.round(editingTax.rate * 100), // Convert decimal to integer percentage
                description: editingTax.description || undefined
            }

            await gql(UpdateTaxDocument, { input })
            onRequestSubmit?.(e)
        } catch (error) {
            console.error('Error updating tax:', error)
        }
    }

    if (!editingTax) return null

    return (
        <Modal
            open={open}
            modalHeading="Edit Tax"
            primaryButtonText="Update"
            secondaryButtonText="Cancel"
            onRequestSubmit={handleSaveTax}
            onRequestClose={onRequestClose}
        >
            <Form>
                <TextInput
                    id="name"
                    name="name"
                    labelText="Name"
                    value={editingTax.name}
                    onChange={handleInputChange}
                    required
                />
                <TextInput
                    id="rate"
                    name="rate"
                    labelText="Rate (%)"
                    type="number"
                    value={editingTax.rate}
                    onChange={handleInputChange}
                    required
                />
                <TextInput
                    id="description"
                    name="description"
                    labelText="Description"
                    value={editingTax.description || ''}
                    onChange={handleInputChange}
                />
            </Form>
        </Modal>
    )
}

export default EditTaxModal
