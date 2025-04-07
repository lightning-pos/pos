'use client'
import React, { useState, useEffect } from 'react'
import {
    Modal,
    TextInput,
    NumberInput,
    Form,
    Stack,
} from '@carbon/react'
import { VariantValue, VariantValueUpdateInput } from '@/lib/graphql/graphql'

interface EditVariantValueModalProps {
    open: boolean
    onRequestClose: () => void
    onSave: (input: VariantValueUpdateInput) => void
    variantValue: VariantValue
}

const EditVariantValueModal: React.FC<EditVariantValueModalProps> = ({
    open,
    onRequestClose,
    onSave,
    variantValue,
}) => {
    const [value, setValue] = useState('')
    const [displayOrder, setDisplayOrder] = useState<number>(0)
    const [valueInvalid, setValueInvalid] = useState(false)

    useEffect(() => {
        if (variantValue) {
            setValue(variantValue.value)
            setDisplayOrder(variantValue.displayOrder)
        }
    }, [variantValue])

    const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault()

        if (!value.trim()) {
            setValueInvalid(true)
            return
        }

        const input: VariantValueUpdateInput = {
            id: variantValue.id,
            value: value.trim() !== variantValue.value ? value.trim() : undefined,
            displayOrder: displayOrder !== variantValue.displayOrder ? displayOrder : undefined,
        }

        onSave(input)
    }

    return (
        <Modal
            open={open}
            modalHeading="Edit Variant Value"
            primaryButtonText="Save"
            secondaryButtonText="Cancel"
            onRequestSubmit={handleSubmit}
            onRequestClose={onRequestClose}
        >
            <Form onSubmit={handleSubmit}>
                <Stack gap={5}>
                    <TextInput
                        id="edit-value"
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
                        id="edit-displayOrder"
                        label="Display Order"
                        helperText="Lower numbers appear first."
                        value={displayOrder}
                        onChange={(e, { value }) => setDisplayOrder(typeof value === 'number' ? value : displayOrder)}
                        min={0}
                        step={1}
                    />
                </Stack>
            </Form>
        </Modal>
    )
}

export default EditVariantValueModal
