'use client'
import React, { useState, useEffect, useCallback } from 'react'
import { Content } from '@carbon/react'
import DataTable from '@/components/ui/DataTable'
import AddTaxModal from './add_tax_modal'
import EditTaxModal from './edit_tax_modal'
import DeleteTaxModal from './delete_tax_modal'
import { GetTaxesDocument, Tax } from '@/lib/graphql/graphql'
import { gql } from '@/lib/graphql/execute'

const Taxes = () => {
    // Modal States
    const [taxes, setTaxes] = useState<Tax[]>([])
    const [selectedTax, setSelectedTax] = useState<Tax | null>(null)
    const [totalTaxes, setTotalTaxes] = useState(0)

    // UI States
    const [loading, setLoading] = useState(true)
    const [currentPage, setCurrentPage] = useState(1)
    const [pageSize, setPageSize] = useState(10)
    const [isAddModalOpen, setIsAddModalOpen] = useState(false)
    const [isEditModalOpen, setIsEditModalOpen] = useState(false)
    const [isDeleteModalOpen, setIsDeleteModalOpen] = useState(false)

    const fetchTaxes = useCallback(async (page: number, size: number) => {
        const offset = (page - 1) * size
        setLoading(true)
        try {
            const result = await gql(GetTaxesDocument, { first: size, offset })
            setTaxes(result.taxes)
            setTotalTaxes(result.totalTaxes)
        } catch (error) {
            console.error('Error fetching taxes:', error)
        } finally {
            setLoading(false)
        }
    }, [])

    useEffect(() => {
        fetchTaxes(currentPage, pageSize)
    }, [fetchTaxes, currentPage, pageSize])

    const headers = [
        { key: 'name', header: 'Name' },
        { key: 'rate', header: 'Rate (%)' },
        { key: 'description', header: 'Description' }
    ]

    const handleAddTax = () => {
        setIsAddModalOpen(true)
    }

    const handleEditTax = (tax: Tax) => {
        setSelectedTax(tax)
        setIsEditModalOpen(true)
    }

    const handleDeleteTax = (tax: Tax) => {
        setSelectedTax(tax)
        setIsDeleteModalOpen(true)
    }

    const handlePageChange = (page: number, size: number) => {
        setCurrentPage(page)
        setPageSize(size)
    }

    return (
        <Content className='min-h-[calc(100dvh-3rem)] p-0 flex flex-col'>
            <div className="p-4 flex-grow flex flex-col" style={{ height: 'calc(100vh - 12rem)' }}>
                <DataTable<Tax>
                    title="Tax"
                    description="Manage your taxes here. You can add, edit, or delete taxes as needed."
                    headers={headers}
                    tableRows={taxes}
                    loading={loading}
                    totalItems={totalTaxes}
                    currentPage={currentPage}
                    pageSize={pageSize}
                    pageSizes={[10, 20, 30, 40, 50]}
                    onPageChange={handlePageChange}
                    onAddClick={handleAddTax}
                    onEditClick={handleEditTax}
                    onDeleteClick={handleDeleteTax}
                />
            </div>
            <AddTaxModal
                open={isAddModalOpen}
                onRequestClose={() => setIsAddModalOpen(false)}
                onRequestSubmit={() => {
                    fetchTaxes(currentPage, pageSize)
                    setIsAddModalOpen(false)
                }}
            />
            {selectedTax && (
                <EditTaxModal
                    open={isEditModalOpen}
                    tax={selectedTax}
                    onRequestClose={() => {
                        setIsEditModalOpen(false)
                        setSelectedTax(null)
                    }}
                    onRequestSubmit={() => {
                        fetchTaxes(currentPage, pageSize)
                        setIsEditModalOpen(false)
                        setSelectedTax(null)
                    }}
                />
            )}
            {selectedTax && (
                <DeleteTaxModal
                    open={isDeleteModalOpen}
                    tax={selectedTax}
                    onRequestClose={() => {
                        setIsDeleteModalOpen(false)
                        setSelectedTax(null)
                    }}
                    onRequestSubmit={() => {
                        fetchTaxes(currentPage, pageSize)
                        setIsDeleteModalOpen(false)
                        setSelectedTax(null)
                    }}
                />
            )}
        </Content>
    )
}

export default Taxes
