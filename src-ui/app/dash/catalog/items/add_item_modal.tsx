import React, { useState } from "react";
import { Modal, TextInput, TextArea, NumberInput, Select, SelectItem, MultiSelect, ModalProps } from '@carbon/react'
import { NewItem, ItemCategory, Tax, itemsTable, itemTaxesTable } from '@/lib/db/sqlite/schema'
import { uid } from "uid";
import { useDb } from "@/components/providers/drizzle_provider";
import { invoke } from "@tauri-apps/api/core";

interface AddItemModalProps extends ModalProps {
    categories: ItemCategory[];
    taxesList: Tax[];
}

const AddItemModal: React.FC<AddItemModalProps> = ({
    open,
    onRequestClose,
    onRequestSubmit,
    categories,
    taxesList,
}) => {
    const db = useDb()
    const [newItem, setNewItem] = useState<NewItem>({
        id: "",
        name: "",
        description: "",
        price: 0,
        categoryId: "",
        createdAt: new Date(),
        updatedAt: new Date(),
    });
    const [selectedTaxIds, setSelectedTaxIds] = useState<string[]>([]);

    const handleInputChange = (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement | HTMLTextAreaElement>) => {
        const { name, value } = e.target;
        setNewItem((prev) => ({ ...prev, [name]: value }));
    };

    const handlePriceChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const value = parseFloat(e.target.value);
        setNewItem((prev) => ({ ...prev, price: isNaN(value) ? 0 : value * 100 }));
    };

    const handleTaxChange = ({ selectedItems }: { selectedItems: { id: string }[] }) => {
        setSelectedTaxIds(selectedItems.map(item => item.id));
    };

    const addItem = async (e: React.FormEvent) => {
        await invoke('graphql', {
            query: `#graphql
            mutation {
                    createItem(
                        item: {
                            name: "${newItem.name}",
                            description: "${newItem.description}",
                            price: "${newItem.price}",
                            nature: GOODS,
                            state: ACTIVE,
                            categoryId: "${newItem.categoryId}",
                        }
                    ) {
                        id
                    }
                }`
        })

        onRequestSubmit?.(e as React.FormEvent<HTMLFormElement>);
    }

    return (
        <Modal
            open={open}
            modalHeading="Add New Item"
            primaryButtonText="Add Item"
            onRequestSubmit={addItem}
            onRequestClose={onRequestClose}
        >
            <TextInput
                id="item-name"
                name="name"
                labelText="Item Name"
                value={newItem.name}
                onChange={handleInputChange}
                required
            />
            <TextArea
                id="item-description"
                name="description"
                labelText="Description"
                value={newItem.description || ''}
                onChange={handleInputChange}
            />
            <NumberInput
                id="item-price"
                name="price"
                label="Price"
                value={0}
                onChange={(event, state) => handlePriceChange(event as unknown as React.ChangeEvent<HTMLInputElement>)}
                step={0.01}
                min={0}
            />
            <Select
                id="item-category"
                name="categoryId"
                labelText="Category"
                value={newItem.categoryId}
                onChange={handleInputChange}
                required
            >
                <SelectItem disabled hidden value="" text="Choose a category" />
                {categories.map((category) => (
                    <SelectItem key={category.id} value={category.id} text={category.name} />
                ))}
            </Select>
            <MultiSelect
                id="item-taxes"
                titleText="Taxes"
                label="Select taxes"
                items={taxesList.map(tax => ({ id: tax.id, label: tax.name }))}
                initialSelectedItems={[]}
                onChange={handleTaxChange}
            />
        </Modal>
    );
};

export default AddItemModal;
