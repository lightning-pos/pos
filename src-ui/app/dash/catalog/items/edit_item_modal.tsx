import React, { useEffect, useState } from "react";
import { Modal, TextInput, Form, TextArea, NumberInput, Select, SelectItem, MultiSelect, ModalProps } from '@carbon/react'
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

interface Item {
    id: string;
    name: string;
    description?: string;
    price: number;
    categoryId: string;
}

interface EditItemModalProps extends ModalProps {
    item: Item;
    categories: ItemCategory[];
}

const EditItemModal: React.FC<EditItemModalProps> = ({
    open,
    onRequestClose,
    onRequestSubmit,
    item,
    categories,
}) => {
    const [localItem, setLocalItem] = useState<Item | null>(null);
    const [taxes, setTaxes] = useState<Tax[]>([]);
    const [selectedTaxIds, setSelectedTaxIds] = useState<string[]>([]);

    useEffect(() => {
        setLocalItem(item);
        if (open) {
            fetchTaxes();
            fetchItemTaxes();
        }
    }, [item, open]);

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

    const fetchItemTaxes = async () => {
        try {
            const result = await invoke('graphql', {
                query: `#graphql
                    query {
                        item(id: "${item.id}") {
                            taxes {
                                id
                            }
                        }
                    }
                `
            }) as { data: { item: { taxes: { id: string }[] } } };
            setSelectedTaxIds(result[0].item.taxes.map(tax => tax.id));
        } catch (error) {
            console.error('Error fetching item taxes:', error);
        }
    };

    const handleInputChange = (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement | HTMLTextAreaElement>) => {
        const { name, value } = e.target;
        setLocalItem((prev) => prev ? { ...prev, [name]: value } : null);
    };

    const handlePriceChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const value = parseFloat(e.target.value);
        setLocalItem((prev) => prev ? { ...prev, price: isNaN(value) ? 0 : value * 100 } : null);
    };

    const handleTaxChange = ({ selectedItems }: { selectedItems: Tax[] }) => {
        setSelectedTaxIds(selectedItems.map(item => item.id));
    };

    const editItem = async (e: React.FormEvent) => {
        if (!localItem) return;

        try {
            // Update item details
            await invoke('graphql', {
                query: `#graphql
                    mutation {
                        updateItem(
                            item: {
                                id: "${localItem.id}",
                                name: "${localItem.name}",
                                description: "${localItem.description || ''}",
                                price: "${localItem.price}",
                                categoryId: "${localItem.categoryId}",
                            }
                        ) {
                            id
                        }
                    }`
            });

            // Get current tax assignments
            const currentTaxes = await invoke('graphql', {
                query: `#graphql
                    query {
                        item(id: "${localItem.id}") {
                            taxes {
                                id
                            }
                        }
                    }
                `
            }) as { data: { item: { taxes: { id: string }[] } } };

            const currentTaxIds = new Set(currentTaxes[0].item.taxes.map(tax => tax.id));
            const newTaxIds = new Set(selectedTaxIds);

            // Remove taxes that are no longer selected
            for (const taxId of currentTaxIds) {
                if (!newTaxIds.has(taxId)) {
                    await invoke('graphql', {
                        query: `#graphql
                            mutation {
                                removeTaxFromItem(
                                    itemId: "${localItem.id}",
                                    taxId: "${taxId}"
                                )
                            }`
                    });
                }
            }

            // Add newly selected taxes
            for (const taxId of newTaxIds) {
                if (!currentTaxIds.has(taxId)) {
                    await invoke('graphql', {
                        query: `#graphql
                            mutation {
                                assignTaxToItem(
                                    input: {
                                        itemId: "${localItem.id}",
                                        taxId: "${taxId}"
                                    }
                                )
                            }`
                    });
                }
            }

            onRequestSubmit?.(e as React.FormEvent<HTMLFormElement>);
        } catch (error) {
            console.error('Error updating item:', error);
        }
    };

    if (!open || !localItem) return null;

    return (
        <Modal
            open={open}
            modalHeading="Edit Item"
            primaryButtonText="Save Changes"
            onRequestSubmit={editItem}
            onRequestClose={onRequestClose}
        >
            <div className="flex flex-col gap-5">
                <TextInput
                    id="item-name"
                    name="name"
                    labelText="Item Name"
                    value={localItem.name}
                    onChange={handleInputChange}
                    required
                />
                <TextArea
                    id="item-description"
                    name="description"
                    labelText="Description"
                    value={localItem.description || ''}
                    onChange={handleInputChange}
                />
                <NumberInput
                    id="item-price"
                    name="price"
                    label="Price"
                    value={localItem.price / 100} // Convert cents to dollars
                    onChange={(event, state) => handlePriceChange(event as unknown as React.ChangeEvent<HTMLInputElement>)}
                    step={0.01}
                    min={0}
                />
                <Select
                    id="item-category"
                    name="categoryId"
                    labelText="Category"
                    value={localItem.categoryId}
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
                    initialSelectedItems={taxes.filter(tax => selectedTaxIds.includes(tax.id))}
                    itemToString={(tax: Tax) => tax ? `${tax.name} (${tax.rate}%)` : ''}
                    onChange={handleTaxChange}
                />
            </div>
        </Modal>
    );
};

export default EditItemModal;
