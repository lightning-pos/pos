import React, { useState, useEffect } from 'react'
import { Modal, TextInput, Form, TextArea, ModalProps } from '@carbon/react'
import { PurchaseCategory } from '@/lib/graphql/graphql'

interface PurchaseCategoryUpdate {
    name?: string | null
    description?: string | null
}

interface EditPurchaseCategoryModalProps extends Omit<ModalProps, 'onSubmit'> {
    category: PurchaseCategory
    onSave: (update: PurchaseCategoryUpdate) => Promise<void>
}

const EditPurchaseCategoryModal: React.FC<EditPurchaseCategoryModalProps> = ({
    open,
    onRequestClose,
    onSave,
    category,
}) => {
    const [updatedCategory, setUpdatedCategory] = useState<PurchaseCategoryUpdate>({
        name: category.name || '',
        description: category.description || null,
    })

    useEffect(() => {
        if (category) {
            setUpdatedCategory({
                name: category.name || '',
                description: category.description || null,
            })
        }
    }, [category])

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault()
        await onSave(updatedCategory)
    }

    return (
        <Modal
            open={open}
            onRequestClose={onRequestClose}
            modalHeading="Edit Purchase Category"
            primaryButtonText="Save"
            onRequestSubmit={handleSubmit}
        >
            <Form onSubmit={handleSubmit} className='flex flex-col gap-4'>
                <TextInput
                    id="edit-category-name"
                    labelText="Category Name"
                    value={updatedCategory.name || ''}
                    onChange={(e) => setUpdatedCategory(prev => ({ ...prev, name: e.target.value }))}
                    required
                />
                <TextArea
                    id="edit-category-description"
                    labelText="Description"
                    value={updatedCategory.description || ''}
                    onChange={(e) => setUpdatedCategory(prev => ({ ...prev, description: e.target.value }))}
                />
            </Form>
        </Modal>
    )
}

export default EditPurchaseCategoryModal
