'use client'
import { ShoppingCart } from '@carbon/icons-react'
import { Content, SideNav, SideNavItems, SideNavLink } from '@carbon/react'
import React, { useEffect, useState, useCallback } from 'react'
import { useDb } from '@/components/providers/drizzle_provider'
import { PurchaseOrder, purchaseOrdersTable } from '@/lib/db/sqlite/schema'
import { desc, sql } from 'drizzle-orm'
import { money } from '@/lib/util/money'
import PurchaseDetailsModal from './purchase_details_modal'
import AddPurchaseModal from './add_purchase_modal'
import DataTable from '@/components/ui/DataTable'

interface TableRow extends PurchaseOrder {
  totalAmountTransformed: string;
  createdAtTransformed: string;
}

const Purchases = () => {
  const db = useDb()
  const [purchases, setPurchases] = useState<PurchaseOrder[]>([])
  const [page, setPage] = useState(1)
  const [pageSize, setPageSize] = useState(10)
  const [totalItems, setTotalItems] = useState(0)
  const [selectedPurchase, setSelectedPurchase] = useState<PurchaseOrder | null>(null)
  const [isAddModalOpen, setIsAddModalOpen] = useState(false)
  const [loading, setLoading] = useState(false)

  const fetchPurchases = useCallback(async (page: number, size: number) => {
    const offset = (page - 1) * size
    const result = await db.select().from(purchaseOrdersTable).orderBy(desc(purchaseOrdersTable.createdAt)).limit(size).offset(offset)
    const [{ count }] = await db.select({ count: sql<number>`count(*)` }).from(purchaseOrdersTable)
    setPurchases(result)
    setTotalItems(count)
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
    fetchPurchases(page, pageSize)
  }

  const handleAddPurchase = () => {
    setIsAddModalOpen(true)
  }

  const handleAddModalClose = () => {
    setIsAddModalOpen(false)
    fetchPurchases(page, pageSize)
  }

  const handleEditPurchase = (purchase: PurchaseOrder) => {
    setSelectedPurchase(purchase)
  }

  const handleDeletePurchase = async (purchase: PurchaseOrder) => {
    if (confirm(`Are you sure you want to delete purchase ${purchase.id}?`)) {
      await db.delete(purchaseOrdersTable).where(sql`${purchaseOrdersTable.id} = ${purchase.id}`)
      fetchPurchases(page, pageSize)
    }
  }

  const headers = [
    { key: 'id', header: 'Purchase ID' },
    { key: 'supplierName', header: 'Supplier' },
    { key: 'totalAmountTransformed', header: 'Total Amount' },
    { key: 'state', header: 'State' },
    { key: 'createdAtTransformed', header: 'Created At' },
  ]

  const tableRows: TableRow[] = purchases.map(purchase => ({
    ...purchase,
    totalAmountTransformed: money(purchase.totalAmount ?? 0, 'INR').format(),
    createdAtTransformed: new Date(purchase.createdAt ?? 0).toLocaleString(),
  }))

  return (
    <>
      <SideNav isFixedNav expanded={true} isChildOfHeader={false} aria-label="Side navigation">
        <SideNavItems>
          <SideNavLink renderIcon={ShoppingCart} large href='#'>Overview</SideNavLink>
        </SideNavItems>
      </SideNav>
      <Content className='min-h-[calc(100dvh-3rem)] p-0'>
        <div className="p-4">
          <DataTable<TableRow>
            title="Purchases"
            description="List of all purchase orders"
            headers={headers}
            tableRows={tableRows}
            loading={loading}
            totalItems={totalItems}
            currentPage={page}
            pageSize={pageSize}
            pageSizes={[10, 20, 30, 40, 50]}
            onPageChange={(newPage, newPageSize) => {
              setPage(newPage)
              setPageSize(newPageSize)
              fetchPurchases(newPage, newPageSize)
            }}
            onAddClick={handleAddPurchase}
            onEditClick={handleEditPurchase}
            onDeleteClick={handleDeletePurchase}
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
