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
    const [editingTax, setEditingTax] = useState<Omit<Tax, 'rate'> & { rate: string } | null>(null)

    useEffect(() => {
        if (tax) {
            setEditingTax({
                ...tax,
                rate: tax.rate.toString()
            })
        }
    }, [tax])

    const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const { name, value } = e.target
        if (!editingTax) return

        if (name === 'rate') {
            // Only allow numbers and decimal point
            const sanitizedValue = value.replace(/[^0-9.]/g, '')
            // Ensure only one decimal point
            const parts = sanitizedValue.split('.')
            const finalValue = parts.length > 2 ? parts[0] + '.' + parts.slice(1).join('') : sanitizedValue
            setEditingTax(prev => prev ? { ...prev, rate: finalValue } : null)
        } else {
            setEditingTax(prev => prev ? { ...prev, [name]: value } : null)
        }
    }

    const handleSaveTax = async (e: React.MouseEvent<HTMLElement>) => {
        if (!editingTax) return

        try {
            const rateAsNumber = parseFloat(editingTax.rate || '0')
            if (isNaN(rateAsNumber)) {
                console.error('Invalid tax rate')
                return
            }

            const input: TaxUpdateInput = {
                id: editingTax.id,
                name: editingTax.name,
                rate: Math.round(rateAsNumber * 100), // Convert decimal to integer percentage
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
                    value={editingTax.rate}
                    onChange={handleInputChange}
                    required
                    placeholder="e.g. 18.5"
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
