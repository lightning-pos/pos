'use client'
import React, { useState } from 'react'
import {
    Modal,
    TextInput,
    TextArea,
    Form,
    Stack,
} from '@carbon/react'
import { VariantTypeNewInput } from '@/lib/graphql/graphql'

interface AddVariantTypeModalProps {
    open: boolean
    onRequestClose: () => void
    onSave: (input: VariantTypeNewInput) => void
}

const AddVariantTypeModal: React.FC<AddVariantTypeModalProps> = ({
    open,
    onRequestClose,
    onSave,
}) => {
    const [name, setName] = useState('')
    const [description, setDescription] = useState('')
    const [nameInvalid, setNameInvalid] = useState(false)

    const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault()

        if (!name.trim()) {
            setNameInvalid(true)
            return
        }

        const input: VariantTypeNewInput = {
            name: name.trim(),
            description: description.trim() || null,
        }

        onSave(input)
        resetForm()
    }

    const resetForm = () => {
        setName('')
        setDescription('')
        setNameInvalid(false)
    }

    const handleClose = () => {
        resetForm()
        onRequestClose()
    }

    return (
        <Modal
            open={open}
            modalHeading="Add Variant Type"
            primaryButtonText="Save"
            secondaryButtonText="Cancel"
            onRequestSubmit={handleSubmit}
            onRequestClose={handleClose}
        >
            <Form onSubmit={handleSubmit}>
                <Stack gap={5}>
                    <TextInput
                        id="name"
                        labelText="Name"
                        placeholder="e.g., Size, Color, Material"
                        value={name}
                        onChange={(e) => {
                            setName(e.target.value)
                            setNameInvalid(false)
                        }}
                        invalid={nameInvalid}
                        invalidText="Name is required"
                        required
                    />
                    <TextArea
                        id="description"
                        labelText="Description"
                        placeholder="Optional description"
                        value={description}
                        onChange={(e) => setDescription(e.target.value)}
                    />
                </Stack>
            </Form>
        </Modal>
    )
}

export default AddVariantTypeModal
