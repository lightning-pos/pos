import React, { useState, useEffect } from 'react'
import { Modal, TextInput, Form, DatePicker, DatePickerInput, Select, SelectItem } from '@carbon/react'
import { Expense, GetPurchaseCategoriesForExpensesDocument } from '@/lib/graphql/graphql'
import { formatDateYMD } from '@/lib/util/date_format'
import { sanitizeDecimalInput } from '@/lib/util/number_format'
import { gql } from '@/lib/graphql/execute'

interface EditExpenseModalProps {
    isOpen: boolean
    expense: Expense | null
    onClose: () => void
    onSave: () => void
    setExpense: React.Dispatch<React.SetStateAction<Expense | null>>
}

const EditExpenseModal: React.FC<EditExpenseModalProps> = ({
    isOpen,
    expense,
    onClose,
    onSave,
    setExpense
}) => {
    const [categories, setCategories] = useState<Array<{ id: string, name: string }>>([]);

    useEffect(() => {
        const fetchCategories = async () => {
            try {
                const result = await gql(GetPurchaseCategoriesForExpensesDocument);
                setCategories(result.allPurchaseCategories || []);
            } catch (error) {
                console.error('Error fetching expense categories:', error);
            }
        };

        fetchCategories();
    }, []);

    const handleAmountChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const value = sanitizeDecimalInput(e.target.value, 2);
        setExpense(prev => prev ? { ...prev, amount: value } : null);
    };

    return (
        <Modal
            open={isOpen}
            modalHeading="Edit Expense"
            primaryButtonText="Update"
            secondaryButtonText="Cancel"
            onRequestSubmit={onSave}
            onRequestClose={onClose}
        >
            <Form>
                <TextInput
                    id="title"
                    labelText="Title"
                    value={expense?.title || ''}
                    onChange={(e) => setExpense(prev => prev ? { ...prev, title: e.target.value } : null)}
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
                                setExpense(prev => prev ? {
                                    ...prev,
                                    expenseDate: inputDate
                                } : null);
                            }
                        }}
                    />
                </DatePicker>

                <Select
                    id="category"
                    labelText="Category"
                    value={expense?.categoryId || ''}
                    onChange={(e) => setExpense(prev => prev ? { ...prev, categoryId: e.target.value } : null)}
                    required
                >
                    <SelectItem value="" text="Choose a category" disabled hidden />
                    {categories.map(category => (
                        <SelectItem key={category.id} value={category.id} text={category.name} />
                    ))}
                </Select>

                <TextInput
                    id="description"
                    labelText="Description"
                    value={expense?.description || ''}
                    onChange={(e) => setExpense(prev => prev ? { ...prev, description: e.target.value } : null)}
                />
            </Form>
        </Modal>
    )
}

export default EditExpenseModal
