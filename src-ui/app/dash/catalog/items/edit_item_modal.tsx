import React, { useState, useEffect } from 'react'
import { Modal, TextInput, Form, TextArea, Select, SelectItem, ModalProps } from '@carbon/react'
import { Item, ItemGroup, Tax, UpdateItem, ItemNature, ItemState } from '@/lib/graphql/graphql'

interface EditItemModalProps extends Omit<ModalProps, 'onSubmit'> {
    onSave: (item: UpdateItem) => Promise<void>
    item: Item
    categories: ItemGroup[]
    taxes: Tax[]
}

const EditItemModal: React.FC<EditItemModalProps> = ({
    open,
    onRequestClose,
    onSave,
    item,
    categories,
    taxes,
}) => {
    const [localItem, setLocalItem] = useState<UpdateItem>({
        id: item?.id,
        name: item?.name,
        description: item?.description,
        nature: item?.nature,
        state: item?.state,
        price: item?.price,
        categoryId: item?.category?.id,
    })

    useEffect(() => {
        if (item && open) {
            setLocalItem({
                id: item.id,
                name: item.name,
                description: item.description,
                nature: item.nature,
                state: item.state,
                price: item.price,
                categoryId: item.category?.id,
            })
        }
    }, [item, open])

    const handleClose = (e: React.SyntheticEvent<HTMLElement>) => {
        onRequestClose?.(e)
    }

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault()
        await onSave(localItem)
    }

    return (
        <Modal
            open={open}
            onRequestClose={handleClose}
            modalHeading="Edit Item"
            primaryButtonText="Save"
            onRequestSubmit={handleSubmit}
        >
            <Form onSubmit={handleSubmit} className='flex flex-col gap-4'>
                <TextInput
                    id="item-name"
                    labelText="Item Name"
                    value={localItem.name || ''}
                    onChange={(e) => setLocalItem(prev => ({ ...prev, name: e.target.value }))}
                    required
                />
                <TextArea
                    id="item-description"
                    labelText="Description"
                    value={localItem.description || ''}
                    onChange={(e) => setLocalItem(prev => ({ ...prev, description: e.target.value }))}
                />
                <TextInput
                    id="item-price"
                    labelText="Price"
                    type="number"
                    value={localItem.price || '0'}
                    onChange={(e) => setLocalItem(prev => ({ ...prev, price: e.target.value }))}
                    required
                />
                <Select
                    id="item-nature"
                    labelText="Nature"
                    value={localItem.nature || ItemNature.Goods}
                    onChange={(e) => setLocalItem(prev => ({ ...prev, nature: e.target.value as ItemNature }))}
                    required
                >
                    <SelectItem value={ItemNature.Goods} text="Goods" />
                    <SelectItem value={ItemNature.Service} text="Service" />
                </Select>
                <Select
                    id="item-state"
                    labelText="State"
                    value={localItem.state || ItemState.Active}
                    onChange={(e) => setLocalItem(prev => ({ ...prev, state: e.target.value as ItemState }))}
                    required
                >
                    <SelectItem value={ItemState.Active} text="Active" />
                    <SelectItem value={ItemState.Inactive} text="Inactive" />
                    <SelectItem value={ItemState.Deleted} text="Deleted" />
                </Select>
                <Select
                    id="item-category"
                    labelText="Category"
                    value={localItem.categoryId || ''}
                    onChange={(e) => setLocalItem(prev => ({ ...prev, categoryId: e.target.value }))}
                    required
                >
                    <SelectItem disabled value="" text="Choose a category" />
                    {categories.map((category) => (
                        <SelectItem
                            key={category.id}
                            value={category.id}
                            text={category.name}
                        />
                    ))}
                </Select>
            </Form>
        </Modal>
    )
}

export default EditItemModal
