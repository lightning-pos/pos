import React, { useState, useEffect } from 'react'
import { Modal, TextInput, Form, TextArea, ModalProps } from '@carbon/react'
import { ItemGroup, ItemGroupUpdate } from '@/lib/graphql/graphql'

interface EditCategoryModalProps extends Omit<ModalProps, 'onSubmit'> {
    category: ItemGroup
    onSave: (category: ItemGroupUpdate) => Promise<void>
}

const EditCategoryModal: React.FC<EditCategoryModalProps> = ({
    open,
    onRequestClose,
    onSave,
    category,
}) => {
    const [localCategory, setLocalCategory] = useState<ItemGroupUpdate>({
        id: category?.id,
        name: category?.name,
        description: category?.description,
    })

    useEffect(() => {
        if (category && open) {
            setLocalCategory({
                id: category.id,
                name: category.name,
                description: category.description,
            })
        }
    }, [category, open])

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault()
        await onSave(localCategory)
    }

    const handleClose = (e: React.SyntheticEvent<HTMLElement>) => {
        onRequestClose?.(e)
    }

    return (
        <Modal
            open={open}
            onRequestClose={handleClose}
            modalHeading="Edit Category"
            primaryButtonText="Save"
            onRequestSubmit={handleSubmit}
        >
            <Form onSubmit={handleSubmit} className='flex flex-col gap-4'>
                <TextInput
                    id="category-name"
                    labelText="Category Name"
                    value={localCategory.name || ''}
                    onChange={(e) => setLocalCategory(prev => ({ ...prev, name: e.target.value }))}
                    required
                />
                <TextArea
                    id="category-description"
                    labelText="Description"
                    value={localCategory.description || ''}
                    onChange={(e) => setLocalCategory(prev => ({ ...prev, description: e.target.value }))}
                />
            </Form>
        </Modal>
    )
}

export default EditCategoryModal
