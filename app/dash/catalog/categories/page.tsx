'use client'
import React, { useState, useEffect, useCallback } from 'react'
import { Add } from '@carbon/icons-react'
import { Content, DataTable, Table, TableHead, TableRow, TableHeader, TableBody, TableCell, Pagination, DataTableSkeleton, Button, Modal, TextInput, Form, TableToolbar, TableToolbarContent, TableContainer, TextArea, OverflowMenu, OverflowMenuItem } from '@carbon/react'
import { drizzleDb } from '@/components/providers/system_provider'
import { Category as CategorySchema } from '@/lib/powersync/app_schema'
import { uid } from 'uid'
import { eq } from 'drizzle-orm'
import { itemCategories } from '@/lib/pglite/schema'

const Categories = () => {
  const [currentPage, setCurrentPage] = useState(1)
  const [pageSize, setPageSize] = useState(10)
  const [categories, setCategories] = useState<CategorySchema[]>([])
  const [loading, setLoading] = useState(true)
  const [isModalOpen, setIsModalOpen] = useState(false)
  const [editingCategory, setEditingCategory] = useState<CategorySchema | null>(null)
  const [isDeleteModalOpen, setIsDeleteModalOpen] = useState(false)

  useEffect(() => {
    fetchCategories()
  }, [])

  const headers = [
    { key: 'name', header: 'Name' },
    { key: 'description', header: 'Description' },
    { key: 'state', header: 'State' }
  ]

  const startIndex = (currentPage - 1) * pageSize
  const endIndex = startIndex + pageSize
  const paginatedCategories = categories.slice(startIndex, endIndex)

  const fetchCategories = useCallback(async () => {
    setLoading(true)
    try {
      const result = await drizzleDb.select().from(itemCategories)
      setCategories(result as unknown as CategorySchema[]) // Cast to 'unknown' first
    } catch (error) {
      console.error('Error fetching categories:', error)
    } finally {
      setLoading(false)
    }
  }, [])

  const handleSaveCategory = useCallback(async (e: React.FormEvent) => {
    e.preventDefault()
    if (!editingCategory) return
    try {
      if (editingCategory.id) {
        await drizzleDb.update(itemCategories)
          .set({
            name: editingCategory.name || '',
            description: editingCategory.description || null
          })
          .where(eq(itemCategories.id, editingCategory.id))
      } else {
        await drizzleDb.insert(itemCategories).values({
          id: uid(),
          name: editingCategory.name || '',
          description: editingCategory.description || null,
          state: editingCategory.status || null
        })
      }
      setIsModalOpen(false)
      setEditingCategory(null)
      fetchCategories()
    } catch (error) {
      console.error('Error saving category:', error)
    }
  }, [editingCategory, fetchCategories])

  const handleDeleteCategory = useCallback(async () => {
    if (!editingCategory?.id) return
    try {
      await drizzleDb.delete(itemCategories)
        .where(eq(itemCategories.id, editingCategory.id))
      setIsDeleteModalOpen(false)
      setEditingCategory(null)
      fetchCategories()
    } catch (error) {
      console.error('Error deleting category:', error)
    }
  }, [editingCategory, fetchCategories])

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
                <Button
                  renderIcon={Add}
                  onClick={() => {
                    setEditingCategory({ id: '', name: '', description: '', status: 'active' } as CategorySchema)
                    setIsModalOpen(true)
                  }}
                >
                  Add Category
                </Button>
              </TableToolbarContent>
            </TableToolbar>
            <DataTable rows={paginatedCategories} headers={headers}>
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
                                const category = categories.find(c => c.id === row.id)
                                setEditingCategory(category || null)
                                setIsModalOpen(true)
                              }}
                            />
                            <OverflowMenuItem
                              itemText="Delete"
                              hasDivider
                              isDelete
                              onClick={() => {
                                const category = categories.find(c => c.id === row.id)
                                setEditingCategory(category || null)
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
        onRequestClose={() => {
          setIsModalOpen(false)
          setEditingCategory(null)
        }}
        modalHeading={editingCategory?.id ? "Edit Category" : "Add New Category"}
        primaryButtonText="Save"
        onRequestSubmit={handleSaveCategory}
      >
        <Form onSubmit={handleSaveCategory} className='flex flex-col gap-4'>
          <TextInput
            id="category-name"
            labelText="Category Name"
            value={editingCategory?.name || ''}
            onChange={(e) => setEditingCategory(prev => prev ? { ...prev, name: e.target.value } : null)}
            required
          />
          <TextArea
            id="category-description"
            labelText="Description"
            value={editingCategory?.description || ''}
            onChange={(e) => setEditingCategory(prev => prev ? { ...prev, description: e.target.value } : null)}
          />
        </Form>
      </Modal>

      <Modal
        open={isDeleteModalOpen}
        onRequestClose={() => {
          setIsDeleteModalOpen(false)
          setEditingCategory(null)
        }}
        modalHeading="Delete Category"
        primaryButtonText="Delete"
        secondaryButtonText="Cancel"
        danger
        onRequestSubmit={handleDeleteCategory}
      >
        <p>Are you sure you want to delete the category &quot;{editingCategory?.name}&quot;? This action cannot be undone.</p>
      </Modal>
    </Content>
  )
}

export default Categories
