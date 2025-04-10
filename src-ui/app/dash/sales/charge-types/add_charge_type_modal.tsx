'use client'
import React, { useState } from 'react'
import { Modal, TextInput, Form, TextArea } from '@carbon/react'
import { SalesChargeTypeNewInput } from '@/lib/graphql/graphql'

interface AddChargeTypeModalProps {
  isOpen: boolean
  onClose: () => void
  onSave: (chargeType: SalesChargeTypeNewInput) => void
}

const AddChargeTypeModal: React.FC<AddChargeTypeModalProps> = ({
    isOpen,
    onClose,
    onSave
}) => {
    const [newChargeType, setNewChargeType] = useState<SalesChargeTypeNewInput>({
        name: '',
        description: ''
    })

    const [nameError, setNameError] = useState('')

    const handleInputChange = (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
        const { name, value } = e.target
        setNewChargeType(prev => ({ ...prev, [name]: value }))

        if (name === 'name' && !value.trim()) {
            setNameError('Name is required')
        } else if (name === 'name') {
            setNameError('')
        }
    }

    const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault()

        if (!newChargeType.name.trim()) {
            setNameError('Name is required')
            return
        }

        onSave(newChargeType)

        // Reset form
        setNewChargeType({
            name: '',
            description: ''
        })
        setNameError('')
    }

    return (
        <Modal
            open={isOpen}
            modalHeading="Add Charge Type"
            primaryButtonText="Save"
            secondaryButtonText="Cancel"
            onRequestSubmit={handleSubmit}
            onRequestClose={() => {
                onClose()
                setNewChargeType({ name: '', description: '' })
                setNameError('')
            }}
        >
            <Form onSubmit={handleSubmit}>
                <div className="space-y-4 pt-4">
                    <TextInput
                        id="name"
                        name="name"
                        labelText="Name"
                        placeholder="Enter charge type name"
                        value={newChargeType.name}
                        onChange={handleInputChange}
                        invalid={!!nameError}
                        invalidText={nameError}
                        required
                    />

                    <TextArea
                        id="description"
                        name="description"
                        labelText="Description"
                        placeholder="Enter description (optional)"
                        value={newChargeType.description || ''}
                        onChange={handleInputChange}
                    />
                </div>
            </Form>
        </Modal>
    )
}

export default AddChargeTypeModal
