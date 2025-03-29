import React, { useState } from 'react'
import { Modal, TextInput, Form, TextArea, ModalProps } from '@carbon/react'

interface AddPurchaseCategoryModalProps extends Omit<ModalProps, 'onSubmit'> {
    onSave: (name: string, description: string | null) => Promise<void>
}

const AddPurchaseCategoryModal: React.FC<AddPurchaseCategoryModalProps> = ({
    open,
    onRequestClose,
    onSave,
}) => {
    const [name, setName] = useState('')
    const [description, setDescription] = useState<string | null>(null)

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault()
        await onSave(name, description)
        setName('')
        setDescription(null)
    }

    const handleClose = (e: React.SyntheticEvent<HTMLElement>) => {
        onRequestClose?.(e)
        setName('')
        setDescription(null)
    }

    return (
        <Modal
            open={open}
            onRequestClose={handleClose}
            modalHeading="Add New Purchase Category"
            primaryButtonText="Save"
            onRequestSubmit={handleSubmit}
        >
            <Form onSubmit={handleSubmit} className='flex flex-col gap-4'>
                <TextInput
                    id="category-name"
                    labelText="Category Name"
                    value={name}
                    onChange={(e) => setName(e.target.value)}
                    required
                />
                <TextArea
                    id="category-description"
                    labelText="Description"
                    value={description || ''}
                    onChange={(e) => setDescription(e.target.value)}
                />
            </Form>
        </Modal>
    )
}

export default AddPurchaseCategoryModal
