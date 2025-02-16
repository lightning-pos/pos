import React, { useState } from 'react'
import { Modal, TextInput, Form, TextArea, ModalProps } from '@carbon/react'
import { ItemGroupNew } from '@/lib/graphql/graphql'

interface AddCategoryModalProps extends Omit<ModalProps, 'onSubmit'> {
    onSave: (category: ItemGroupNew) => Promise<void>
}

const AddCategoryModal: React.FC<AddCategoryModalProps> = ({
    open,
    onRequestClose,
    onSave,
}) => {
    const [newCategory, setNewCategory] = useState<ItemGroupNew>({
        name: '',
        description: '',
    })

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault()
        await onSave(newCategory)
        setNewCategory({ name: '', description: '' })
    }

    const handleClose = (e: React.SyntheticEvent<HTMLElement>) => {
        onRequestClose?.(e)
        setNewCategory({ name: '', description: '' })
    }

    return (
        <Modal
            open={open}
            onRequestClose={handleClose}
            modalHeading="Add New Category"
            primaryButtonText="Save"
            onRequestSubmit={handleSubmit}
        >
            <Form onSubmit={handleSubmit} className='flex flex-col gap-4'>
                <TextInput
                    id="category-name"
                    labelText="Category Name"
                    value={newCategory.name}
                    onChange={(e) => setNewCategory(prev => ({ ...prev, name: e.target.value }))}
                    required
                />
                <TextArea
                    id="category-description"
                    labelText="Description"
                    value={newCategory.description || ''}
                    onChange={(e) => setNewCategory(prev => ({ ...prev, description: e.target.value }))}
                />
            </Form>
        </Modal>
    )
}

export default AddCategoryModal
