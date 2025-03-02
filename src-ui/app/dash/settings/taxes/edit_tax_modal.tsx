import React, { useState, useEffect } from 'react'
import { Modal, TextInput, Form, ModalProps } from '@carbon/react'
import { UpdateTaxDocument, Tax, TaxUpdateInput } from '@/lib/graphql/graphql'
import { gql } from '@/lib/graphql/execute'
import { sanitizeDecimalInput } from '@/lib/util/number_format'

interface EditTaxModalProps extends ModalProps {
    tax: Tax
}

const EditTaxModal: React.FC<EditTaxModalProps> = ({
    open,
    onRequestClose,
    onRequestSubmit,
    tax
}) => {
    const [editingTax, setEditingTax] = useState<TaxUpdateInput | null>(null)

    useEffect(() => {
        if (tax) {
            setEditingTax({
                id: tax.id,
                name: tax.name,
                rate: tax.rate.toString(),
                description: tax.description || undefined
            })
        }
    }, [tax])

    const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const { name, value } = e.target
        if (!editingTax) return

        if (name === 'rate') {
            const finalValue = sanitizeDecimalInput(value, 4)
            setEditingTax(prev => prev ? { ...prev, rate: finalValue } : null)
        } else {
            setEditingTax(prev => prev ? { ...prev, [name]: value } : null)
        }
    }

    const handleSaveTax = async (e: React.MouseEvent<HTMLElement>) => {
        if (!editingTax) return

        try {
            await gql(UpdateTaxDocument, { input: editingTax })
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
                    value={editingTax.name || ''}
                    onChange={handleInputChange}
                    required
                />
                <TextInput
                    id="rate"
                    name="rate"
                    labelText="Rate (%)"
                    value={editingTax.rate || ''}
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
