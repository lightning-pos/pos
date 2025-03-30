import React, { useState, useEffect } from 'react'
import { Modal, TextInput, Form, TextArea, ModalProps, Dropdown } from '@carbon/react'
import { PurchaseCategory, PurchaseCategoryState } from '@/lib/graphql/graphql'

interface PurchaseCategoryUpdate {
    name?: string | null
    description?: string | null
    state?: PurchaseCategoryState | null
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
        state: category.state || PurchaseCategoryState.Active,
    })

    useEffect(() => {
        if (category) {
            setUpdatedCategory({
                name: category.name || '',
                description: category.description || null,
                state: category.state || PurchaseCategoryState.Active,
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
                <Dropdown
                    id="edit-category-state"
                    titleText="Status"
                    label="Select status"
                    items={[
                        { id: PurchaseCategoryState.Active, label: 'Active' },
                        { id: PurchaseCategoryState.Inactive, label: 'Inactive' },
                    ]}
                    itemToString={(item) => item?.label || ''}
                    selectedItem={updatedCategory.state === PurchaseCategoryState.Active
                        ? { id: PurchaseCategoryState.Active, label: 'Active' }
                        : { id: PurchaseCategoryState.Inactive, label: 'Inactive' }}
                    onChange={(e) => {
                        if (e.selectedItem) {
                            setUpdatedCategory(prev => ({ ...prev, state: e.selectedItem?.id as PurchaseCategoryState }))
                        }
                    }}
                />
            </Form>
        </Modal>
    )
}

export default EditPurchaseCategoryModal
