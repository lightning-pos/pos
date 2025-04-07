'use client'
import React, { useState } from 'react'
import {
    Modal,
    TextInput,
    NumberInput,
    Form,
    Stack,
} from '@carbon/react'
import { VariantValueNewInput } from '@/lib/graphql/graphql'

interface AddVariantValueModalProps {
    open: boolean
    onRequestClose: () => void
    onSave: (input: VariantValueNewInput) => void
    variantTypeId: string
}

const AddVariantValueModal: React.FC<AddVariantValueModalProps> = ({
    open,
    onRequestClose,
    onSave,
    variantTypeId,
}) => {
    const [value, setValue] = useState('')
    const [displayOrder, setDisplayOrder] = useState<number | undefined>(undefined)
    const [valueInvalid, setValueInvalid] = useState(false)

    const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault()

        if (!value.trim()) {
            setValueInvalid(true)
            return
        }

        const input: VariantValueNewInput = {
            variantTypeId,
            value: value.trim(),
            displayOrder: displayOrder !== undefined ? displayOrder : undefined,
        }

        onSave(input)
        resetForm()
    }

    const resetForm = () => {
        setValue('')
        setDisplayOrder(undefined)
        setValueInvalid(false)
    }

    const handleClose = () => {
        resetForm()
        onRequestClose()
    }

    return (
        <Modal
            open={open}
            modalHeading="Add Variant Value"
            primaryButtonText="Save"
            secondaryButtonText="Cancel"
            onRequestSubmit={handleSubmit}
            onRequestClose={handleClose}
        >
            <Form onSubmit={handleSubmit}>
                <Stack gap={5}>
                    <TextInput
                        id="value"
                        labelText="Value"
                        placeholder="e.g., Small, Red, Cotton"
                        value={value}
                        onChange={(e) => {
                            setValue(e.target.value)
                            setValueInvalid(false)
                        }}
                        invalid={valueInvalid}
                        invalidText="Value is required"
                        required
                    />
                    <NumberInput
                        id="displayOrder"
                        label="Display Order"
                        helperText="Optional. Lower numbers appear first."
                        value={displayOrder}
                        onChange={(e, { value }) => setDisplayOrder(typeof value === 'number' ? value : undefined)}
                        min={0}
                        step={1}
                    />
                </Stack>
            </Form>
        </Modal>
    )
}

export default AddVariantValueModal
