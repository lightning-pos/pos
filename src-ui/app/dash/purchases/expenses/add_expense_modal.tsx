import React from 'react'
import { Modal, TextInput, Form, DatePicker, DatePickerInput, Select, SelectItem, NumberInput } from '@carbon/react'
import { Expense } from '@/lib/graphql/graphql'
import { formatDateYMD } from '@/lib/util/date_format'
import { sanitizeDecimalInput } from '@/lib/util/number_format'

interface AddExpenseModalProps {
    isOpen: boolean
    expense: Partial<Expense>
    onClose: () => void
    onSave: () => void
    setExpense: React.Dispatch<React.SetStateAction<Partial<Expense>>>
}

const EXPENSE_CATEGORIES = [
    'Office Supplies',
    'Rent',
    'Utilities',
    'Equipment',
    'Salaries',
    'Marketing',
    'Travel',
    'Maintenance',
    'Inventory',
    'Miscellaneous'
];

const AddExpenseModal: React.FC<AddExpenseModalProps> = ({
    isOpen,
    expense,
    onClose,
    onSave,
    setExpense
}) => {
    const handleAmountChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const value = sanitizeDecimalInput(e.target.value, 2);
        setExpense(prev => ({ ...prev, amount: value }));
    };

    return (
        <Modal
            open={isOpen}
            modalHeading="Add New Expense"
            primaryButtonText="Add"
            secondaryButtonText="Cancel"
            onRequestSubmit={onSave}
            onRequestClose={onClose}
        >
            <Form>
                <TextInput
                    id="title"
                    labelText="Title"
                    value={expense?.title || ''}
                    onChange={(e) => setExpense(prev => ({ ...prev, title: e.target.value }))}
                    required
                />

                <TextInput
                    id="amount"
                    labelText="Amount"
                    value={expense?.amount?.toString() || ''}
                    onChange={handleAmountChange}
                    required
                />

                <DatePicker dateFormat="Y-m-d" datePickerType="single">
                    <DatePickerInput
                        id="expense-date"
                        labelText="Expense Date"
                        placeholder="yyyy-mm-dd"
                        defaultValue={expense?.expenseDate ? formatDateYMD(expense.expenseDate) : ''}
                        onChange={(e) => {
                            const inputDate = e.target.value;
                            if (inputDate) {
                                setExpense(prev => ({
                                    ...prev,
                                    expenseDate: inputDate
                                }));
                            }
                        }}
                    />
                </DatePicker>

                <Select
                    id="category"
                    labelText="Category"
                    value={expense?.category || ''}
                    onChange={(e) => setExpense(prev => ({ ...prev, category: e.target.value }))}
                    required
                >
                    <SelectItem value="" text="Choose a category" disabled hidden />
                    {EXPENSE_CATEGORIES.map(category => (
                        <SelectItem key={category} value={category} text={category} />
                    ))}
                </Select>

                <TextInput
                    id="description"
                    labelText="Description"
                    value={expense?.description || ''}
                    onChange={(e) => setExpense(prev => ({ ...prev, description: e.target.value }))}
                />
            </Form>
        </Modal>
    )
}

export default AddExpenseModal
