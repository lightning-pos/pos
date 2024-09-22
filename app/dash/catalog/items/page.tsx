'use client'
import React, { useState, useEffect, useCallback } from 'react'
import { Add } from '@carbon/icons-react'
import { Content, DataTable, Table, TableHead, TableRow, TableHeader, TableBody, TableCell, Pagination, DataTableSkeleton, Button, Modal, TextInput, Form, TableToolbar, TableToolbarContent, TableContainer, TextArea, OverflowMenu, OverflowMenuItem, Select, SelectItem, NumberInput, MultiSelect } from '@carbon/react'
import { drizzleDb } from '@/components/providers/system_provider'
import { Item, ItemCategory, Tax, ItemTax, items, itemCategories, taxes, itemTaxes } from '@/lib/pglite/schema'
import { eq, inArray } from 'drizzle-orm'
import { uid } from 'uid'

// Define a new interface that extends Item and includes taxIds
interface ItemWithTaxIds extends Item {
  taxIds: string;
}

const BaseMenu = () => {
  const [currentPage, setCurrentPage] = useState(1)
  const [pageSize, setPageSize] = useState(10)
  const [itemsList, setItemsList] = useState<ItemWithTaxIds[]>([])
  const [categories, setCategories] = useState<ItemCategory[]>([])
  const [loading, setLoading] = useState(true)
  const [isModalOpen, setIsModalOpen] = useState(false)
  const [editingItem, setEditingItem] = useState<Partial<ItemWithTaxIds> | null>(null)
  const [isDeleteModalOpen, setIsDeleteModalOpen] = useState(false)
  const [taxesList, setTaxesList] = useState<Tax[]>([])

  const fetchData = useCallback(async () => {
    setLoading(true)
    try {
      const itemsResult = await drizzleDb.select().from(items)
      const itemTaxesResult = await drizzleDb.select().from(itemTaxes)

      const itemsWithTaxIds: ItemWithTaxIds[] = itemsResult.map(item => ({
        ...item,
        taxIds: itemTaxesResult
          .filter(it => it.itemId === item.id)
          .map(it => it.taxId)
          .join(',')
      }))

      setItemsList(itemsWithTaxIds)
      const categoriesResult = await drizzleDb.select().from(itemCategories)
      setCategories(categoriesResult as ItemCategory[])
      const taxesResult = await drizzleDb.select().from(taxes)
      setTaxesList(taxesResult)
    } catch (error) {
      console.error('Error fetching data:', error)
    } finally {
      setLoading(false)
    }
  }, [])

  useEffect(() => {
    void fetchData()
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
  const paginatedItems = itemsList.slice(startIndex, endIndex).map(item => ({
    ...item,
    category: categories.find(cat => cat.id === item.categoryId)?.name || 'Unknown',
    taxes: item.taxIds ? taxesList.filter(tax => item.taxIds?.split(',').includes(tax.id)).map(tax => tax.name).join(', ') : ''
  }))

  const handleSaveItem = async (e: React.FormEvent) => {
    e.preventDefault()
    if (!editingItem) return
    try {
      const itemData: Partial<Item> = {
        name: editingItem.name,
        description: editingItem.description,
        price: Number(editingItem.price) * 100, // Convert to cents
        categoryId: editingItem.categoryId,
      }

      if (editingItem.id) {
        await drizzleDb.update(items)
          .set(itemData)
          .where(eq(items.id, editingItem.id))
          .execute()

        // Delete existing item_taxes
        await drizzleDb.delete(itemTaxes)
          .where(eq(itemTaxes.itemId, editingItem.id))
          .execute()
      } else {
        const newItemId = uid()
        await drizzleDb.insert(items).values({ ...itemData, id: newItemId } as Item).execute()
        itemData.id = newItemId
      }

      // Insert new item_taxes
      if (editingItem.taxIds) {
        const taxIdsArray = editingItem.taxIds.split(',')
        for (const taxId of taxIdsArray) {
          await drizzleDb.insert(itemTaxes).values({
            id: uid(),
            itemId: itemData.id!,
            taxId: taxId,
          } as ItemTax).execute()
        }
      }

      setIsModalOpen(false)
      setEditingItem(null)
      void fetchData()
    } catch (error) {
      console.error('Error saving item:', error)
    }
  }

  const handleDeleteItem = async () => {
    if (!editingItem?.id) return
    try {
      await drizzleDb.delete(itemTaxes)
        .where(eq(itemTaxes.itemId, editingItem.id))
        .execute()
      await drizzleDb.delete(items)
        .where(eq(items.id, editingItem.id))
        .execute()
      setIsDeleteModalOpen(false)
      setEditingItem(null)
      void fetchData()
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
                    setEditingItem({ name: '', description: '', price: 0, categoryId: '', taxIds: '' })
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
                          <OverflowMenu aria-label="Actions">
                            <OverflowMenuItem
                              itemText="Edit"
                              onClick={() => {
                                const item = itemsList.find(i => i.id === row.id)
                                setEditingItem(item || null)
                                setIsModalOpen(true)
                              }}
                            />
                            <OverflowMenuItem
                              itemText="Delete"
                              hasDivider
                              isDelete
                              onClick={() => {
                                const item = itemsList.find(i => i.id === row.id)
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
              totalItems={itemsList.length}
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
            value={(editingItem?.price || 0) / 100} // Convert from cents to dollars
            onChange={(e) => setEditingItem(prev => prev ? { ...prev, price: Number((e.target as HTMLInputElement).value) * 100 } : null)}
            step={0.01}
            min={0}
          />
          <Select
            id="item-category"
            labelText="Category"
            value={editingItem?.categoryId || ''}
            onChange={(e) => setEditingItem(prev => prev ? { ...prev, categoryId: e.target.value } : null)}
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
            items={taxesList.map(tax => ({ id: tax.id, label: `${tax.name} (${tax.rate / 100}%)` }))}
            selectedItems={
              editingItem?.taxIds
                ? editingItem.taxIds.split(',').map(id => ({
                  id,
                  label: taxesList.find(tax => tax.id === id)?.name || ''
                }))
                : []
            }
            onChange={(e) => {
              const selectedTaxIds = e.selectedItems?.map(item => (item as { id: string }).id).join(',') || ''
              setEditingItem(prev => prev ? { ...prev, taxIds: selectedTaxIds } : null)
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
