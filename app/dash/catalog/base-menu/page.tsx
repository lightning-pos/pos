'use client'
import React, { useState, useEffect, useCallback } from 'react'
import { Add, Label } from '@carbon/icons-react'
import { Content, DataTable, Table, TableHead, TableRow, TableHeader, TableBody, TableCell, Pagination, DataTableSkeleton, Button, Modal, TextInput, Form, TableToolbar, TableToolbarContent, TableContainer, TextArea, OverflowMenu, OverflowMenuItem, Select, SelectItem, NumberInput, MultiSelect } from '@carbon/react'
import { db } from '@/components/providers/system_provider'
import { Item as ItemSchema, Category as CategorySchema, Tax as TaxSchema } from '@/lib/powersync/app_schema'
import { uid } from 'uid'

const BaseMenu = () => {
  const [currentPage, setCurrentPage] = useState(1)
  const [pageSize, setPageSize] = useState(10)
  const [items, setItems] = useState<ItemSchema[]>([])
  const [categories, setCategories] = useState<CategorySchema[]>([])
  const [loading, setLoading] = useState(true)
  const [isModalOpen, setIsModalOpen] = useState(false)
  const [editingItem, setEditingItem] = useState<Partial<ItemSchema> | null>(null)
  const [isDeleteModalOpen, setIsDeleteModalOpen] = useState(false)
  const [taxes, setTaxes] = useState<TaxSchema[]>([])

  const fetchData = useCallback(async () => {
    setLoading(true)
    try {
      const itemsResult: ItemSchema[] = await db.selectFrom('items').selectAll().execute()
      setItems(itemsResult)
      const categoriesResult: CategorySchema[] = await db.selectFrom('item_categories').selectAll().execute()
      setCategories(categoriesResult)
      const taxesResult: TaxSchema[] = await db.selectFrom('taxes').selectAll().execute()
      console.log(taxesResult)
      setTaxes(taxesResult)
    } catch (error) {
      console.error('Error fetching data:', error)
    } finally {
      setLoading(false)
    }
  }, [])

  useEffect(() => {
    fetchData()
  }, [fetchData])

  const headers = [
    { key: 'name', header: 'Name' },
    { key: 'description', header: 'Description' },
    { key: 'price', header: 'Price' },
    { key: 'category', header: 'Category' },
    { key: 'taxes', header: 'Taxes' },
  ]

  const startIndex = (currentPage - 1) * pageSize
  const endIndex = startIndex + pageSize
  const paginatedItems = items.slice(startIndex, endIndex).map(item => ({
    ...item,
    category: categories.find(cat => cat.id === item.item_category_id)?.name || 'Unknown',
    taxes: item.tax_ids ? taxes.filter(tax => item.tax_ids?.split(',').includes(tax.id)).map(tax => tax.name).join(', ') : ''
  }))

  const handleSaveItem = async (e: React.FormEvent) => {
    e.preventDefault()
    if (!editingItem) return
    try {
      const itemData = {
        ...editingItem,
        tax_ids: Array.isArray(editingItem.tax_ids) ? editingItem.tax_ids.join(',') : editingItem.tax_ids || ''
      }
      if (itemData.id) {
        await db.updateTable('items')
          .set(itemData)
          .where('id', '=', itemData.id)
          .execute()
      } else {
        await db.insertInto('items').values({ ...itemData, id: uid() }).execute()
      }
      setIsModalOpen(false)
      setEditingItem(null)
      fetchData()
    } catch (error) {
      console.error('Error saving item:', error)
    }
  }

  const handleDeleteItem = async () => {
    if (!editingItem?.id) return
    try {
      await db.deleteFrom('items')
        .where('id', '=', editingItem.id)
        .execute()
      setIsDeleteModalOpen(false)
      setEditingItem(null)
      fetchData()
    } catch (error) {
      console.error('Error deleting item:', error)
    }
  }

  return (
    <Content className='min-h-[calc(100dvh-3rem)] p-0 flex flex-col'>
      <div className="p-4 flex-grow flex flex-col" style={{ height: 'calc(100vh - 12rem)' }}>
        {loading ? (
          <DataTableSkeleton headers={headers} rowCount={pageSize} />
        ) : (
          <TableContainer
            title="Menu Items"
            description="Manage your menu items here. You can add, edit, or delete items as needed."
          >
            <TableToolbar>
              <TableToolbarContent>
                <Button
                  renderIcon={Add}
                  onClick={() => {
                    setEditingItem({ name: '', description: '', price: 0, item_category_id: '', tax_ids: '' }) // Changed tax_ids to empty string
                    setIsModalOpen(true)
                  }}
                >
                  Add Item
                </Button>
              </TableToolbarContent>
            </TableToolbar>
            <DataTable rows={paginatedItems} headers={headers}>
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
                                const item = items.find(i => i.id === row.id)
                                setEditingItem(item || null)
                                setIsModalOpen(true)
                              }}
                            />
                            <OverflowMenuItem
                              itemText="Delete"
                              hasDivider
                              isDelete
                              onClick={() => {
                                const item = items.find(i => i.id === row.id)
                                setEditingItem(item || null)
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
              totalItems={items.length}
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
          setEditingItem(null)
        }}
        modalHeading={editingItem?.id ? "Edit Item" : "Add New Item"}
        primaryButtonText="Save"
        onRequestSubmit={handleSaveItem}
      >
        <Form onSubmit={handleSaveItem} className='flex flex-col gap-4'>
          <TextInput
            id="item-name"
            labelText="Item Name"
            value={editingItem?.name || ''}
            onChange={(e) => setEditingItem(prev => prev ? { ...prev, name: e.target.value } : null)}
            required
          />
          <TextArea
            id="item-description"
            labelText="Description"
            value={editingItem?.description || ''}
            onChange={(e) => setEditingItem(prev => prev ? { ...prev, description: e.target.value } : null)}
          />
          <NumberInput
            id="item-price"
            label="Price"
            value={editingItem?.price || 0}
            onChange={(e) => setEditingItem(prev => prev ? { ...prev, price: Number((e.target as HTMLInputElement).value) } : null)}
            step={0.01}
            min={0}
          />
          <Select
            id="item-category"
            labelText="Category"
            value={editingItem?.item_category_id || ''}
            onChange={(e) => setEditingItem(prev => prev ? { ...prev, item_category_id: e.target.value } : null)}
            required
          >
            <SelectItem disabled hidden value="" text="Choose a category" />
            {categories.map((category) => (
              <SelectItem key={category.id} value={category.id} text={category.name || ''} />
            ))}
          </Select>
          <MultiSelect
            id="item-taxes"
            titleText="Taxes"
            label="Select taxes"
            items={taxes.map(tax => ({ id: tax.id, label: `${tax.name} (${tax.rate}%)` }))}
            selectedItems={
              editingItem?.tax_ids
                ? editingItem.tax_ids.split(',').map(id => ({
                  id,
                  label: taxes.find(tax => tax.id === id)?.name || ''
                }))
                : []
            }
            onChange={(e) => {
              const selectedTaxIds = e.selectedItems?.map(item => item.id).join(',') || ''
              setEditingItem(prev => prev ? { ...prev, tax_ids: selectedTaxIds } : null)
            }}
          />
        </Form>
      </Modal>

      <Modal
        open={isDeleteModalOpen}
        onRequestClose={() => {
          setIsDeleteModalOpen(false)
          setEditingItem(null)
        }}
        modalHeading="Delete Item"
        primaryButtonText="Delete"
        secondaryButtonText="Cancel"
        danger
        onRequestSubmit={handleDeleteItem}
      >
        <p>Are you sure you want to delete the item &quot;{editingItem?.name}&quot;? This action cannot be undone.</p>
      </Modal>
    </Content>
  )
}

export default BaseMenu
