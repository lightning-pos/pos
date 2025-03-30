'use client'
import React from 'react'
import { Modal } from '@carbon/react'

interface DeleteChannelModalProps {
    isOpen: boolean
    channelId: string
    channelName: string
    onClose: () => void
    onDelete: () => void
}

const DeleteChannelModal: React.FC<DeleteChannelModalProps> = ({
    isOpen,
    channelId,
    channelName,
    onClose,
    onDelete
}) => {
    return (
        <Modal
            open={isOpen}
            modalHeading="Delete Channel"
            primaryButtonText="Delete"
            secondaryButtonText="Cancel"
            danger
            onRequestSubmit={onDelete}
            onRequestClose={onClose}
        >
            <p>
                Are you sure you want to delete the channel "{channelName}"? This action cannot be undone.
            </p>
        </Modal>
    )
}

export default DeleteChannelModal
