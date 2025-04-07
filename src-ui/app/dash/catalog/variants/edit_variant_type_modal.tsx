'use client'
import React, { useState, useEffect } from 'react'
import {
    Modal,
    TextInput,
    TextArea,
    Form,
    Stack,
} from '@carbon/react'
import { VariantType, VariantTypeUpdateInput } from '@/lib/graphql/graphql'

interface EditVariantTypeModalProps {
    open: boolean
    onRequestClose: () => void
    onSave: (input: VariantTypeUpdateInput) => void
    variantType: VariantType
}

const EditVariantTypeModal: React.FC<EditVariantTypeModalProps> = ({
    open,
    onRequestClose,
    onSave,
    variantType,
}) => {
    const [name, setName] = useState('')
    const [description, setDescription] = useState('')
    const [nameInvalid, setNameInvalid] = useState(false)

    useEffect(() => {
        if (variantType) {
            setName(variantType.name)
            setDescription(variantType.description || '')
        }
    }, [variantType])

    const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault()

        if (!name.trim()) {
            setNameInvalid(true)
            return
        }

        const input: VariantTypeUpdateInput = {
            id: variantType.id,
            name: name.trim() !== variantType.name ? name.trim() : undefined,
            description: description.trim() !== (variantType.description || '') 
                ? (description.trim() || null) 
                : undefined,
        }

        onSave(input)
    }

    return (
        <Modal
            open={open}
            modalHeading="Edit Variant Type"
            primaryButtonText="Save"
            secondaryButtonText="Cancel"
            onRequestSubmit={handleSubmit}
            onRequestClose={onRequestClose}
        >
            <Form onSubmit={handleSubmit}>
                <Stack gap={5}>
                    <TextInput
                        id="edit-name"
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
                        id="edit-description"
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

export default EditVariantTypeModal
