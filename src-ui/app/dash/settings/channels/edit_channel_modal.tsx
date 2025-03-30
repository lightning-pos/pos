'use client'
import React from 'react'
import { Modal, Form, TextInput, TextArea, Toggle } from '@carbon/react'
import { Channel } from '@/lib/graphql/graphql'

interface EditChannelModalProps {
    isOpen: boolean
    channel: Channel
    onClose: () => void
    setChannel: React.Dispatch<React.SetStateAction<Channel | null>>
    onSave: () => void
}

const EditChannelModal: React.FC<EditChannelModalProps> = ({
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
            modalHeading="Edit Channel"
            primaryButtonText="Save"
            secondaryButtonText="Cancel"
            onRequestSubmit={handleSubmit}
            onRequestClose={onClose}
        >
            <Form onSubmit={handleSubmit}>
                <div className="space-y-6">
                    <TextInput
                        id="name"
                        labelText="Channel Name"
                        value={channel.name}
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
                        toggled={channel.isActive}
                        onToggle={() => setChannel({ ...channel, isActive: !channel.isActive })}
                    />
                </div>
            </Form>
        </Modal>
    )
}

export default EditChannelModal
