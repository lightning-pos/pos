'use client'
import { ShoppingCart } from '@carbon/icons-react'
import { Content, SideNav, SideNavItems, SideNavLink, DataTable, TableContainer, Table, TableHead, TableRow, TableHeader, TableBody, TableCell, Pagination, Button } from '@carbon/react'
import React, { useEffect, useState, useCallback } from 'react'
import { useDb } from '@/components/providers/drizzle_provider'
import { PurchaseOrder, purchaseOrdersTable } from '@/lib/db/sqlite/schema'
import { desc } from 'drizzle-orm'
import { money } from '@/lib/util/money'
import PurchaseDetailsModal from './purchase_details_modal'
import AddPurchaseModal from './add_purchase_modal'

const Purchases = () => {
  const db = useDb()
  const [purchases, setPurchases] = useState<PurchaseOrder[]>([])
  const [page, setPage] = useState(1)
  const [pageSize, setPageSize] = useState(10)
  const [selectedPurchase, setSelectedPurchase] = useState<PurchaseOrder | null>(null)
  const [isAddModalOpen, setIsAddModalOpen] = useState(false)

  const fetchPurchases = useCallback(async (page: number, size: number) => {
    const offset = (page - 1) * size
    const result = await db.select().from(purchaseOrdersTable).orderBy(desc(purchaseOrdersTable.createdAt)).limit(size).offset(offset)
    setPurchases(result)
  }, [db])

  useEffect(() => {
    fetchPurchases(page, pageSize)
  }, [fetchPurchases, page, pageSize])

  const handlePurchaseClick = (purchaseId: string) => {
    const purchase = purchases.find(p => p.id === purchaseId)
    if (purchase) {
      setSelectedPurchase(purchase)
    }
  }

  const handleCloseModal = () => {
    setSelectedPurchase(null)
    fetchPurchases(page, pageSize) // Refresh the purchase list to reflect any changes
  }

  const handleAddPurchase = () => {
    setIsAddModalOpen(true)
  }

  const handleAddModalClose = () => {
    setIsAddModalOpen(false)
    fetchPurchases(page, pageSize) // Refresh the purchase list after adding a new purchase
  }

  const headers = [
    { key: 'id', header: 'Purchase ID' },
    { key: 'supplierName', header: 'Supplier' },
    { key: 'totalAmount', header: 'Total Amount' },
    { key: 'createdAt', header: 'Created At' },
    { key: 'state', header: 'Status' },
  ]

  const rows = purchases.map(purchase => ({
    id: purchase.id,
    supplierName: purchase.supplierName,
    totalAmount: money(purchase.totalAmount ?? 0, 'INR').format(),
    createdAt: new Date(purchase.createdAt ?? 0).toLocaleString(),
    state: purchase.state,
  }))

  const paginatedRows = rows.slice((page - 1) * pageSize, page * pageSize)

  return (
    <>
      <SideNav isFixedNav expanded={true} isChildOfHeader={false} aria-label="Side navigation">
        <SideNavItems>
          <SideNavLink renderIcon={ShoppingCart} large href='#'>Overview</SideNavLink>
        </SideNavItems>
      </SideNav>
      <Content className='min-h-[calc(100dvh-3rem)] p-0'>
        <div className="p-4">
          <div className="flex justify-between items-center mb-4">
            <h1 className="text-2xl font-bold">Purchases</h1>
            <Button onClick={handleAddPurchase}>Add Purchase</Button>
          </div>
          <DataTable rows={paginatedRows} headers={headers}>
            {({ rows, headers, getTableProps, getHeaderProps, getRowProps }) => (
              <TableContainer>
                <Table {...getTableProps()}>
                  <TableHead>
                    <TableRow>
                      {headers.map((header) => (
                        <TableHeader {...getHeaderProps({ header })} key={header.key}>
                          {header.header}
                        </TableHeader>
                      ))}
                    </TableRow>
                  </TableHead>
                  <TableBody>
                    {rows.map((row) => (
                      <TableRow {...getRowProps({ row })} key={row.id} onClick={() => handlePurchaseClick(row.id as string)}>
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
              fetchPurchases(page, pageSize)
            }}
          />
        </div>
      </Content>

      {selectedPurchase && (
        <PurchaseDetailsModal
          open={!!selectedPurchase}
          onRequestClose={handleCloseModal}
          purchase={selectedPurchase}
        />
      )}

      <AddPurchaseModal
        open={isAddModalOpen}
        onRequestClose={handleAddModalClose}
      />
    </>
  )
}

export default Purchases
