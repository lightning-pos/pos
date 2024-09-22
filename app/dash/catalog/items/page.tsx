'use client'
import React from 'react'
import { Content } from '@carbon/react'
import { ItemsProvider, useItems } from './items_context'
import DataTable from '@/components/ui/DataTable'
import SaveItemModal from './save_item_modal'
import DeleteItemModal from './delete_item_modal'
import { Item } from '@/lib/pglite/schema'

interface ItemWithTaxIds extends Item {
  taxIds: string;
}

const ItemsContent = () => {
  const {
    itemsList,
    categories,
    taxesList,
    loading,
    currentPage,
    pageSize,
    setCurrentPage,
    setPageSize,
    setEditingItem,
    setIsModalOpen,
    setIsDeleteModalOpen
  } = useItems()

  const headers = [
    { key: 'name', header: 'Name' },
    { key: 'description', header: 'Description' },
    { key: 'price', header: 'Price' },
    { key: 'category', header: 'Category' },
    { key: 'taxes', header: 'Taxes' },
  ]

  const rows = itemsList.map(item => ({
    ...item,
    category: categories.find(cat => cat.id === item.categoryId)?.name || 'Unknown',
    taxes: item.taxIds ? taxesList.filter(tax => item.taxIds?.split(',').includes(tax.id)).map(tax => tax.name).join(', ') : ''
  }))

  const handleAddItem = () => {
    setEditingItem({ name: '', description: '', price: 0, categoryId: '', taxIds: '' })
    setIsModalOpen(true)
  }

  const handleEditItem = (item: ItemWithTaxIds) => {
    setEditingItem(item)
    setIsModalOpen(true)
  }

  const handleDeleteItem = (item: ItemWithTaxIds) => {
    setEditingItem(item)
    setIsDeleteModalOpen(true)
  }

  return (
    <Content className='min-h-[calc(100dvh-3rem)] p-0 flex flex-col'>
      <div className="p-4 flex-grow flex flex-col" style={{ height: 'calc(100vh - 12rem)' }}>
        <DataTable<ItemWithTaxIds>
          title="Menu Items"
          description="Manage your menu items here. You can add, edit, or delete items as needed."
          headers={headers}
          rows={rows}
          loading={loading}
          totalItems={itemsList.length}
          currentPage={currentPage}
          pageSize={pageSize}
          pageSizes={[10, 20, 30, 40, 50]}
          onPageChange={(page, pageSize) => {
            setCurrentPage(page)
            setPageSize(pageSize)
          }}
          onAddClick={handleAddItem}
          onEditClick={handleEditItem}
          onDeleteClick={handleDeleteItem}
        />
      </div>
      <SaveItemModal />
      <DeleteItemModal />
    </Content>
  )
}

const Items = () => {
  return (
    <ItemsProvider>
      <ItemsContent />
    </ItemsProvider>
  )
}

export default Items
