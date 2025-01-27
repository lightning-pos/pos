'use client'
import React, { useState, useEffect, useCallback } from 'react'
import { Content } from '@carbon/react'
import DataTable from '@/components/ui/DataTable'
import AddTaxModal from './add_tax_modal'
import EditTaxModal from './edit_tax_modal'
import DeleteTaxModal from './delete_tax_modal'
import { Tax, taxesTable } from '@/lib/db/sqlite/schema'
import { useDb } from '@/components/providers/drizzle_provider'

const Taxes = () => {
  const db = useDb()

  // Modal States
  const [taxes, setTaxes] = useState<Tax[]>([])
  const [selectedTax, setSelectedTax] = useState<Tax | null>()

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
      const result = await db.select().from(taxesTable).limit(size).offset(offset)
      setTaxes(result)
    } catch (error) {
      console.error('Error fetching taxes:', error)
    } finally {
      setLoading(false)
    }
  }, [db])

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

  return (
    <Content className='min-h-[calc(100dvh-3rem)] p-0 flex flex-col'>
      <div className="p-4 flex-grow flex flex-col" style={{ height: 'calc(100vh - 12rem)' }}>
        <DataTable<Tax>
          title="Tax"
          description="Manage your taxes here. You can add, edit, or delete taxes as needed."
          headers={headers}
          tableRows={taxes}
          loading={loading}
          totalItems={taxes.length}
          currentPage={currentPage}
          pageSize={pageSize}
          pageSizes={[10, 20, 30, 40, 50]}
          onPageChange={(page, pageSize) => {
            setCurrentPage(page)
            setPageSize(pageSize)
          }}
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
          onRequestClose={() => {
            setIsEditModalOpen(false)
            setSelectedTax(null)
          }}
          onRequestSubmit={() => {
            fetchTaxes(currentPage, pageSize)
            setIsEditModalOpen(false)
            setSelectedTax(null)
          }}
          tax={selectedTax}
        />
      )}
      {selectedTax && (
        <DeleteTaxModal
          open={isDeleteModalOpen}
          onRequestClose={() => {
            setIsDeleteModalOpen(false)
            setSelectedTax(null)
          }}
          onRequestSubmit={() => {
            fetchTaxes(currentPage, pageSize)
            setIsDeleteModalOpen(false)
            setSelectedTax(null)
          }}
          tax={selectedTax}
        />
      )}
    </Content>
  )
}

export default Taxes
