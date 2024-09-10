'use client'
import React, { useState, useEffect } from 'react'
import { Content, DataTable, TableContainer, Table, TableHead, TableRow, TableHeader, TableBody, TableCell, TableToolbar, TableToolbarSearch, TableToolbarContent, Button, OverflowMenu, OverflowMenuItem, Pagination } from '@carbon/react'
import { Add } from '@carbon/icons-react'
import { db } from '@/components/providers/system_provider'
import { Customer } from '@/lib/powersync/app_schema'
import { uid } from 'uid'
import SaveCustomerModal from './save_customer_modal'

const CustomersOverview = () => {
  const [customers, setCustomers] = useState<Customer[]>([])
  const [filteredCustomers, setFilteredCustomers] = useState<Customer[]>([])
  const [isModalOpen, setIsModalOpen] = useState(false)
  const [editingCustomer, setEditingCustomer] = useState<Partial<Customer> | null>(null)
  const [currentPage, setCurrentPage] = useState(1)
  const [pageSize, setPageSize] = useState(10)

  useEffect(() => {
    const fetchCustomers = async () => {
      const result = await db.selectFrom('customers').selectAll().execute()
      setCustomers(result)
      setFilteredCustomers(result)
    }
    fetchCustomers()
  }, [])

  const headers = [
    { key: 'name', header: 'Name' },
    { key: 'email', header: 'Email' },
    { key: 'phone_number', header: 'Phone Number' },
    { key: 'country_code', header: 'Country Code' },
    { key: 'actions', header: 'Actions' },
  ]

  const handleSearch = (searchTerm: string) => {
    const lowercasedTerm = searchTerm.toLowerCase()
    const filtered = customers.filter(customer =>
      customer.name?.toLowerCase().includes(lowercasedTerm) ||
      customer.email?.toLowerCase().includes(lowercasedTerm) ||
      customer.phone_number?.includes(searchTerm)
    )
    setFilteredCustomers(filtered)
    setCurrentPage(1)
  }

  const handleAddOrUpdateCustomer = async () => {
    if (editingCustomer?.id) {
      // Update existing customer
      await db.updateTable('customers').set(editingCustomer).where('id', '=', editingCustomer.id).execute()
      setCustomers(customers.map(c => c.id === editingCustomer.id ? editingCustomer as Customer : c))
      setFilteredCustomers(filteredCustomers.map(c => c.id === editingCustomer.id ? editingCustomer as Customer : c))
    } else {
      // Add new customer
      const newCustomer = { ...editingCustomer, id: uid() } as Customer
      await db.insertInto('customers').values(newCustomer).execute()
      setCustomers([...customers, newCustomer])
      setFilteredCustomers([...filteredCustomers, newCustomer])
    }
    setIsModalOpen(false)
    setEditingCustomer(null)
  }

  const handleDeleteCustomer = async (id: string) => {
    await db.deleteFrom('customers').where('id', '=', id).execute()
    const updatedCustomers = customers.filter(c => c.id !== id)
    setCustomers(updatedCustomers)
    setFilteredCustomers(updatedCustomers)
  }

  const paginatedCustomers = filteredCustomers.slice((currentPage - 1) * pageSize, currentPage * pageSize)

  return (
    <Content className='min-h-[calc(100dvh-3rem)] p-4'>
      <TableContainer title="Customers Overview" description="Manage your customers here. Add, edit or delete them as needed.">
        <TableToolbar>
          <TableToolbarContent>
            <TableToolbarSearch
              onChange={(e) => handleSearch(typeof e === 'string' ? e : e.target.value)}
              placeholder="Search customers"
            />
            <Button renderIcon={Add} onClick={() => { setEditingCustomer({}); setIsModalOpen(true); }}>Add Customer</Button>
          </TableToolbarContent>
        </TableToolbar>
        <DataTable rows={paginatedCustomers} headers={headers}>
          {({ rows, headers, getTableProps, getHeaderProps, getRowProps }) => (
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
                  <TableRow {...getRowProps({ row })} key={row.id}>
                    {row.cells.map((cell) => (
                      <TableCell key={cell.id}>
                        {cell.info.header === 'actions' ? (
                          <OverflowMenu flipped>
                            <OverflowMenuItem
                              itemText="Edit"
                              onClick={() => {
                                setEditingCustomer(customers.find(c => c.id === row.id) || {});
                                setIsModalOpen(true);
                              }}
                            />
                            <OverflowMenuItem
                              itemText="Delete"
                              isDelete
                              hasDivider
                              onClick={() => handleDeleteCustomer(row.id)}
                            />
                          </OverflowMenu>
                        ) : (
                          cell.value
                        )}
                      </TableCell>
                    ))}
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          )}
        </DataTable>
        <Pagination
          totalItems={filteredCustomers.length}
          pageSize={pageSize}
          pageSizes={[10, 20, 30, 40, 50]}
          page={currentPage}
          onChange={({ page, pageSize }) => {
            setCurrentPage(page);
            setPageSize(pageSize);
          }}
        />
      </TableContainer>

      <SaveCustomerModal
        isOpen={isModalOpen}
        editingCustomer={editingCustomer}
        onClose={() => { setIsModalOpen(false); setEditingCustomer(null); }}
        onSave={handleAddOrUpdateCustomer}
        onCustomerChange={setEditingCustomer}
      />
    </Content>
  )
}

export default CustomersOverview
