import React from 'react'
import { Modal } from '@carbon/react'
import { useItems } from './items_context'

const DeleteItemModal = () => {
    const {
        editingItem,
        isDeleteModalOpen,
        handleDeleteItem,
        setEditingItem,
        setIsDeleteModalOpen
    } = useItems()

    return (
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
    )
}

export default DeleteItemModal
