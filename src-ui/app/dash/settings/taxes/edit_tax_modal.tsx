import React, { useState, useEffect } from 'react'
import { Modal, TextInput, Form, ModalProps } from '@carbon/react'
import { invoke } from '@tauri-apps/api/core'

interface Tax {
    id: string
    name: string
    rate: number
    description?: string
    createdAt: string
    updatedAt: string
}

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

    const handleSaveTax = async (e: React.FormEvent) => {
        e.preventDefault()
        if (!editingTax) return
        try {
            await invoke('graphql', {
                query: `#graphql
          mutation {
            updateTax(
              input: {
                id: "${editingTax.id}",
                name: "${editingTax.name}",
                rate: ${Math.round(editingTax.rate * 100)},
                description: "${editingTax.description || ''}"
              }
            ) {
              id
              name
              rate
              description
            }
          }
        `
            })
            onRequestSubmit?.(e as React.FormEvent<HTMLFormElement>)
        } catch (error) {
            console.error('Error updating tax:', error)
        }
    }

    if (!editingTax) return null

    return (
        <Modal
            open={open}
            onRequestClose={onRequestClose}
            modalHeading="Edit Tax"
            primaryButtonText="Save Changes"
            onRequestSubmit={handleSaveTax}
        >
            <Form onSubmit={handleSaveTax} className='flex flex-col gap-4'>
                <TextInput
                    id="tax-name"
                    name="name"
                    labelText="Tax Name"
                    value={editingTax.name}
                    onChange={handleInputChange}
                    required
                />
                <TextInput
                    id="tax-rate"
                    name="rate"
                    labelText="Tax Rate (%)"
                    type="number"
                    value={editingTax.rate}
                    onChange={handleInputChange}
                    required
                />
                <TextInput
                    id="tax-description"
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
