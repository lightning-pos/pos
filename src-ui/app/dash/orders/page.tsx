'use client'
import { Receipt } from '@carbon/icons-react'
import { Content, SideNav, SideNavItems, SideNavLink, DataTable, TableContainer, Table, TableHead, TableRow, TableHeader, TableBody, TableCell, Pagination } from '@carbon/react'
import React, { useEffect, useState, useCallback } from 'react'
import OrderDetailsModal from './order_details_modal'
import { invoke } from '@tauri-apps/api/core'

interface SalesOrderState {
    DRAFT: 'DRAFT'
    COMPLETED: 'COMPLETED'
    CANCELLED: 'CANCELLED'
}

interface SalesOrder {
    id: string
    customerId: string
    customerName: string
    customerPhoneNumber: string
    orderDate: string
    netAmount: number
    discAmount: number
    taxableAmount: number
    taxAmount: number
    totalAmount: number
    state: keyof SalesOrderState
    createdAt: string
    updatedAt: string
    customer: {
        id: string
        fullName: string
        phone: string
    }
    items: SalesOrderItem[]
}

interface SalesOrderItem {
    id: string
    orderId: string
    itemId: string
    itemName: string
    quantity: number
    priceAmount: number
    taxAmount: number
    createdAt: string
    updatedAt: string
}

interface PaymentMethod {
    method: string
    amount: number
}

const formatPrice = (price: number): string => {
    return new Intl.NumberFormat('en-IN', {
        style: 'currency',
        currency: 'INR'
    }).format(price / 100);
};

const Orders = () => {
    const [orders, setOrders] = useState<SalesOrder[]>([])
    const [page, setPage] = useState(1)
    const [pageSize, setPageSize] = useState(10)
    const [selectedOrder, setSelectedOrder] = useState<SalesOrder | null>(null)
    const [selectedOrderItems, setSelectedOrderItems] = useState<SalesOrderItem[]>([])
    const [loading, setLoading] = useState(false)
    const [totalOrders, setTotalOrders] = useState(0)

    const fetchOrders = useCallback(async (page: number, size: number) => {
        setLoading(true)
        try {
            const offset = (page - 1) * size
            const result: Array<{ salesOrders: SalesOrder[], totalOrders: number }> = await invoke('graphql', {
                query: `#graphql
                    query {
                        salesOrders(first: ${size}, offset: ${offset}) {
                            id
                            customerId
                            customerName
                            customerPhoneNumber
                            orderDate
                            netAmount
                            discAmount
                            taxableAmount
                            taxAmount
                            totalAmount
                            state
                            createdAt
                            updatedAt
                            customer {
                                id
                                fullName
                                phone
                            }
                            items {
                                id
                                orderId
                                itemId
                                itemName
                                quantity
                                priceAmount
                                taxAmount
                                createdAt
                                updatedAt
                            }
                        }
                    }
                `
            })
            setOrders(result[0].salesOrders)
            setTotalOrders(result[0].totalOrders)
        } catch (error) {
            console.error('Error fetching orders:', error)
        } finally {
            setLoading(false)
        }
    }, [])

    useEffect(() => {
        fetchOrders(page, pageSize)
    }, [fetchOrders, page, pageSize])

    const handleOrderClick = async (orderId: string) => {
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

    const formatPaymentMethods = (paymentMethodJson: string): string => {
        try {
            const paymentMethods: PaymentMethod[] = JSON.parse(paymentMethodJson);
            return paymentMethods
                .filter(pm => pm.amount > 0)
                .map(pm => `${pm.method} (${formatPrice(pm.amount)})`)
                .join(', ');
        } catch (error) {
            console.error('Error parsing payment method JSON:', error);
            return paymentMethodJson; // Return the original string if parsing fails
        }
    }

    const headers = [
        { key: 'id', header: 'Order ID' },
        { key: 'totalAmount', header: 'Total Amount' },
        { key: 'customerName', header: 'Customer' },
        { key: 'createdAt', header: 'Created At' },
        { key: 'status', header: 'Status' },
    ]

    const rows = orders.map(order => ({
        id: order.id,
        totalAmount: formatPrice(order.totalAmount ?? 0),
        customerName: order.customerPhoneNumber,
        createdAt: new Date(order.createdAt ?? 0).toLocaleString(),
        status: order.state,
    }))

    return (
        <>
            <SideNav isFixedNav expanded={true} isChildOfHeader={false} aria-label="Side navigation">
                <SideNavItems>
                    <SideNavLink renderIcon={Receipt} large href='#'>Overview</SideNavLink>
                </SideNavItems>
            </SideNav>
            <Content className='min-h-[calc(100dvh-3rem)] p-0'>
                <div className="p-4">
                    <h1 className="text-2xl font-bold mb-4">Orders</h1>
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
                            setPage(page);
                            setPageSize(pageSize);
                            fetchOrders(page, pageSize)
                        }}
                    />
                </div>
            </Content>

            {selectedOrder && (
                <OrderDetailsModal
                    open={!!selectedOrder}
                    onRequestClose={handleCloseModal}
                    order={selectedOrder}
                    orderItems={selectedOrderItems}
                />
            )}
        </>
    )
}

export default Orders
