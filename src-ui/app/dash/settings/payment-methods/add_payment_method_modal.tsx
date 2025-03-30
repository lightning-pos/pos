'use client';

import { useState } from 'react';
import {
    Modal,
    TextInput,
    Form,
    Stack,
    Select,
    SelectItem,
    TextArea,
    InlineLoading
} from '@carbon/react';
import { gql } from '@/lib/graphql/execute';
import { CreatePaymentMethodDocument, PaymentMethodState } from '@/lib/graphql/graphql';

interface AddPaymentMethodModalProps {
    isOpen: boolean;
    onClose: () => void;
    onSave: () => void;
}

export default function AddPaymentMethodModal({ isOpen, onClose, onSave }: AddPaymentMethodModalProps) {
    // Form state
    const [name, setName] = useState('');
    const [code, setCode] = useState('');
    const [description, setDescription] = useState('');
    const [state, setState] = useState<PaymentMethodState>(PaymentMethodState.Active);

    // UI states
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const [formErrors, setFormErrors] = useState<{
        name?: string;
        code?: string;
    }>({});

    // Validation function
    const validateForm = (): boolean => {
        const errors: { name?: string; code?: string } = {};

        if (!name.trim()) {
            errors.name = 'Name is required';
        }

        if (!code.trim()) {
            errors.code = 'Code is required';
        } else if (code.length > 10) {
            errors.code = 'Code must be 10 characters or less';
        }

        setFormErrors(errors);
        return Object.keys(errors).length === 0;
    };

    // Handle the form submission
    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();

        if (!validateForm()) {
            return;
        }

        try {
            setLoading(true);
            setError(null);

            await gql(CreatePaymentMethodDocument, {
                name,
                code,
                description: description.trim() || null,
                state,
            });

            // Reset form
            setName('');
            setCode('');
            setDescription('');
            setState(PaymentMethodState.Active);
            setFormErrors({});

            // Notify parent component
            onSave();
        } catch (err) {
            console.error('Error creating payment method:', err);
            setError('Failed to create payment method. Please try again.');
        } finally {
            setLoading(false);
        }
    };

    const handleCancel = () => {
        // Reset form
        setName('');
        setCode('');
        setDescription('');
        setState(PaymentMethodState.Active);
        setFormErrors({});
        setError(null);

        // Close modal
        onClose();
    };

    return (
        <Modal
            open={isOpen}
            modalHeading="Add Payment Method"
            primaryButtonText="Create"
            secondaryButtonText="Cancel"
            onRequestClose={handleCancel}
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
                        id="add-payment-method-code"
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
                        id="add-payment-method-name"
                        labelText="Name"
                        value={name}
                        onChange={(e) => setName(e.target.value)}
                        invalid={!!formErrors.name}
                        invalidText={formErrors.name}
                        placeholder="E.g., Cash, Credit Card, Bank Transfer"
                        required
                    />

                    <TextArea
                        id="add-payment-method-description"
                        labelText="Description"
                        value={description}
                        onChange={(e) => setDescription(e.target.value)}
                        placeholder="Enter a description (optional)"
                    />

                    <Select
                        id="add-payment-method-state"
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
                        <InlineLoading description="Creating payment method..." />
                    </div>
                )}
            </Form>
        </Modal>
    );
}
