import React, { useState, useEffect } from "react";
import { Modal, TextInput, TextArea, NumberInput, Select, SelectItem, MultiSelect, ModalProps } from '@carbon/react'
import { invoke } from "@tauri-apps/api/core";

interface Tax {
    id: string;
    name: string;
    rate: number;
    description?: string;
}

interface ItemCategory {
    id: string;
    name: string;
    description?: string;
}

interface AddItemModalProps extends ModalProps {
    categories: ItemCategory[];
}

const AddItemModal: React.FC<AddItemModalProps> = ({
    open,
    onRequestClose,
    onRequestSubmit,
    categories,
}) => {
    const [newItem, setNewItem] = useState({
        name: "",
        description: "",
        price: 0,
        categoryId: "",
    });
    const [selectedTaxIds, setSelectedTaxIds] = useState<string[]>([]);
    const [taxes, setTaxes] = useState<Tax[]>([]);

    useEffect(() => {
        // Fetch taxes when modal opens
        if (open) {
            fetchTaxes();
        }
    }, [open]);

    const fetchTaxes = async () => {
        try {
            const result = await invoke('graphql', {
                query: `#graphql
                    query {
                        taxes {
                            id
                            name
                            rate
                            description
                        }
                    }
                `
            }) as { data: { taxes: Tax[] } };
            setTaxes(result[0].taxes.map(tax => ({
                ...tax,
                rate: tax.rate / 100 // Convert from basis points to percentage
            })));
        } catch (error) {
            console.error('Error fetching taxes:', error);
        }
    };

    const handleInputChange = (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement | HTMLTextAreaElement>) => {
        const { name, value } = e.target;
        setNewItem((prev) => ({ ...prev, [name]: value }));
    };

    const handlePriceChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const value = parseFloat(e.target.value);
        setNewItem((prev) => ({ ...prev, price: isNaN(value) ? 0 : value * 100 }));
    };

    const handleTaxChange = ({ selectedItems }: { selectedItems: Tax[] }) => {
        setSelectedTaxIds(selectedItems.map(item => item.id));
    };

    const addItem = async (e: React.FormEvent) => {
        try {
            // First create the item
            const createItemResult = await invoke('graphql', {
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
            }) as { data: { createItem: { id: string } } };

            // Then assign selected taxes to the item
            const itemId = createItemResult[0].createItem.id;
            for (const taxId of selectedTaxIds) {
                await invoke('graphql', {
                    query: `#graphql
                        mutation {
                            assignTaxToItem(
                                input: {
                                    itemId: "${itemId}",
                                    taxId: "${taxId}"
                                }
                            )
                        }`
                });
            }

            onRequestSubmit?.(e as React.FormEvent<HTMLFormElement>);
        } catch (error) {
            console.error('Error creating item with taxes:', error);
        }
    };

    return (
        <Modal
            open={open}
            modalHeading="Add New Item"
            primaryButtonText="Add Item"
            onRequestSubmit={addItem}
            onRequestClose={onRequestClose}
        >
            <div className="flex flex-col gap-5">
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
                    <SelectItem value="" text="Select a category" />
                    {categories.map((category) => (
                        <SelectItem key={category.id} value={category.id} text={category.name} />
                    ))}
                </Select>
                <MultiSelect
                    id="item-taxes"
                    titleText="Taxes"
                    label="Select taxes"
                    items={taxes}
                    itemToString={(tax: Tax) => tax ? `${tax.name} (${tax.rate}%)` : ''}
                    onChange={handleTaxChange}
                />
            </div>
        </Modal>
    );
};

export default AddItemModal;
