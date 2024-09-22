import React from 'react'
import { Modal } from '@carbon/react'
import { useCategories } from './categories_context'

const DeleteCategoryModal = () => {
  const {
    editingCategory,
    isDeleteModalOpen,
    handleDeleteCategory,
    setEditingCategory,
    setIsDeleteModalOpen
  } = useCategories()

  return (
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
  )
}

export default DeleteCategoryModal
