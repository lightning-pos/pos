import React from 'react'
import { Modal, TextInput, Form, TextArea, NumberInput, Select, SelectItem, MultiSelect } from '@carbon/react'
import { useItems } from './items_context'

const SaveItemModal = () => {
  const {
    editingItem,
    isModalOpen,
    handleSaveItem,
    setEditingItem,
    setIsModalOpen,
    categories,
    taxesList
  } = useItems()

  return (
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
          value={(editingItem?.price || 0) / 100}
          onChange={(e, { value }) => setEditingItem(prev => prev ? { ...prev, price: Number(value) * 100 } : null)}
          step={1}
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
            <SelectItem key={category.id} value={category.id} text={category.name} />
          ))}
        </Select>
        <MultiSelect
          id="item-taxes"
          titleText="Taxes"
          label="Select taxes"
          items={taxesList.map(tax => ({ id: tax.id, label: `${tax.name} (${tax.rate / 100}%)` }))}
          selectedItems={
            editingItem?.taxIds
              ? Array.from(new Set(editingItem.taxIds.split(','))).map(id => ({
                id,
                label: taxesList.find(tax => tax.id === id)?.name || ''
              }))
              : []
          }
          onChange={(e) => {
            const selectedTaxIds = Array.from(new Set(e.selectedItems?.map(item => (item as { id: string }).id) || [])).join(',')
            setEditingItem(prev => prev ? { ...prev, taxIds: selectedTaxIds } : null)
          }}
        />
      </Form>
    </Modal>
  )
}

export default SaveItemModal
