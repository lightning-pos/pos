'use client'
import React from 'react'
import { Modal, Form, TextInput, TextArea, Toggle } from '@carbon/react'
import { Channel } from '@/lib/graphql/graphql'
interface AddChannelModalProps {
    isOpen: boolean
    channel: Partial<Channel>
    onClose: () => void
    setChannel: React.Dispatch<React.SetStateAction<Partial<Channel>>>
    onSave: () => void
}

const AddChannelModal: React.FC<AddChannelModalProps> = ({
    isOpen,
    channel,
    onClose,
    setChannel,
    onSave
}) => {
    const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault()
        onSave()
    }

    return (
        <Modal
            open={isOpen}
            modalHeading="Add Channel"
            primaryButtonText="Create"
            secondaryButtonText="Cancel"
            onRequestSubmit={handleSubmit}
            onRequestClose={onClose}
        >
            <Form onSubmit={handleSubmit}>
                <div className="space-y-6">
                    <TextInput
                        id="name"
                        labelText="Channel Name"
                        value={channel.name || ''}
                        onChange={(e) => setChannel({ ...channel, name: e.target.value })}
                        required
                    />

                    <TextArea
                        id="description"
                        labelText="Description"
                        value={channel.description || ''}
                        onChange={(e) => setChannel({ ...channel, description: e.target.value || null })}
                    />

                    <Toggle
                        id="is_active"
                        labelText="Status"
                        labelA="Inactive"
                        labelB="Active"
                        toggled={channel.isActive ?? true}
                        onToggle={() => setChannel({ ...channel, isActive: !(channel.isActive ?? true) })}
                    />
                </div>
            </Form>
        </Modal>
    )
}

export default AddChannelModal
