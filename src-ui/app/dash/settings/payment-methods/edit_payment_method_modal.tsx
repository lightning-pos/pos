'use client'

import { useState, useEffect } from 'react'
import {
    Modal,
    TextInput,
    Form,
    Stack,
    Select,
    SelectItem,
    TextArea,
    InlineLoading
} from '@carbon/react'
import { gql } from '@/lib/graphql/execute'
import { UpdatePaymentMethodDocument, PaymentMethod, PaymentMethodState } from '@/lib/graphql/graphql'

interface EditPaymentMethodModalProps {
    isOpen: boolean;
    paymentMethod: PaymentMethod;
    onClose: () => void;
    onSave: () => void;
}

export default function EditPaymentMethodModal({ isOpen, paymentMethod, onClose, onSave }: EditPaymentMethodModalProps) {
    // Form state
    const [name, setName] = useState(paymentMethod.name)
    const [code, setCode] = useState(paymentMethod.code)
    const [description, setDescription] = useState(paymentMethod.description || '')
    const [state, setState] = useState<PaymentMethodState>(paymentMethod.state as PaymentMethodState)

    // UI states
    const [loading, setLoading] = useState(false)
    const [error, setError] = useState<string | null>(null)
    const [formErrors, setFormErrors] = useState<{
        name?: string;
        code?: string;
    }>({})

    // Reset form when payment method changes
    useEffect(() => {
        if (isOpen) {
            setName(paymentMethod.name)
            setCode(paymentMethod.code)
            setDescription(paymentMethod.description || '')
            setState(paymentMethod.state as PaymentMethodState)
            setFormErrors({})
            setError(null)
        }
    }, [isOpen, paymentMethod])

    // Validation function
    const validateForm = (): boolean => {
        const errors: { name?: string; code?: string } = {}

        if (!name.trim()) {
            errors.name = 'Name is required'
        }

        if (!code.trim()) {
            errors.code = 'Code is required'
        } else if (code.length > 10) {
            errors.code = 'Code must be 10 characters or less'
        }

        setFormErrors(errors)
        return Object.keys(errors).length === 0
    }

    // Determine which fields have changed
    const getChangedFields = () => {
        const changes: any = { id: paymentMethod.id }

        if (name !== paymentMethod.name) {
            changes.name = name
        }

        if (code !== paymentMethod.code) {
            changes.code = code
        }

        if ((description || '') !== (paymentMethod.description || '')) {
            changes.description = description.trim() || null
        }

        if (state !== paymentMethod.state) {
            changes.state = state
        }

        return changes
    }

    // Handle the form submission
    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault()

        if (!validateForm()) {
            return
        }

        try {
            setLoading(true)
            setError(null)

            const changes = getChangedFields()

            if (Object.keys(changes).length > 1) { // More than just the ID
                await gql(UpdatePaymentMethodDocument, changes)
            }

            // Notify parent component
            onSave()
        } catch (err) {
            console.error('Error updating payment method:', err)
            setError('Failed to update payment method. Please try again.')
        } finally {
            setLoading(false)
        }
    }

    return (
        <Modal
            open={isOpen}
            modalHeading={`Edit Payment Method: ${paymentMethod.name}`}
            primaryButtonText="Save"
            secondaryButtonText="Cancel"
            onRequestClose={onClose}
            onRequestSubmit={handleSubmit}
            primaryButtonDisabled={loading}
        >
            {error && (
                <div className="bg-red-100 border-l-4 border-red-500 text-red-700 p-4 mb-4">
                    {error}
                </div>
            )}

            <Form className="mt-4">
                <Stack gap={6}>
                    <TextInput
                        id="edit-payment-method-code"
                        labelText="Code"
                        value={code}
                        onChange={(e) => setCode(e.target.value)}
                        invalid={!!formErrors.code}
                        invalidText={formErrors.code}
                        placeholder="E.g., CASH, CC, BANK"
                        maxLength={10}
                        required
                    />

                    <TextInput
                        id="edit-payment-method-name"
                        labelText="Name"
                        value={name}
                        onChange={(e) => setName(e.target.value)}
                        invalid={!!formErrors.name}
                        invalidText={formErrors.name}
                        placeholder="E.g., Cash, Credit Card, Bank Transfer"
                        required
                    />

                    <TextArea
                        id="edit-payment-method-description"
                        labelText="Description"
                        value={description}
                        onChange={(e) => setDescription(e.target.value)}
                        placeholder="Enter a description (optional)"
                    />

                    <Select
                        id="edit-payment-method-state"
                        labelText="Status"
                        value={state}
                        onChange={(e) => setState(e.target.value as PaymentMethodState)}
                    >
                        <SelectItem value={PaymentMethodState.Active} text="Active" />
                        <SelectItem value={PaymentMethodState.Inactive} text="Inactive" />
                    </Select>
                </Stack>

                {loading && (
                    <div className="mt-4">
                        <InlineLoading description="Updating payment method..." />
                    </div>
                )}
            </Form>
        </Modal>
    )
}
