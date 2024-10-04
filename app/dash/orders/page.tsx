'use client'
import { Receipt } from '@carbon/icons-react'
import { Content, SideNav, SideNavItems, SideNavLink, DataTable, TableContainer, Table, TableHead, TableRow, TableHeader, TableBody, TableCell, Pagination } from '@carbon/react'
import React, { useEffect, useState, useCallback } from 'react'
import { useDb } from '@/components/providers/drizzle_provider'
import { Order, OrderItem, ordersTable, orderItemsTable } from '@/lib/db/sqlite/schema'
import OrderDetailsModal from './order_details_modal'
import { desc, eq } from 'drizzle-orm'
import { money } from '@/lib/util/money'

interface PaymentMethod {
  method: string;
  amount: number;
}

const Orders = () => {
  const db = useDb()
  const [orders, setOrders] = useState<Order[]>([])
  const [page, setPage] = useState(1)
  const [pageSize, setPageSize] = useState(10)
  const [selectedOrder, setSelectedOrder] = useState<Order | null>(null)
  const [selectedOrderItems, setSelectedOrderItems] = useState<OrderItem[]>([])


  const fetchOrders = useCallback(async () => {
    const result = await db.select().from(ordersTable).orderBy(desc(ordersTable.createdAt))
    setOrders(result)
  }, [db])

  useEffect(() => {
    fetchOrders()
  }, [fetchOrders])

  const handleOrderClick = async (orderId: string) => {
    const order = orders.find(o => o.id === orderId)
    if (order) {
      setSelectedOrder(order)
      const items = await db.select().from(orderItemsTable).where(eq(orderItemsTable.orderId, order.id))
      setSelectedOrderItems(items)
    }
  }

  const handleCloseModal = () => {
    setSelectedOrder(null)
    setSelectedOrderItems([])
    fetchOrders() // Refresh the order list to reflect any changes
  }

  const formatPaymentMethods = (paymentMethodJson: string): string => {
    try {
      const paymentMethods: PaymentMethod[] = JSON.parse(paymentMethodJson);
      return paymentMethods
        .filter(pm => pm.amount > 0)
        .map(pm => `${pm.method} (Rs. ${pm.amount.toFixed(2)})`)
        .join(', ');
    } catch (error) {
      console.error('Error parsing payment method JSON:', error);
      return paymentMethodJson; // Return the original string if parsing fails
    }
  }

  const headers = [
    { key: 'id', header: 'Order ID' },
    { key: 'totalAmount', header: 'Total Amount' },
    { key: 'paymentMethod', header: 'Payment Method' },
    { key: 'createdAt', header: 'Created At' },
    { key: 'status', header: 'Status' },
  ]

  const rows = orders.map(order => ({
    id: order.id,
    totalAmount: money(order.totalAmount ?? 0, 'INR').format(),
    // payment_method: formatPaymentMethods(order.paymentMethod || ''),
    createdAt: new Date(order.createdAt ?? 0).toLocaleString(),
    status: order.state,
  }))

  const paginatedRows = rows.slice((page - 1) * pageSize, page * pageSize)

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
          <DataTable rows={paginatedRows} headers={headers}>
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
            totalItems={rows.length}
            onChange={({ page, pageSize }) => {
              setPage(page);
              setPageSize(pageSize);
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
