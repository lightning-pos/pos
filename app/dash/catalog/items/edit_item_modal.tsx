import React, { useEffect, useState } from "react";
import { Modal, TextInput, Form, TextArea, NumberInput, Select, SelectItem, MultiSelect, ModalProps } from '@carbon/react'
import { Item, ItemCategory, itemsTable, itemTaxesTable, Tax } from '@/lib/db/sqlite/schema'
import { useDb } from "@/components/providers/drizzle_provider";
import { eq } from "drizzle-orm";

interface EditItemModalProps extends ModalProps {
  item: Item | null;
  categories: ItemCategory[];
  taxesList: Tax[];
  selectedTaxes: string[];
}

const EditItemModal: React.FC<EditItemModalProps> = ({
  open,
  onRequestClose,
  onRequestSubmit,
  item,
  selectedTaxes,
  categories,
  taxesList,
}) => {
  const db = useDb()
  const [localItem, setLocalItem] = useState<Item | null>(null);
  const [localSelectedTaxes, setLocalSelectedTaxes] = useState<string[]>([]);

  useEffect(() => {
    setLocalItem(item);
    setLocalSelectedTaxes(selectedTaxes);
  }, [item, selectedTaxes]);

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement | HTMLTextAreaElement>) => {
    const { name, value } = e.target;
    setLocalItem((prev) => prev ? { ...prev, [name]: value } : null);
  };

  const handlePriceChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const value = parseFloat(e.target.value);
    setLocalItem((prev) => prev ? { ...prev, price: isNaN(value) ? 0 : value * 100 } : null);
  };

  const handleTaxChange = ({ selectedItems }: { selectedItems: { id: string }[] }) => {
    setLocalSelectedTaxes(selectedItems.map(item => item.id));
  };

  const editItem = async (e: React.FormEvent) => {
    if (!localItem) return;

    await db.transaction(async (tx) => {
      const currentUtcTime = new Date()
      const currentLocalTime = new Date(currentUtcTime.toLocaleString('en-IN', { timeZone: 'Asia/Kolkata' }))
      await tx.update(itemsTable).set({
        name: localItem.name,
        description: localItem.description,
        price: Number(localItem.price),
        categoryId: localItem.categoryId,
        updatedAt: currentLocalTime,
      }).where(eq(itemsTable.id, localItem.id)).execute();

      await tx.delete(itemTaxesTable).where(eq(itemTaxesTable.itemId, localItem.id)).execute();

      for (const taxId of localSelectedTaxes) {
        await tx.insert(itemTaxesTable).values({
          itemId: localItem.id,
          taxId: taxId,
        }).execute();
      }
    });

    onRequestSubmit?.(e as React.FormEvent<HTMLFormElement>);
  }

  if (!open || !localItem) return null;

  return (
    <Modal
      open={open}
      modalHeading="Edit Item"
      primaryButtonText="Save Changes"
      onRequestSubmit={editItem}
      onRequestClose={onRequestClose}
    >
      <TextInput
        id="item-name"
        name="name"
        labelText="Item Name"
        value={localItem.name || ''}
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
        value={(localItem.price || 0) / 100}
        onChange={(event, state) => handlePriceChange(event as unknown as React.ChangeEvent<HTMLInputElement>)}
        step={0.01}
        min={0}
      />
      <Select
        id="item-category"
        name="categoryId"
        labelText="Category"
        value={localItem.categoryId || ''}
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
        initialSelectedItems={localSelectedTaxes.map(tax => ({ id: tax }))}
        onChange={handleTaxChange}
      />
    </Modal>
  );
};

export default EditItemModal;
