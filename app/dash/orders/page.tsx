'use client'
import { Receipt } from '@carbon/icons-react'
import { Content, SideNav, SideNavItems, SideNavLink, DataTable, TableContainer, Table, TableHead, TableRow, TableHeader, TableBody, TableCell, Pagination } from '@carbon/react'
import React, { useEffect, useState } from 'react'
import { db } from '@/components/providers/system_provider'
import { Order } from '@/lib/powersync/app_schema'

const Orders = () => {
  const [orders, setOrders] = useState<Order[]>([])
  const [page, setPage] = useState(1)
  const [pageSize, setPageSize] = useState(10)

  useEffect(() => {
    const fetchOrders = async () => {
      const result = await db
        .selectFrom('orders')
        .selectAll()
        .orderBy('created_at', 'desc')
        .execute()
      setOrders(result)
    }
    fetchOrders()
  }, [])

  const headers = [
    { key: 'id', header: 'Order ID' },
    { key: 'total_amount', header: 'Total Amount' },
    { key: 'payment_method', header: 'Payment Method' },
    { key: 'created_at', header: 'Created At' },
  ]

  const rows = orders.map(order => ({
    id: order.id,
    total_amount: `Rs. ${(order.total_amount ?? 0).toFixed(2)}`,
    payment_method: order.payment_method,
    created_at: new Date(order.created_at ?? 0).toLocaleString(),
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
                        <TableHeader {...getHeaderProps({ header })}>
                          {header.header}
                        </TableHeader>
                      ))}
                    </TableRow>
                  </TableHead>
                  <TableBody>
                    {rows.map((row) => (
                      <TableRow {...getRowProps({ row })}>
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
    </>
  )
}

export default Orders
