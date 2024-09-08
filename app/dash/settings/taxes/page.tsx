'use client'
import React, { useState, useEffect, useCallback } from 'react'
import { Add } from '@carbon/icons-react'
import { Content, DataTable, Table, TableHead, TableRow, TableHeader, TableBody, TableCell, Pagination, DataTableSkeleton, Button, Modal, TextInput, Form, TableToolbar, TableToolbarContent, TableContainer, OverflowMenu, OverflowMenuItem } from '@carbon/react'
import { db } from '@/components/providers/system_provider'
import { Tax as TaxSchema } from '@/lib/powersync/app_schema'
import { uid } from 'uid'

const Taxes = () => {
  const [currentPage, setCurrentPage] = useState(1)
  const [pageSize, setPageSize] = useState(10)
  const [taxes, setTaxes] = useState<TaxSchema[]>([])
  const [loading, setLoading] = useState(true)
  const [isModalOpen, setIsModalOpen] = useState(false)
  const [editingTax, setEditingTax] = useState<TaxSchema | null>(null)
  const [isDeleteModalOpen, setIsDeleteModalOpen] = useState(false)

  useEffect(() => {
    const fetchTaxes = async () => {
      setLoading(true)
      try {
        const result: TaxSchema[] = await db.selectFrom('taxes').selectAll().execute()
        setTaxes(result)
      } catch (error) {
        console.error('Error fetching taxes:', error)
      } finally {
        setLoading(false)
      }
    }

    fetchTaxes()
  }, [])

  const headers = [
    { key: 'name', header: 'Name' },
    { key: 'rate', header: 'Rate (%)' },
    { key: 'description', header: 'Description' },
  ]

  const startIndex = (currentPage - 1) * pageSize
  const endIndex = startIndex + pageSize
  const paginatedTaxes = taxes.slice(startIndex, endIndex)

  const fetchTaxes = useCallback(async () => {
    try {
      const result: TaxSchema[] = await db.selectFrom('taxes').selectAll().execute()
      setTaxes(result)
    } catch (error) {
      console.error('Error fetching taxes:', error)
    }
  }, [])

  const handleSaveTax = useCallback(async (e: React.FormEvent) => {
    e.preventDefault()
    if (!editingTax) return
    try {
      if (editingTax.id) {
        await db.updateTable('taxes')
          .set({ name: editingTax.name, rate: editingTax.rate, description: editingTax.description })
          .where('id', '=', editingTax.id)
          .execute()
      } else {
        await db.insertInto('taxes').values({ ...editingTax, id: uid() }).execute()
      }
      setIsModalOpen(false)
      setEditingTax(null)
      fetchTaxes()
    } catch (error) {
      console.error('Error saving tax:', error)
    }
  }, [editingTax, fetchTaxes])

  const handleDeleteTax = useCallback(async () => {
    if (!editingTax?.id) return
    try {
      await db.deleteFrom('taxes')
        .where('id', '=', editingTax.id)
        .execute()
      setIsDeleteModalOpen(false)
      setEditingTax(null)
      fetchTaxes()
    } catch (error) {
      console.error('Error deleting tax:', error)
    }
  }, [editingTax, fetchTaxes])

  return (
    <Content className='min-h-[calc(100dvh-3rem)] p-0 flex flex-col'>
      <div className="p-4 flex-grow flex flex-col" style={{ height: 'calc(100vh - 12rem)' }}>
        {loading ? (
          <DataTableSkeleton headers={headers} rowCount={pageSize} />
        ) : (
          <TableContainer
            title="Taxes"
            description="Manage your taxes here. You can add, edit, or delete taxes as needed."
          >
            <TableToolbar>
              <TableToolbarContent>
                <Button
                  renderIcon={Add}
                  onClick={() => {
                    setEditingTax({ id: '', name: '', rate: 0, description: '' })
                    setIsModalOpen(true)
                  }}
                >
                  Add Tax
                </Button>
              </TableToolbarContent>
            </TableToolbar>
            <DataTable rows={paginatedTaxes} headers={headers}>
              {({ rows, headers, getTableProps }) => (
                <Table {...getTableProps()}>
                  <TableHead>
                    <TableRow>
                      {headers.map((header) => (
                        <TableHeader key={header.key}>{header.header}</TableHeader>
                      ))}
                      <TableHeader key="actions" style={{ width: '8rem' }}>Actions</TableHeader>
                    </TableRow>
                  </TableHead>
                  <TableBody>
                    {rows.map((row) => (
                      <TableRow key={row.id}>
                        {row.cells.map((cell) => (
                          <TableCell key={cell.id}>{cell.value}</TableCell>
                        ))}
                        <TableCell>
                          <OverflowMenu label="Actions">
                            <OverflowMenuItem
                              itemText="Edit"
                              onClick={() => {
                                const tax = taxes.find(t => t.id === row.id)
                                setEditingTax(tax || null)
                                setIsModalOpen(true)
                              }}
                            />
                            <OverflowMenuItem
                              itemText="Delete"
                              hasDivider
                              isDelete
                              onClick={() => {
                                const tax = taxes.find(t => t.id === row.id)
                                setEditingTax(tax || null)
                                setIsDeleteModalOpen(true)
                              }}
                            />
                          </OverflowMenu>
                        </TableCell>
                      </TableRow>
                    ))}
                  </TableBody>
                </Table>
              )}
            </DataTable>
            <Pagination
              totalItems={taxes.length}
              backwardText="Previous page"
              forwardText="Next page"
              pageSize={pageSize}
              pageSizes={[10, 20, 30, 40, 50]}
              itemsPerPageText="Items per page:"
              onChange={({ page, pageSize }) => {
                setCurrentPage(page)
                setPageSize(pageSize)
              }}
            />
          </TableContainer>
        )}
      </div>

      <Modal
        open={isModalOpen}
        onRequestClose={() => {
          setIsModalOpen(false)
          setEditingTax(null)
        }}
        modalHeading={editingTax?.id ? "Edit Tax" : "Add New Tax"}
        primaryButtonText="Save"
        onRequestSubmit={handleSaveTax}
      >
        <Form onSubmit={handleSaveTax} className='flex flex-col gap-4'>
          <TextInput
            id="tax-name"
            labelText="Tax Name"
            value={editingTax?.name || ''}
            onChange={(e) => setEditingTax(prev => prev ? { ...prev, name: e.target.value } : null)}
            required
          />
          <TextInput
            id="tax-rate"
            labelText="Tax Rate (%)"
            type="number"
            value={editingTax?.rate || ''}
            onChange={(e) => setEditingTax(prev => prev ? { ...prev, rate: parseFloat(e.target.value) } : null)}
            required
          />
          <TextInput
            id="tax-description"
            labelText="Description"
            value={editingTax?.description || ''}
            onChange={(e) => setEditingTax(prev => prev ? { ...prev, description: e.target.value } : null)}
          />
        </Form>
      </Modal>

      <Modal
        open={isDeleteModalOpen}
        onRequestClose={() => {
          setIsDeleteModalOpen(false)
          setEditingTax(null)
        }}
        modalHeading="Delete Tax"
        primaryButtonText="Delete"
        secondaryButtonText="Cancel"
        danger
        onRequestSubmit={handleDeleteTax}
      >
        <p>Are you sure you want to delete the tax &quot;{editingTax?.name}&quot;? This action cannot be undone.</p>
      </Modal>
    </Content>
  )
}

export default Taxes
