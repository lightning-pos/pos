import React, { useState } from 'react'
import { Modal, TextInput, Form, TextArea, Select, SelectItem, DatePicker, DatePickerInput, NumberInput, ModalProps } from '@carbon/react'
import { DiscountNewInput, DiscountType, DiscountScope, DiscountState } from '@/lib/graphql/graphql'
import { formatToLocalDateTime } from '@/lib/util/date_format'

interface AddDiscountModalProps extends Omit<ModalProps, 'onSubmit'> {
    onSave: (discount: DiscountNewInput) => Promise<void>
}

const AddDiscountModal: React.FC<AddDiscountModalProps> = ({
    open,
    onRequestClose,
    onSave,
}) => {
    const [newDiscount, setNewDiscount] = useState<Omit<DiscountNewInput, 'value'> & { value: number }>({
        name: '',
        description: '',
        discountType: DiscountType.Percentage,
        value: 0,
        scope: DiscountScope.AllItems,
        state: DiscountState.Active,
        startDate: null,
        endDate: null,
    })

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault()

        // Format dates and value for the API
        const formattedDiscount: DiscountNewInput = {
            ...newDiscount,
            // Convert number to string for Money type
            value: newDiscount.value.toString(),
            // Format dates without timezone for LocalDateTime using simple format
            startDate: newDiscount.startDate ? formatToLocalDateTime(new Date(newDiscount.startDate)) : null,
            endDate: newDiscount.endDate ? formatToLocalDateTime(new Date(newDiscount.endDate)) : null,
        }

        // Log the formatted object for debugging
        console.log('Submitting discount:', formattedDiscount);

        await onSave(formattedDiscount)
        resetForm()
    }

    const resetForm = () => {
        setNewDiscount({
            name: '',
            description: '',
            discountType: DiscountType.Percentage,
            value: 0,
            scope: DiscountScope.AllItems,
            state: DiscountState.Active,
            startDate: null,
            endDate: null,
        })
    }

    const handleClose = (e: React.SyntheticEvent<HTMLElement>) => {
        onRequestClose?.(e)
        resetForm()
    }

    return (
        <Modal
            open={open}
            onRequestClose={handleClose}
            modalHeading="Add New Discount"
            primaryButtonText="Save"
            onRequestSubmit={handleSubmit}
            size="lg"
        >
            <Form onSubmit={handleSubmit} className='flex flex-col gap-4'>
                <TextInput
                    id="discount-name"
                    labelText="Discount Name"
                    value={newDiscount.name}
                    onChange={(e) => setNewDiscount(prev => ({ ...prev, name: e.target.value }))}
                    required
                />

                <TextArea
                    id="discount-description"
                    labelText="Description"
                    value={newDiscount.description || ''}
                    onChange={(e) => setNewDiscount(prev => ({ ...prev, description: e.target.value }))}
                />

                <div className="grid grid-cols-2 gap-4">
                    <Select
                        id="discount-type"
                        labelText="Discount Type"
                        value={newDiscount.discountType}
                        onChange={(e) => setNewDiscount(prev => ({
                            ...prev,
                            discountType: e.target.value as DiscountType
                        }))}
                        required
                    >
                        <SelectItem value={DiscountType.Percentage} text="Percentage" />
                        <SelectItem value={DiscountType.FixedAmount} text="Fixed Amount" />
                    </Select>

                    <NumberInput
                        id="discount-value"
                        label="Value"
                        min={0}
                        max={newDiscount.discountType === DiscountType.Percentage ? 100 : undefined}
                        step={0.01}
                        value={String(newDiscount.value)}
                        onChange={(e, { value }) => setNewDiscount(prev => ({
                            ...prev,
                            value: parseFloat(String(value))
                        }))}
                        helperText={newDiscount.discountType === DiscountType.Percentage ? "Enter percentage (0-100)" : "Enter amount"}
                        invalidText="Value must be greater than or equal to 0"
                    />
                </div>

                <Select
                    id="discount-scope"
                    labelText="Scope"
                    value={newDiscount.scope}
                    onChange={(e) => setNewDiscount(prev => ({
                        ...prev,
                        scope: e.target.value as DiscountScope
                    }))}
                    required
                    helperText={newDiscount.scope === DiscountScope.SpecificItems ? "You can assign specific items to this discount after creation" : ""}
                >
                    <SelectItem value={DiscountScope.AllItems} text="All Items" />
                    <SelectItem value={DiscountScope.SpecificItems} text="Specific Items" />
                </Select>

                <Select
                    id="discount-state"
                    labelText="Status"
                    value={newDiscount.state || DiscountState.Active}
                    onChange={(e) => setNewDiscount(prev => ({
                        ...prev,
                        state: e.target.value as DiscountState
                    }))}
                    required
                >
                    <SelectItem value={DiscountState.Active} text="Active" />
                    <SelectItem value={DiscountState.Inactive} text="Inactive" />
                    <SelectItem value={DiscountState.Scheduled} text="Scheduled" />
                </Select>

                <div className="grid grid-cols-2 gap-4">
                    <DatePicker
                        datePickerType="single"
                        dateFormat="m/d/Y"
                        onChange={([date]) => setNewDiscount(prev => ({
                            ...prev,
                            startDate: date ? date.toISOString() : null
                        }))}
                    >
                        <DatePickerInput
                            id="discount-start-date"
                            labelText="Start Date"
                            placeholder="mm/dd/yyyy"
                        />
                    </DatePicker>

                    <DatePicker
                        datePickerType="single"
                        dateFormat="m/d/Y"
                        onChange={([date]) => setNewDiscount(prev => ({
                            ...prev,
                            endDate: date ? date.toISOString() : null
                        }))}
                    >
                        <DatePickerInput
                            id="discount-end-date"
                            labelText="End Date"
                            placeholder="mm/dd/yyyy"
                        />
                    </DatePicker>
                </div>
            </Form>
        </Modal>
    )
}

export default AddDiscountModal
