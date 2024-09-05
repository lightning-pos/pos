'use client'
import React, { useState, useEffect } from 'react'
import { Add, Delete, Edit } from '@carbon/icons-react'
import { Content, DataTable, Table, TableHead, TableRow, TableHeader, TableBody, TableCell, Pagination, DataTableSkeleton, Button, Modal, TextInput, Form, TableToolbar, TableToolbarContent, TableContainer, TextArea, OverflowMenu, OverflowMenuItem } from '@carbon/react'
import { db } from '@/components/providers/system_provider'
import { Category as CategorySchema } from '@/lib/powersync/app_schema'
import { uid } from 'uid'

const Categories = () => {
  const [currentPage, setCurrentPage] = useState(1)
  const [pageSize, setPageSize] = useState(10)
  const [categories, setCategories] = useState<CategorySchema[]>([])
  const [loading, setLoading] = useState(true)
  const [isModalOpen, setIsModalOpen] = useState(false)
  const [newCategory, setNewCategory] = useState({ name: '', description: '' })

  useEffect(() => {
    const fetchCategories = async () => {
      setLoading(true)
      try {
        const result: CategorySchema[] = await db.selectFrom('item_categories').selectAll().execute()
        setCategories(result)
      } catch (error) {
        console.error('Error fetching categories:', error)
      } finally {
        setLoading(false)
      }
    }

    fetchCategories()
  }, [])

  const headers = [
    { key: 'name', header: 'Name' },
    { key: 'description', header: 'Description' },
    { key: 'status', header: 'Status' }
  ]

  const startIndex = (currentPage - 1) * pageSize
  const endIndex = startIndex + pageSize
  const paginatedCategories = categories.slice(startIndex, endIndex)

  const handleAddCategory = async (e: React.FormEvent) => {
    e.preventDefault()
    try {
      await db.insertInto('item_categories').values({ id: uid(), ...newCategory }).execute()
      setIsModalOpen(false)
      setNewCategory({ name: '', description: '' })
      // Refetch categories
      const result: CategorySchema[] = await db.selectFrom('item_categories').selectAll().execute()
      setCategories(result)
    } catch (error) {
      console.error('Error adding category:', error)
    }
  }

  return (
    <Content className='min-h-[calc(100dvh-3rem)] p-0 flex flex-col'>
      <div className="p-4 flex-grow flex flex-col" style={{ height: 'calc(100vh - 12rem)' }}>
        {loading ? (
          <DataTableSkeleton headers={headers} rowCount={pageSize} />
        ) : (
          <TableContainer
            title="Category"
            description="Manage your categories here. You can add, edit, or delete categories as needed."
          >
            <TableToolbar>
              <TableToolbarContent>
                <Button renderIcon={Add} onClick={() => setIsModalOpen(true)}>Add Category</Button>
              </TableToolbarContent>
            </TableToolbar>
            <DataTable rows={paginatedCategories} headers={headers}>
              {({ rows, headers, getTableProps }) => {
                return (
                  <Table {...getTableProps()}>
                    <TableHead>
                      <TableRow>
                        {headers.map((header) => (
                          <TableHeader key={header.key}>
                            {header.header}
                          </TableHeader>
                        ))}
                        <TableHeader key="actions" style={{ width: '8rem' }}>
                          Actions
                        </TableHeader>
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
                );
              }}
            </DataTable>
            <Pagination
              totalItems={categories.length}
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
        modalHeading="Add New Category"
        primaryButtonText="Add"
        onRequestSubmit={handleAddCategory}
      >
        <Form onSubmit={handleAddCategory} className='flex flex-col gap-4'>
          <TextInput
            id="category-name"
            labelText="Category Name"
            value={newCategory.name}
            onChange={(e) => setNewCategory({ ...newCategory, name: e.target.value })}
            required
          />
          <TextArea
            id="category-description"
            labelText="Description"
            value={newCategory.description}
            onChange={(e) => setNewCategory({ ...newCategory, description: e.target.value })}
          />
        </Form>
      </Modal>
    </Content>
  )
}

export default Categories
