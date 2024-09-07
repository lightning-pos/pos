'use client'
import React, { useState, useEffect } from 'react'
import { Add } from '@carbon/icons-react'
import { Content, DataTable, Table, TableHead, TableRow, TableHeader, TableBody, TableCell, Pagination, DataTableSkeleton, Button, Modal, TextInput, Form, TableToolbar, TableToolbarContent, TableContainer, TextArea, OverflowMenu, OverflowMenuItem, Select, SelectItem, NumberInput } from '@carbon/react'
import { db } from '@/components/providers/system_provider'
import { Item as ItemSchema, Category as CategorySchema } from '@/lib/powersync/app_schema'
import { uid } from 'uid'

const BaseMenu = () => {
  const [currentPage, setCurrentPage] = useState(1)
  const [pageSize, setPageSize] = useState(10)
  const [items, setItems] = useState<ItemSchema[]>([])
  const [categories, setCategories] = useState<CategorySchema[]>([])
  const [loading, setLoading] = useState(true)
  const [isModalOpen, setIsModalOpen] = useState(false)
  const [newItem, setNewItem] = useState({ name: '', description: '', price: 0, item_category_id: '' })

  useEffect(() => {
    const fetchData = async () => {
      setLoading(true)
      try {
        const itemsResult: ItemSchema[] = await db.selectFrom('items').selectAll().execute()
        setItems(itemsResult)
        const categoriesResult: CategorySchema[] = await db.selectFrom('item_categories').selectAll().execute()
        setCategories(categoriesResult)
      } catch (error) {
        console.error('Error fetching data:', error)
      } finally {
        setLoading(false)
      }
    }

    fetchData()
  }, [])

  const headers = [
    { key: 'name', header: 'Name' },
    { key: 'description', header: 'Description' },
    { key: 'price', header: 'Price' },
    { key: 'category', header: 'Category' },
  ]

  const startIndex = (currentPage - 1) * pageSize
  const endIndex = startIndex + pageSize
  const paginatedItems = items.slice(startIndex, endIndex).map(item => ({
    ...item,
    category: categories.find(cat => cat.id === item.item_category_id)?.name || 'Unknown'
  }))

  const handleAddItem = async (e: React.FormEvent) => {
    e.preventDefault()
    try {
      await db.insertInto('items').values({ id: uid(), ...newItem }).execute()
      setIsModalOpen(false)
      setNewItem({ name: '', description: '', price: 0, item_category_id: '' })
      // Refetch items
      const result: ItemSchema[] = await db.selectFrom('items').selectAll().execute()
      setItems(result)
    } catch (error) {
      console.error('Error adding item:', error)
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
                <Button renderIcon={Add} onClick={() => setIsModalOpen(true)}>Add Item</Button>
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
                            <OverflowMenuItem itemText="Edit" />
                            <OverflowMenuItem itemText="Delete" hasDivider isDelete />
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
        onRequestClose={() => setIsModalOpen(false)}
        modalHeading="Add New Item"
        primaryButtonText="Add"
        onRequestSubmit={handleAddItem}
      >
        <Form onSubmit={handleAddItem} className='flex flex-col gap-4'>
          <TextInput
            id="item-name"
            labelText="Item Name"
            value={newItem.name}
            onChange={(e) => setNewItem({ ...newItem, name: e.target.value })}
            required
          />
          <TextArea
            id="item-description"
            labelText="Description"
            value={newItem.description}
            onChange={(e) => setNewItem({ ...newItem, description: e.target.value })}
          />
          <NumberInput
            id="item-price"
            label="Price"
            value={newItem.price}
            onChange={(e) => setNewItem({ ...newItem, price: Number((e.target as HTMLInputElement).value) })}
            step={1}
            min={0}
          />
          <Select
            id="item-category"
            labelText="Category"
            value={newItem.item_category_id}
            onChange={(e) => setNewItem({ ...newItem, item_category_id: e.target.value })}
            required
          >
            <SelectItem disabled hidden value="" text="Choose a category" />
            {categories.map((category) => (
              <SelectItem key={category.id} value={category.id} text={category.name ? category.name : ''} />
            ))}
          </Select>
        </Form>
      </Modal>
    </Content>
  )
}

export default BaseMenu
