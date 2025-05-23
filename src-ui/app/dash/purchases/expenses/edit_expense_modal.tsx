import React, { useState, useEffect } from 'react'
import { Modal, TextInput, Form, DatePicker, DatePickerInput, Select, SelectItem } from '@carbon/react'
import { GetPurchaseCategoriesForExpensesDocument, GetCostCentersForExpensesDocument, CostCenterState } from '@/lib/graphql/graphql'
import { TableRow } from './types'
import { formatDateYMD } from '@/lib/util/date_format'
import { sanitizeDecimalInput } from '@/lib/util/number_format'
import { gql } from '@/lib/graphql/execute'

interface EditExpenseModalProps {
    isOpen: boolean
    expense: TableRow | null
    onClose: () => void
    onSave: () => void
    setExpense: React.Dispatch<React.SetStateAction<TableRow | null>>
}

const EditExpenseModal: React.FC<EditExpenseModalProps> = ({
    isOpen,
    expense,
    onClose,
    onSave,
    setExpense
}) => {
    const [categories, setCategories] = useState<Array<{ id: string, name: string }>>([])
    const [costCenters, setCostCenters] = useState<Array<{ id: string, name: string, code: string, state: CostCenterState }>>([])

    useEffect(() => {
        const fetchData = async () => {
            try {
                // Fetch categories
                const categoriesResult = await gql(GetPurchaseCategoriesForExpensesDocument)
                setCategories(categoriesResult.allPurchaseCategories || [])

                // Fetch cost centers
                const costCentersResult = await gql(GetCostCentersForExpensesDocument)
                setCostCenters(costCentersResult.allCostCenters || [])
            } catch (error) {
                console.error('Error fetching data:', error)
            }
        }

        fetchData()
    }, [])

    const handleAmountChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const value = sanitizeDecimalInput(e.target.value, 2)
        setExpense(prev => prev ? { ...prev, amount: value } : null)
    }

    // Filter active cost centers
    const activeCostCenters = costCenters.filter(cc => cc.state === CostCenterState.Active)

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
                            const inputDate = e.target.value
                            if (inputDate) {
                                setExpense(prev => prev ? {
                                    ...prev,
                                    expenseDate: inputDate
                                } : null)
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

                <Select
                    id="costCenter"
                    labelText="Cost Center"
                    value={expense?.costCenterId || ''}
                    onChange={(e) => setExpense(prev => prev ? { ...prev, costCenterId: e.target.value } : null)}
                    required
                >
                    <SelectItem value="" text="Choose a cost center" disabled hidden />
                    {activeCostCenters.map(costCenter => (
                        <SelectItem
                            key={costCenter.id}
                            value={costCenter.id}
                            text={`${costCenter.code} - ${costCenter.name}`}
                        />
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
