import React, { useState } from 'react'
import { Modal, TextInput, Form, TextArea, ModalProps, Dropdown } from '@carbon/react'
import { PurchaseCategoryState } from '@/lib/graphql/graphql'

interface AddPurchaseCategoryModalProps extends Omit<ModalProps, 'onSubmit'> {
    onSave: (name: string, description: string | null, state: PurchaseCategoryState) => Promise<void>
}

const AddPurchaseCategoryModal: React.FC<AddPurchaseCategoryModalProps> = ({
    open,
    onRequestClose,
    onSave,
}) => {
    const [name, setName] = useState('')
    const [description, setDescription] = useState<string | null>(null)
    const [state, setState] = useState<PurchaseCategoryState>(PurchaseCategoryState.Active)

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault()
        await onSave(name, description, state)
        setName('')
        setDescription(null)
        setState(PurchaseCategoryState.Active)
    }

    const handleClose = (e: React.SyntheticEvent<HTMLElement>) => {
        onRequestClose?.(e)
        setName('')
        setDescription(null)
        setState(PurchaseCategoryState.Active)
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
                <Dropdown
                    id="category-state"
                    titleText="Status"
                    label="Select status"
                    items={[
                        { id: PurchaseCategoryState.Active, label: 'Active' },
                        { id: PurchaseCategoryState.Inactive, label: 'Inactive' },
                    ]}
                    itemToString={(item) => item?.label || ''}
                    selectedItem={state === PurchaseCategoryState.Active
                        ? { id: PurchaseCategoryState.Active, label: 'Active' }
                        : { id: PurchaseCategoryState.Inactive, label: 'Inactive' }}
                    onChange={(e) => {
                        if (e.selectedItem) {
                            setState(e.selectedItem?.id as PurchaseCategoryState)
                        }
                    }}
                />
            </Form>
        </Modal>
    )
}

export default AddPurchaseCategoryModal
