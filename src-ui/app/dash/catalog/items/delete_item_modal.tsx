import React from 'react'
import { Modal, ModalProps } from '@carbon/react'
import { Item } from '@/lib/graphql/graphql'

interface DeleteItemModalProps extends Omit<ModalProps, 'onSubmit'> {
    onSave: () => Promise<void>
    item: Item
}

const DeleteItemModal: React.FC<DeleteItemModalProps> = ({
    open,
    onRequestClose,
    onSave,
    item,
}) => {
    const handleClose = (e: React.SyntheticEvent<HTMLElement>) => {
        onRequestClose?.(e)
    }

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault()
        await onSave()
    }

    return (
        <Modal
            open={open}
            onRequestClose={handleClose}
            modalHeading="Delete Item"
            primaryButtonText="Delete"
            danger
            onRequestSubmit={handleSubmit}
        >
            <p>Are you sure you want to delete {item.name}? This action cannot be undone.</p>
        </Modal>
    )
}

export default DeleteItemModal
