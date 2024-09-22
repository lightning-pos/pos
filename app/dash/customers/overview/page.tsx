'use client'
import React from 'react'
import { Content } from '@carbon/react'
import { Customer } from '@/lib/powersync/app_schema'
import SaveCustomerModal from './save_customer_modal'
import DataTable from '@/components/ui/DataTable'
import DeleteCustomerModal from './delete_customer_modal'
import { CustomersProvider, useCustomers } from './customers_context'

const CustomersContent = () => {
  const {
    customers,
    loading,
    currentPage,
    pageSize,
    setCurrentPage,
    setPageSize,
    setEditingCustomer,
    setIsModalOpen,
    setIsDeleteModalOpen
  } = useCustomers()

  const headers = [
    { key: 'name', header: 'Name' },
    { key: 'email', header: 'Email' },
    { key: 'phoneNumber', header: 'Phone Number' },
    { key: 'countryCode', header: 'Country Code' },
  ]

  const handleAddCustomer = () => {
    setEditingCustomer({})
    setIsModalOpen(true)
  }

  const handleEditCustomer = (customer: Customer) => {
    setEditingCustomer(customer as any)
    setIsModalOpen(true)
  }

  const handleDeleteCustomerClick = (customer: Customer) => {
    setEditingCustomer(customer as any)
    setIsDeleteModalOpen(true)
  }

  return (
    <Content className='min-h-[calc(100dvh-3rem)] p-0 flex flex-col'>
      <div className="p-4 flex-grow flex flex-col" style={{ height: 'calc(100vh - 12rem)' }}>
        <DataTable<Customer>
          title="Customers"
          description="Manage your customers here. You can add, edit, or delete customers as needed."
          headers={headers}
          tableRows={customers as any}
          loading={loading}
          totalItems={customers.length}
          currentPage={currentPage}
          pageSize={pageSize}
          pageSizes={[10, 20, 30, 40, 50]}
          onPageChange={(page, pageSize) => {
            setCurrentPage(page)
            setPageSize(pageSize)
          }}
          onAddClick={handleAddCustomer}
          onEditClick={handleEditCustomer}
          onDeleteClick={handleDeleteCustomerClick}
        />
      </div>
      <SaveCustomerModal />
      <DeleteCustomerModal />
    </Content>
  )
}

const CustomersOverview = () => {
  return (
    <CustomersProvider>
      <CustomersContent />
    </CustomersProvider>
  )
}

export default CustomersOverview
