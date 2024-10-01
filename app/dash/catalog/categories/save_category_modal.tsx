import React from 'react'
import { Modal, TextInput, Form, TextArea } from '@carbon/react'
import { NewItemCategory } from '@/lib/db/sqlite/schema'

interface SaveCategoryModalProps {
  isOpen: boolean
  onClose: () => void
  onSave: (e: React.FormEvent) => Promise<void>
  category: NewItemCategory | null
  setCategory: React.Dispatch<React.SetStateAction<NewItemCategory | null>>
}

const SaveCategoryModal: React.FC<SaveCategoryModalProps> = ({
  isOpen,
  onClose,
  onSave,
  category,
  setCategory
}) => {
  return (
    <Modal
      open={isOpen}
      onRequestClose={() => {
        onClose()
        setCategory(null)
      }}
      modalHeading={category?.id ? "Edit Category" : "Add New Category"}
      primaryButtonText="Save"
      onRequestSubmit={onSave}
    >
      <Form onSubmit={onSave} className='flex flex-col gap-4'>
        <TextInput
          id="category-name"
          labelText="Category Name"
          value={category?.name || ''}
          onChange={(e) => setCategory(prev => prev ? { ...prev, name: e.target.value } : null)}
          required
        />
        <TextArea
          id="category-description"
          labelText="Description"
          value={category?.description || ''}
          onChange={(e) => setCategory(prev => prev ? { ...prev, description: e.target.value } : null)}
        />
      </Form>
    </Modal>
  )
}

export default SaveCategoryModal
