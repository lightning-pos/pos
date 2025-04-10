'use client'
import React, { useState, useEffect, useCallback } from 'react'
import { Content } from '@carbon/react'
import DataTable from '@/components/ui/DataTable'
import DeleteExpenseModal from './delete_expense_modal'
import AddExpenseModal from './add_expense_modal'
import EditExpenseModal from './edit_expense_modal'
import { gql } from '@/lib/graphql/execute'
import { GetExpensesDocument, CreateExpenseDocument, UpdateExpenseDocument, DeleteExpenseDocument } from '@/lib/graphql/graphql'
import { formatCurrency } from '@/lib/util/number_format'
import { formatDateForDisplay, formatToLocalDateTime, formatDateYMD } from '@/lib/util/date_format'
import { TableRow } from './types'

const ExpensesPage = () => {
    // State declarations
    const [expenses, setExpenses] = useState<TableRow[]>([])
    const [totalExpenses, setTotalExpenses] = useState(0)
    const [loading, setLoading] = useState(true)
    const [newExpense, setNewExpense] = useState<Partial<TableRow>>({})
    const [editingExpense, setEditingExpense] = useState<TableRow | null>(null)
    const [isAddModalOpen, setIsAddModalOpen] = useState(false)
    const [isEditModalOpen, setIsEditModalOpen] = useState(false)
    const [isDeleteModalOpen, setIsDeleteModalOpen] = useState(false)
    const [currentPage, setCurrentPage] = useState(1)
    const [pageSize, setPageSize] = useState(10)

    // Format expense data for table
    const formatExpenseData = (expenses: any[]): TableRow[] => {
        return expenses.map(expense => ({
            id: expense.id,
            title: expense.title,
            amount: formatCurrency(Number(expense.amount || 0)),
            expenseDate: expense.expenseDate,
            categoryId: expense.categoryId,
            costCenterId: expense.costCenterId,
            description: expense.description,
            createdAt: expense.createdAt,
            updatedAt: expense.updatedAt,
            formattedDate: formatDateForDisplay(expense.expenseDate),
            category: expense.category?.name || 'Uncategorized',
            costCenter: expense.costCenter ? `${expense.costCenter.code} - ${expense.costCenter.name}` : 'Unknown'
        }))
    }

    // Fetch expenses with server-side pagination
    const fetchExpenses = useCallback(async (page: number, size: number) => {
        setLoading(true)
        try {
            const offset = (page - 1) * size
            const result = await gql(GetExpensesDocument, { first: size, offset })
            setExpenses(formatExpenseData(result.expenses))
            setTotalExpenses(result.totalExpenses)
        } catch (error) {
            console.error('Error fetching expenses:', error)
        } finally {
            setLoading(false)
        }
    }, [])

    // Fetch expenses on component mount and when pagination changes
    useEffect(() => {
        fetchExpenses(currentPage, pageSize)
    }, [fetchExpenses, currentPage, pageSize])

    // Add expense
    const handleAddExpense = async () => {
        try {
            await gql(CreateExpenseDocument, {
                input: {
                    title: newExpense.title || '',
                    amount: (Number(newExpense.amount) || 0).toString(),
                    expenseDate: formatToLocalDateTime(newExpense.expenseDate),
                    categoryId: newExpense.categoryId || '',
                    costCenterId: newExpense.costCenterId || '',
                    description: newExpense.description || null
                }
            })
            setIsAddModalOpen(false)
            setNewExpense({})
            fetchExpenses(currentPage, pageSize)
        } catch (error) {
            console.error('Error adding expense:', error)
        }
    }

    // Update expense
    const handleUpdateExpense = async () => {
        if (!editingExpense?.id) return

        try {
            await gql(UpdateExpenseDocument, {
                input: {
                    id: editingExpense.id,
                    title: editingExpense.title,
                    amount: (Number(editingExpense.amount) || 0).toString(),
                    expenseDate: formatToLocalDateTime(editingExpense.expenseDate),
                    categoryId: editingExpense.categoryId,
                    costCenterId: editingExpense.costCenterId,
                    description: editingExpense.description
                }
            })
            setIsEditModalOpen(false)
            setEditingExpense(null)
            fetchExpenses(currentPage, pageSize)
        } catch (error) {
            console.error('Error updating expense:', error)
        }
    }

    // Delete expense
    const handleDeleteExpense = async (id: string) => {
        try {
            await gql(DeleteExpenseDocument, { id })
            setIsDeleteModalOpen(false)
            setEditingExpense(null)
            fetchExpenses(currentPage, pageSize)
        } catch (error) {
            console.error('Error deleting expense:', error)
        }
    }

    const headers = [
        { key: 'title', header: 'Title' },
        { key: 'amount', header: 'Amount' },
        { key: 'formattedDate', header: 'Date' },
        { key: 'category', header: 'Category' },
        { key: 'costCenter', header: 'Cost Center' },
        { key: 'description', header: 'Description' }
    ]

    return (
        <Content className='min-h-[calc(100dvh-3rem)] p-0 flex flex-col'>
            <div className="p-4 flex-grow flex flex-col" style={{ minHeight: 'calc(100vh - 12rem)' }}>
                <DataTable<TableRow>
                    title="Expenses"
                    description="Manage your expenses here. Track all business costs and categorize them for better financial management."
                    headers={headers}
                    tableRows={expenses}
                    loading={loading}
                    totalItems={totalExpenses}
                    currentPage={currentPage}
                    pageSize={pageSize}
                    pageSizes={[10, 20, 30, 40, 50]}
                    onPageChange={(page, pageSize) => {
                        setCurrentPage(page)
                        setPageSize(pageSize)
                        fetchExpenses(page, pageSize)
                    }}
                    onAddClick={() => {
                        setNewExpense({
                            expenseDate: formatDateYMD(new Date())
                        })
                        setIsAddModalOpen(true)
                    }}
                    onEditClick={(expense: TableRow) => {
                        // Convert back to original format for editing
                        const originalExpense = {
                            ...expense,
                            amount: expense.amount.replace(/[^0-9.]/g, '')
                        }
                        setEditingExpense(originalExpense)
                        setIsEditModalOpen(true)
                    }}
                    onDeleteClick={(expense: TableRow) => {
                        // Convert back to original format
                        const originalExpense = {
                            ...expense,
                            amount: expense.amount.replace(/[^0-9.]/g, '')
                        }
                        setEditingExpense(originalExpense)
                        setIsDeleteModalOpen(true)
                    }}
                />
            </div>

            <AddExpenseModal
                isOpen={isAddModalOpen}
                expense={newExpense}
                onClose={() => {
                    setIsAddModalOpen(false)
                    setNewExpense({})
                }}
                setExpense={setNewExpense}
                onSave={handleAddExpense}
            />

            {editingExpense && (
                <EditExpenseModal
                    isOpen={isEditModalOpen}
                    expense={editingExpense}
                    onClose={() => {
                        setIsEditModalOpen(false)
                        setEditingExpense(null)
                    }}
                    setExpense={setEditingExpense}
                    onSave={handleUpdateExpense}
                />
            )}

            <DeleteExpenseModal
                isOpen={isDeleteModalOpen}
                expenseId={editingExpense?.id || ''}
                expenseTitle={editingExpense?.title || ''}
                onClose={() => {
                    setIsDeleteModalOpen(false)
                    setEditingExpense(null)
                }}
                onDelete={() => editingExpense?.id && handleDeleteExpense(editingExpense.id)}
            />
        </Content>
    )
}

export default ExpensesPage
