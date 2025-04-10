'use client'
import React, { useState, useEffect, useCallback } from 'react'
import { Content } from '@carbon/react'
import DataTable from '@/components/ui/DataTable'
import AddTaxGroupModal from './add_tax_group_modal'
import EditTaxGroupModal from './edit_tax_group_modal'
import DeleteTaxGroupModal from './delete_tax_group_modal'
import { GetTaxGroupsDocument } from '@/lib/graphql/graphql'
import type { TaxGroup } from '@/lib/graphql/graphql'
import { gql } from '@/lib/graphql/execute'

// Create a new interface for the processed tax groups
interface ProcessedTaxGroup extends Omit<TaxGroup, 'taxes'> {
    taxes: string;
    id: string;
}

const TaxGroups = () => {
    // Modal States
    const [taxGroups, setTaxGroups] = useState<TaxGroup[]>([])
    const [selectedTaxGroup, setSelectedTaxGroup] = useState<TaxGroup | null>(null)
    const [totalTaxGroups, setTotalTaxGroups] = useState(0)

    // UI States
    const [loading, setLoading] = useState(true)
    const [currentPage, setCurrentPage] = useState(1)
    const [pageSize, setPageSize] = useState(10)
    const [isAddModalOpen, setIsAddModalOpen] = useState(false)
    const [isEditModalOpen, setIsEditModalOpen] = useState(false)
    const [isDeleteModalOpen, setIsDeleteModalOpen] = useState(false)

    const fetchTaxGroups = useCallback(async (page: number, size: number) => {
        const offset = (page - 1) * size
        setLoading(true)
        try {
            const result = await gql(GetTaxGroupsDocument, { first: size, offset })
            setTaxGroups(result.taxGroups as TaxGroup[])
            setTotalTaxGroups(result.totalTaxGroups as number)
        } catch (error) {
            console.error('Error fetching tax groups:', error)
        } finally {
            setLoading(false)
        }
    }, [])

    useEffect(() => {
        fetchTaxGroups(currentPage, pageSize)
    }, [fetchTaxGroups, currentPage, pageSize])

    const headers = [
        { key: 'name', header: 'Name' },
        { key: 'description', header: 'Description' },
        {
            key: 'taxes', header: 'Taxes', render: (item: TaxGroup) => {
                return item.taxes?.map(tax => {
                    // Handle the Percentage scalar type
                    const rateStr = String(tax.rate)
                    return `${tax.name} (${rateStr}%)`
                }).join(', ') || 'None'
            }
        }
    ]

    const handleAddTaxGroup = () => {
        setIsAddModalOpen(true)
    }

    const handleEditTaxGroup = (taxGroup: ProcessedTaxGroup) => {
        // Find the original tax group with the same ID
        const originalTaxGroup = taxGroups.find(group => group.id === taxGroup.id)
        if (originalTaxGroup) {
            setSelectedTaxGroup(originalTaxGroup)
            setIsEditModalOpen(true)
        }
    }

    const handleDeleteTaxGroup = (taxGroup: ProcessedTaxGroup) => {
        // Find the original tax group with the same ID
        const originalTaxGroup = taxGroups.find(group => group.id === taxGroup.id)
        if (originalTaxGroup) {
            setSelectedTaxGroup(originalTaxGroup)
            setIsDeleteModalOpen(true)
        }
    }

    const handlePageChange = (page: number, size: number) => {
        setCurrentPage(page)
        setPageSize(size)
    }

    return (
        <Content className='min-h-[calc(100dvh-3rem)] p-0 flex flex-col'>
            <div className="p-4 flex-grow flex flex-col" style={{ height: 'calc(100vh - 12rem)' }}>
                <DataTable<ProcessedTaxGroup>
                    title="Tax Groups"
                    description="Manage your tax groups here. Tax groups allow you to combine multiple taxes (like CGST and SGST) for easier application."
                    headers={headers}
                    tableRows={taxGroups.map(group => ({
                        ...group,
                        taxes: group.taxes?.map(tax => {
                            const rateStr = String(tax.rate)
                            return `${tax.name} (${rateStr}%)`
                        }).join(', ') || 'None'
                    })) as ProcessedTaxGroup[]}
                    loading={loading}
                    totalItems={totalTaxGroups}
                    currentPage={currentPage}
                    pageSize={pageSize}
                    pageSizes={[10, 20, 30, 40, 50]}
                    onPageChange={handlePageChange}
                    onAddClick={handleAddTaxGroup}
                    onEditClick={handleEditTaxGroup}
                    onDeleteClick={handleDeleteTaxGroup}
                />
            </div>
            <AddTaxGroupModal
                open={isAddModalOpen}
                onRequestClose={() => setIsAddModalOpen(false)}
                onRequestSubmit={() => {
                    fetchTaxGroups(currentPage, pageSize)
                    setIsAddModalOpen(false)
                }}
            />
            {selectedTaxGroup && (
                <EditTaxGroupModal
                    open={isEditModalOpen}
                    taxGroup={selectedTaxGroup}
                    onRequestClose={() => {
                        setIsEditModalOpen(false)
                        setSelectedTaxGroup(null)
                    }}
                    onRequestSubmit={() => {
                        fetchTaxGroups(currentPage, pageSize)
                        setIsEditModalOpen(false)
                        setSelectedTaxGroup(null)
                    }}
                />
            )}
            {selectedTaxGroup && (
                <DeleteTaxGroupModal
                    open={isDeleteModalOpen}
                    taxGroup={selectedTaxGroup}
                    onRequestClose={() => {
                        setIsDeleteModalOpen(false)
                        setSelectedTaxGroup(null)
                    }}
                    onRequestSubmit={() => {
                        fetchTaxGroups(currentPage, pageSize)
                        setIsDeleteModalOpen(false)
                        setSelectedTaxGroup(null)
                    }}
                />
            )}
        </Content>
    )
}

export default TaxGroups
