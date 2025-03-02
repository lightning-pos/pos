import React, { useState } from 'react'
import { Modal, TextInput, Form, TextArea, Select, SelectItem, MultiSelect, ModalProps } from '@carbon/react'
import { ItemGroup, Tax, NewItem, ItemNature, ItemState } from '@/lib/graphql/graphql'
import { sanitizeDecimalInput } from '@/lib/util/number_format'

interface AddItemModalProps extends Omit<ModalProps, 'onSubmit'> {
    onSave: (item: NewItem) => Promise<void>
    categories: ItemGroup[]
    taxes: Tax[]
}

const AddItemModal: React.FC<AddItemModalProps> = ({
    open,
    onRequestClose,
    onSave,
    categories,
    taxes,
}) => {
    const [newItem, setNewItem] = useState<NewItem>({
        name: '',
        description: '',
        nature: ItemNature.Goods,
        state: ItemState.Active,
        price: '0',
        categoryId: '',
        taxIds: [],
    })

    const handleClose = (e: React.SyntheticEvent<HTMLElement>) => {
        onRequestClose?.(e)
        setNewItem({
            name: '',
            description: '',
            nature: ItemNature.Goods,
            state: ItemState.Active,
            price: '0',
            categoryId: '',
            taxIds: [],
        })
    }

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault()
        await onSave(newItem)
        setNewItem({
            name: '',
            description: '',
            nature: ItemNature.Goods,
            state: ItemState.Active,
            price: '0',
            categoryId: '',
            taxIds: [],
        })
    }

    const handlePriceChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const value = sanitizeDecimalInput(e.target.value, 2)
        setNewItem(prev => ({ ...prev, price: value }))
    }

    return (
        <Modal
            open={open}
            onRequestClose={handleClose}
            modalHeading="Add New Item"
            primaryButtonText="Save"
            onRequestSubmit={handleSubmit}
        >
            <Form onSubmit={handleSubmit} className='flex flex-col gap-4'>
                <TextInput
                    id="item-name"
                    labelText="Item Name"
                    value={newItem.name}
                    onChange={(e) => setNewItem(prev => ({ ...prev, name: e.target.value }))}
                    required
                />
                <TextArea
                    id="item-description"
                    labelText="Description"
                    value={newItem.description || ''}
                    onChange={(e) => setNewItem(prev => ({ ...prev, description: e.target.value }))}
                />
                <TextInput
                    id="item-price"
                    labelText="Price"
                    value={newItem.price}
                    onChange={handlePriceChange}
                    required
                />
                <Select
                    id="item-nature"
                    labelText="Nature"
                    value={newItem.nature}
                    onChange={(e) => setNewItem(prev => ({ ...prev, nature: e.target.value as ItemNature }))}
                    required
                >
                    <SelectItem value={ItemNature.Goods} text="Goods" />
                    <SelectItem value={ItemNature.Service} text="Service" />
                </Select>
                <Select
                    id="item-state"
                    labelText="State"
                    value={newItem.state}
                    onChange={(e) => setNewItem(prev => ({ ...prev, state: e.target.value as ItemState }))}
                    required
                >
                    <SelectItem value={ItemState.Active} text="Active" />
                    <SelectItem value={ItemState.Inactive} text="Inactive" />
                    <SelectItem value={ItemState.Deleted} text="Deleted" />
                </Select>
                <Select
                    id="item-category"
                    labelText="Category"
                    value={newItem.categoryId}
                    onChange={(e) => setNewItem(prev => ({ ...prev, categoryId: e.target.value }))}
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
                <MultiSelect
                    id="item-taxes"
                    titleText="Taxes"
                    label="Choose taxes"
                    items={taxes}
                    itemToString={(tax: Tax) => tax?.name || ''}
                    selectedItems={taxes.filter(tax => newItem?.taxIds?.includes(tax.id))}
                    onChange={(e) => setNewItem(prev => ({
                        ...prev,
                        taxIds: e?.selectedItems?.map(tax => tax.id)
                    }))}
                />
            </Form>
        </Modal>
    )
}

export default AddItemModal
