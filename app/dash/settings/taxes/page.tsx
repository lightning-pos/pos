'use client'
import React, { useEffect } from 'react'
import { Content } from '@carbon/react'
import { TaxesProvider, useTaxes } from './taxes_context'
import DataTable from '@/components/ui/DataTable'
import SaveTaxModal from './save_tax_modal'
import DeleteTaxModal from './delete_tax_modal'
import { taxesTable } from '@/lib/pglite/schema'

type TaxSchema = typeof taxesTable.$inferSelect

const TaxesContent = () => {
  const {
    taxes,
    loading,
    currentPage,
    pageSize,
    setCurrentPage,
    setPageSize,
    setEditingTax,
    setIsModalOpen,
    setIsDeleteModalOpen
  } = useTaxes()

  const headers = [
    { key: 'name', header: 'Name' },
    { key: 'rate', header: 'Rate (%)' },
    { key: 'description', header: 'Description' }
  ]

  const handleAddTax = () => {
    console.log('Adding new tax')
    setEditingTax({ id: '', name: '', rate: 0, description: '' } as TaxSchema)
    setIsModalOpen(true)
  }

  const handleEditTax = (tax: TaxSchema) => {
    console.log('Editing tax:', tax)
    setEditingTax(tax)
    setIsModalOpen(true)
  }

  const handleDeleteTax = (tax: TaxSchema) => {
    console.log('Deleting tax:', tax)
    setEditingTax(tax)
    setIsDeleteModalOpen(true)
  }

  return (
    <Content className='min-h-[calc(100dvh-3rem)] p-0 flex flex-col'>
      <div className="p-4 flex-grow flex flex-col" style={{ height: 'calc(100vh - 12rem)' }}>
        <DataTable<TaxSchema>
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
            console.log('Page changed to:', page, 'Page size:', pageSize)
            setCurrentPage(page)
            setPageSize(pageSize)
          }}
          onAddClick={handleAddTax}
          onEditClick={handleEditTax}
          onDeleteClick={handleDeleteTax}
        />
      </div>
      <SaveTaxModal />
      <DeleteTaxModal />
    </Content>
  )
}

const Taxes = () => {
  return (
    <TaxesProvider>
      <TaxesContent />
    </TaxesProvider>
  )
}

export default Taxes
