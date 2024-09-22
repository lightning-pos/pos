import React from 'react'
import { Modal, TextInput, Form, TextArea } from '@carbon/react'
import { useCategories } from './categories_context'

const SaveCategoryModal = () => {
    const {
        editingCategory,
        isModalOpen,
        handleSaveCategory,
        setEditingCategory,
        setIsModalOpen
    } = useCategories()

    return (
        <Modal
            open={isModalOpen}
            onRequestClose={() => {
                setIsModalOpen(false)
                setEditingCategory(null)
            }}
            modalHeading={editingCategory?.id ? "Edit Category" : "Add New Category"}
            primaryButtonText="Save"
            onRequestSubmit={handleSaveCategory}
        >
            <Form onSubmit={handleSaveCategory} className='flex flex-col gap-4'>
                <TextInput
                    id="category-name"
                    labelText="Category Name"
                    value={editingCategory?.name || ''}
                    onChange={(e) => setEditingCategory(prev => prev ? { ...prev, name: e.target.value } : null)}
                    required
                />
                <TextArea
                    id="category-description"
                    labelText="Description"
                    value={editingCategory?.description || ''}
                    onChange={(e) => setEditingCategory(prev => prev ? { ...prev, description: e.target.value } : null)}
                />
            </Form>
        </Modal>
    )
}

export default SaveCategoryModal
