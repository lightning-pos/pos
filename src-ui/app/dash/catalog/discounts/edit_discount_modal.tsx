import React, { useState, useEffect } from 'react'
import { Modal, TextInput, Form, TextArea, Select, SelectItem, DatePicker, DatePickerInput, NumberInput, ModalProps } from '@carbon/react'
import { Discount, DiscountUpdateInput, DiscountType, DiscountScope, DiscountState } from '@/lib/graphql/graphql'
import { formatToLocalDateTime } from '@/lib/util/date_format'

interface EditDiscountModalProps extends Omit<ModalProps, 'onSubmit'> {
    discount: Discount
    onSave: (discount: DiscountUpdateInput) => Promise<void>
}

// Custom type to handle numeric value in the UI but string in the API
type EditDiscountFormState = Omit<DiscountUpdateInput, 'value'> & {
    value: number | undefined
}

const EditDiscountModal: React.FC<EditDiscountModalProps> = ({
    open,
    onRequestClose,
    discount,
    onSave,
}) => {
    const [updatedDiscount, setUpdatedDiscount] = useState<EditDiscountFormState>({
        id: discount.id,
        name: discount.name,
        description: discount.description,
        discountType: discount.discountType,
        value: typeof discount.value === 'string' ? parseFloat(discount.value) : discount.value,
        scope: discount.scope,
        state: discount.state,
        startDate: discount.startDate,
        endDate: discount.endDate,
    })

    // Update form when discount changes
    useEffect(() => {
        if (discount) {
            setUpdatedDiscount({
                id: discount.id,
                name: discount.name,
                description: discount.description,
                discountType: discount.discountType,
                value: typeof discount.value === 'string' ? parseFloat(discount.value) : discount.value,
                scope: discount.scope,
                state: discount.state,
                startDate: discount.startDate,
                endDate: discount.endDate,
            })
        }
    }, [discount])

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault()

        // Format dates and value for the API
        const formattedDiscount: DiscountUpdateInput = {
            ...updatedDiscount,
            // Convert number to string for Money type (only if value exists)
            value: updatedDiscount.value !== undefined ? updatedDiscount.value.toString() : undefined,
            // Format dates without timezone for LocalDateTime
            startDate: updatedDiscount.startDate
                ? formatToLocalDateTime(new Date(updatedDiscount.startDate as string))
                : updatedDiscount.startDate,
            endDate: updatedDiscount.endDate
                ? formatToLocalDateTime(new Date(updatedDiscount.endDate as string))
                : updatedDiscount.endDate,
        }

        // Debug log to see what we're sending
        console.log('Updating discount:', formattedDiscount);

        await onSave(formattedDiscount)
    }

    const handleClose = (e: React.SyntheticEvent<HTMLElement>) => {
        onRequestClose?.(e)
    }

    // Format dates for the date picker if they exist
    const formatDateForPicker = (isoDate: string | null | undefined) => {
        if (!isoDate) return undefined
        return new Date(isoDate)
    }

    return (
        <Modal
            open={open}
            onRequestClose={handleClose}
            modalHeading="Edit Discount"
            primaryButtonText="Save"
            onRequestSubmit={handleSubmit}
            size="lg"
        >
            <Form onSubmit={handleSubmit} className='flex flex-col gap-4'>
                <TextInput
                    id="edit-discount-name"
                    labelText="Discount Name"
                    value={updatedDiscount.name || ''}
                    onChange={(e) => setUpdatedDiscount(prev => ({ ...prev, name: e.target.value || undefined }))}
                    required
                />

                <TextArea
                    id="edit-discount-description"
                    labelText="Description"
                    value={updatedDiscount.description || ''}
                    onChange={(e) => {
                        const value = e.target.value || null
                        setUpdatedDiscount(prev => ({ ...prev, description: value }))
                    }}
                />

                <div className="grid grid-cols-2 gap-4">
                    <Select
                        id="edit-discount-type"
                        labelText="Discount Type"
                        value={updatedDiscount.discountType || ''}
                        onChange={(e) => setUpdatedDiscount(prev => ({
                            ...prev,
                            discountType: e.target.value as DiscountType || undefined
                        }))}
                    >
                        <SelectItem value={DiscountType.Percentage} text="Percentage" />
                        <SelectItem value={DiscountType.FixedAmount} text="Fixed Amount" />
                    </Select>

                    <NumberInput
                        id="edit-discount-value"
                        label="Value"
                        min={0}
                        max={updatedDiscount.discountType === DiscountType.Percentage ? 100 : undefined}
                        step={0.01}
                        value={updatedDiscount.value !== undefined ? String(updatedDiscount.value) : "0"}
                        onChange={(e, { value }) => setUpdatedDiscount(prev => ({
                            ...prev,
                            value: parseFloat(String(value))
                        }))}
                        helperText={updatedDiscount.discountType === DiscountType.Percentage ? "Enter percentage (0-100)" : "Enter amount"}
                        invalidText="Value must be greater than or equal to 0"
                    />
                </div>

                <Select
                    id="edit-discount-scope"
                    labelText="Scope"
                    value={updatedDiscount.scope || ''}
                    onChange={(e) => setUpdatedDiscount(prev => ({
                        ...prev,
                        scope: e.target.value as DiscountScope || undefined
                    }))}
                    helperText={updatedDiscount.scope === DiscountScope.SpecificItems ? "You can assign specific items to this discount after saving" : ""}
                >
                    <SelectItem value={DiscountScope.AllItems} text="All Items" />
                    <SelectItem value={DiscountScope.SpecificItems} text="Specific Items" />
                </Select>

                <Select
                    id="edit-discount-state"
                    labelText="Status"
                    value={updatedDiscount.state || ''}
                    onChange={(e) => setUpdatedDiscount(prev => ({
                        ...prev,
                        state: e.target.value as DiscountState || undefined
                    }))}
                >
                    <SelectItem value={DiscountState.Active} text="Active" />
                    <SelectItem value={DiscountState.Inactive} text="Inactive" />
                    <SelectItem value={DiscountState.Scheduled} text="Scheduled" />
                    <SelectItem value={DiscountState.Expired} text="Expired" />
                </Select>

                <div className="grid grid-cols-2 gap-4">
                    <DatePicker
                        datePickerType="single"
                        dateFormat="m/d/Y"
                        value={formatDateForPicker(updatedDiscount.startDate as string)}
                        onChange={([date]) => {
                            if (date) {
                                console.log('Selected start date:', date);
                                setUpdatedDiscount(prev => ({
                                    ...prev,
                                    startDate: date.toISOString()
                                }))
                            } else {
                                setUpdatedDiscount(prev => ({
                                    ...prev,
                                    startDate: null
                                }))
                            }
                        }}
                    >
                        <DatePickerInput
                            id="edit-discount-start-date"
                            labelText="Start Date"
                            placeholder="mm/dd/yyyy"
                        />
                    </DatePicker>

                    <DatePicker
                        datePickerType="single"
                        dateFormat="m/d/Y"
                        value={formatDateForPicker(updatedDiscount.endDate as string)}
                        onChange={([date]) => {
                            if (date) {
                                console.log('Selected end date:', date);
                                setUpdatedDiscount(prev => ({
                                    ...prev,
                                    endDate: date.toISOString()
                                }))
                            } else {
                                setUpdatedDiscount(prev => ({
                                    ...prev,
                                    endDate: null
                                }))
                            }
                        }}
                    >
                        <DatePickerInput
                            id="edit-discount-end-date"
                            labelText="End Date"
                            placeholder="mm/dd/yyyy"
                        />
                    </DatePicker>
                </div>
            </Form>
        </Modal>
    )
}

export default EditDiscountModal
