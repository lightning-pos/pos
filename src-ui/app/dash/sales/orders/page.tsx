'use client'
import React, { useEffect, useState } from 'react'
import { Content, DataTable, TableContainer, Table, TableHead, TableRow, TableHeader, TableBody, TableCell, Pagination, Button } from '@carbon/react'
import { gql } from '@/lib/graphql/execute'
import { GetSalesOrdersDocument, SalesOrder, SalesOrderItem } from '@/lib/graphql/graphql'
import { formatCurrency } from '@/lib/util/number_format'
import OrderDetailsModal from './order_details_modal'
import AddSalesOrderModal from './add_sales_order_modal'
import { Add } from '@carbon/icons-react'

interface PaymentMethod {
    method: string
    amount: number
}

const SalesOrdersPage = () => {
    const [orders, setOrders] = useState<SalesOrder[]>([])
    const [page, setPage] = useState(1)
    const [pageSize, setPageSize] = useState(10)
    const [selectedOrder, setSelectedOrder] = useState<SalesOrder | null>(null)
    const [selectedOrderItems, setSelectedOrderItems] = useState<SalesOrderItem[]>([])
    const [loading, setLoading] = useState(false)
    const [totalOrders, setTotalOrders] = useState(0)
    const [showAddModal, setShowAddModal] = useState(false)

    const fetchOrders = async (page: number, size: number) => {
        setLoading(true)
        try {
            const offset = (page - 1) * size
            const result = await gql(GetSalesOrdersDocument, { first: size, offset })

            if (result.salesOrders) {
                setOrders(result.salesOrders as SalesOrder[])
                setTotalOrders(result.totalSalesOrders)
            }
        } catch (error) {
            console.error('Error fetching orders:', error)
        } finally {
            setLoading(false)
        }
    }

    useEffect(() => {
        fetchOrders(page, pageSize)
    }, [page, pageSize])

    const handleOrderClick = (orderId: string) => {
        const order = orders.find(o => o.id === orderId)
        if (order) {
            setSelectedOrder(order)
            setSelectedOrderItems(order.items)
        }
    }

    const handleCloseModal = () => {
        setSelectedOrder(null)
        setSelectedOrderItems([])
        fetchOrders(page, pageSize) // Refresh the order list to reflect any changes
    }

    const handleAddOrder = () => {
        setShowAddModal(true)
    }

    const handleAddOrderComplete = () => {
        setShowAddModal(false)
        fetchOrders(page, pageSize) // Refresh to show the new order
    }

    const formatPaymentMethods = (paymentMethodJson: string): string => {
        try {
            const paymentMethods: PaymentMethod[] = JSON.parse(paymentMethodJson)
            return paymentMethods
                .filter(pm => pm.amount > 0)
                .map(pm => `${pm.method} (${formatCurrency(parseFloat(pm.amount.toString()))})`)
                .join(', ')
        } catch (error) {
            console.error('Error parsing payment method JSON:', error)
            return paymentMethodJson // Return the original string if parsing fails
        }
    }

    const headers = [
        { key: 'id', header: 'Order ID' },
        { key: 'totalAmount', header: 'Total Amount' },
        { key: 'paidAmount', header: 'Paid Amount' },
        { key: 'customerName', header: 'Customer' },
        { key: 'createdAt', header: 'Created At' },
        { key: 'status', header: 'Status' },
    ]

    const rows = orders.map(order => ({
        id: order.id,
        totalAmount: formatCurrency(parseFloat(order.totalAmount)),
        paidAmount: formatCurrency(parseFloat(order.totalPaidAmount?.toString() || "0")),
        customerName: order.customerName,
        createdAt: new Date(order.createdAt ?? 0).toLocaleString(),
        status: order.state,
    }))

    if (loading && orders.length === 0) {
        return <div className="p-4">Loading orders...</div>
    }

    return (
        <Content className='min-h-[calc(100dvh-3rem)] p-0'>
            <div className="p-4">
                <div className="flex justify-between items-center mb-4">
                    <h1 className="text-2xl font-bold">Sales Orders</h1>
                    <Button
                        kind="primary"
                        renderIcon={Add}
                        onClick={handleAddOrder}>
                        Add Order
                    </Button>
                </div>

                <DataTable rows={rows} headers={headers} isSortable>
                    {({ rows, headers, getTableProps, getHeaderProps, getRowProps }) => (
                        <TableContainer>
                            <Table {...getTableProps()}>
                                <TableHead>
                                    <TableRow>
                                        {headers.map((header) => (
                                            <TableHeader
                                                {...getHeaderProps({ header })}
                                                key={header.key}
                                                onClick={(e: React.MouseEvent) => getHeaderProps({ header }).onClick(e as any)}>
                                                {header.header}
                                            </TableHeader>
                                        ))}
                                    </TableRow>
                                </TableHead>
                                <TableBody>
                                    {rows.map((row) => (
                                        <TableRow {...getRowProps({ row })} key={row.id} onClick={() => handleOrderClick(row.id as string)}>
                                            {row.cells.map((cell) => (
                                                <TableCell key={cell.id}>{cell.value}</TableCell>
                                            ))}
                                        </TableRow>
                                    ))}
                                </TableBody>
                            </Table>
                        </TableContainer>
                    )}
                </DataTable>
                <Pagination
                    backwardText="Previous page"
                    forwardText="Next page"
                    itemsPerPageText="Items per page:"
                    page={page}
                    pageNumberText="Page Number"
                    pageSize={pageSize}
                    pageSizes={[10, 20, 30, 40, 50]}
                    totalItems={totalOrders}
                    onChange={({ page, pageSize }) => {
                        setPage(page)
                        setPageSize(pageSize)
                    }}
                />
            </div>

            {selectedOrder && (
                <OrderDetailsModal
                    open={!!selectedOrder}
                    onRequestClose={handleCloseModal}
                    order={selectedOrder}
                    orderItems={selectedOrderItems}
                />
            )}

            <AddSalesOrderModal
                isOpen={showAddModal}
                onClose={() => setShowAddModal(false)}
                onSave={handleAddOrderComplete}
            />
        </Content>
    )
}

export default SalesOrdersPage
