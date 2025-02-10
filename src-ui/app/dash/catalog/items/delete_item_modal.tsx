import React from 'react'
import { Modal, ModalProps } from '@carbon/react'
import { invoke } from '@tauri-apps/api/core'

interface DeleteItemModalProps extends ModalProps {
    itemId: string;
    itemName: string;
}

const DeleteItemModal: React.FC<DeleteItemModalProps> = ({
    open,
    onRequestClose,
    onRequestSubmit,
    itemId,
    itemName
}) => {
    const deleteItem = async (e: React.FormEvent<HTMLFormElement>) => {
        try {
            await invoke('graphql', {
                query: `#graphql
                    mutation {
                        deleteItem(id: "${itemId}")
                    }
                `
            })
            onRequestSubmit?.(e)
        } catch (error) {
            console.error('Error deleting item:', error)
            alert('Failed to delete item')
        }
    }

    return (
        <Modal
            open={open}
            modalHeading="Delete Item"
            primaryButtonText="Delete"
            onRequestClose={onRequestClose}
            onRequestSubmit={deleteItem}
            danger
        >
            <p>Are you sure you want to delete the item &quot;{itemName}&quot;? This action cannot be undone.</p>
        </Modal>
    )
}

export default DeleteItemModal
