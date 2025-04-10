'use client'
import { useState, useCallback, useEffect } from 'react'
import { Content, Tag } from '@carbon/react'
import DataTable from '@/components/ui/DataTable'
import {
    GetDiscountsDocument,
    Discount,
    DiscountNewInput,
    DiscountUpdateInput,
    CreateDiscountDocument,
    UpdateDiscountDocument,
    DeleteDiscountDocument,
    DiscountState,
    DiscountType
} from '@/lib/graphql/graphql'
import { gql } from '@/lib/graphql/execute'
import AddDiscountModal from './add_discount_modal'
import EditDiscountModal from './edit_discount_modal'
import DeleteDiscountModal from './delete_discount_modal'
import { formatCurrency } from '@/lib/util/number_format'

const Discounts = () => {
    const [discounts, setDiscounts] = useState<Discount[]>([])
    const [currentPage, setCurrentPage] = useState(1)
    const [pageSize, setPageSize] = useState(10)
    const [totalItems, setTotalItems] = useState(0)

    // Modal States
    const [selectedDiscount, setSelectedDiscount] = useState<Discount | null>(null)
    const [isAddModalOpen, setIsAddModalOpen] = useState(false)
    const [isEditModalOpen, setIsEditModalOpen] = useState(false)
    const [isDeleteModalOpen, setIsDeleteModalOpen] = useState(false)

    const fetchDiscounts = useCallback(async (page: number, size: number) => {
        try {
            const result = await gql(GetDiscountsDocument, {
                first: size,
                offset: (page - 1) * size
            })

            // Log received discount data to debug start/end dates
            console.log('Received discounts:', result.discounts)

            setDiscounts(result.discounts)
            setTotalItems(result.discounts.length) // In a real scenario, you would get a count from the API
        } catch (error) {
            console.error('Error fetching discounts:', error)
        }
    }, [])

    const handleAddDiscount = async (discount: DiscountNewInput) => {
        try {
            console.log('Adding discount with data:', discount)
            const result = await gql(CreateDiscountDocument, { discount })
            console.log('Create discount result:', result)
            await fetchDiscounts(currentPage, pageSize)
            setIsAddModalOpen(false)
        } catch (error) {
            console.error('Error adding discount:', error)
        }
    }

    const handleEditDiscount = async (discount: DiscountUpdateInput) => {
        try {
            console.log('Updating discount with data:', discount)
            const result = await gql(UpdateDiscountDocument, { discount })
            console.log('Update discount result:', result)
            await fetchDiscounts(currentPage, pageSize)
            setIsEditModalOpen(false)
            setSelectedDiscount(null)
        } catch (error) {
            console.error('Error updating discount:', error)
        }
    }

    const handleDeleteDiscount = async (id: string) => {
        try {
            await gql(DeleteDiscountDocument, { id })
            await fetchDiscounts(currentPage, pageSize)
            setIsDeleteModalOpen(false)
            setSelectedDiscount(null)
        } catch (error) {
            console.error('Error deleting discount:', error)
        }
    }

    useEffect(() => {
        fetchDiscounts(currentPage, pageSize)
    }, [fetchDiscounts, currentPage, pageSize])

    // Custom cell renderers
    const renderValue = (value: number, row: Discount) => {
        if (row.discountType === DiscountType.Percentage) {
            return `${value}%`
        }
        return formatCurrency(value)
    }

    const renderDate = (date: string | null) => {
        if (!date) return '-'
        try {
            // Format the date in a user-friendly way
            return new Date(date).toLocaleDateString()
        } catch (e) {
            console.error('Error formatting date:', date, e)
            return date || '-'
        }
    }

    const renderState = (state: DiscountState) => {
        let tagType = ''
        switch (state) {
        case DiscountState.Active:
            tagType = 'green'
            break
        case DiscountState.Inactive:
            tagType = 'gray'
            break
        case DiscountState.Scheduled:
            tagType = 'blue'
            break
        case DiscountState.Expired:
            tagType = 'red'
            break
        default:
            tagType = 'gray'
        }
        return <Tag type={tagType as any}>{state}</Tag>
    }

    const headers = [
        { key: 'name', header: 'Name' },
        {
            key: 'value',
            header: 'Value',
            render: (value: number, row: Discount) => renderValue(value, row)
        },
        { key: 'discountType', header: 'Type' },
        {
            key: 'state',
            header: 'Status',
            render: (state: DiscountState) => renderState(state)
        },
        {
            key: 'startDate',
            header: 'Start Date',
            render: (date: string) => renderDate(date)
        },
        {
            key: 'endDate',
            header: 'End Date',
            render: (date: string) => renderDate(date)
        }
    ]

    return (
        <Content className='min-h-[calc(100dvh-3rem)] p-0 flex flex-col'>
            <div className="p-4 flex flex-col gap-4">
                <div className="flex justify-between items-center">
                    <h1 className="text-2xl font-bold">Discounts</h1>
                </div>

                <DataTable<Discount>
                    title="Discounts"
                    description="Manage your discounts here. You can add, edit, or delete discounts as needed."
                    headers={headers}
                    tableRows={discounts}
                    loading={false}
                    totalItems={totalItems}
                    currentPage={currentPage}
                    pageSize={pageSize}
                    pageSizes={[10, 20, 30, 40, 50]}
                    onPageChange={(page, size) => {
                        setCurrentPage(page)
                        setPageSize(size)
                        fetchDiscounts(page, size)
                    }}
                    onAddClick={() => setIsAddModalOpen(true)}
                    onEditClick={(item) => {
                        setSelectedDiscount(item)
                        setIsEditModalOpen(true)
                    }}
                    onDeleteClick={(item) => {
                        setSelectedDiscount(item)
                        setIsDeleteModalOpen(true)
                    }}
                />

                <AddDiscountModal
                    open={isAddModalOpen}
                    onRequestClose={() => setIsAddModalOpen(false)}
                    onSave={handleAddDiscount}
                />

                {selectedDiscount && (
                    <EditDiscountModal
                        open={isEditModalOpen}
                        onRequestClose={() => {
                            setIsEditModalOpen(false)
                            setSelectedDiscount(null)
                        }}
                        onSave={handleEditDiscount}
                        discount={selectedDiscount}
                    />
                )}

                {selectedDiscount && (
                    <DeleteDiscountModal
                        isOpen={isDeleteModalOpen}
                        onClose={() => {
                            setIsDeleteModalOpen(false)
                            setSelectedDiscount(null)
                        }}
                        onDelete={() => handleDeleteDiscount(selectedDiscount.id)}
                        discountName={selectedDiscount.name}
                    />
                )}
            </div>
        </Content>
    )
}

export default Discounts
