import React, { useEffect, useState } from "react";
import { Modal, TextInput, Form, TextArea, NumberInput, Select, SelectItem, MultiSelect, Button } from '@carbon/react'
import { Item, ItemCategory, NewItem, Tax } from '@/lib/pglite/schema'

interface SaveItemModalProps {
  isOpen: boolean;
  onClose: () => void;
  onSave: (item: Item | NewItem) => Promise<void>;
  item: Item | NewItem | null;
  categories: ItemCategory[];
  taxesList: Tax[];
  selectedTaxes: string[];
}

const SaveItemModal: React.FC<SaveItemModalProps> = ({
  isOpen,
  onClose,
  onSave,
  item,
  categories,
  taxesList,
  selectedTaxes,
}) => {
  const [localItem, setLocalItem] = useState<Item | NewItem | null>(null);

  useEffect(() => {
    setLocalItem(item);
  }, [item]);

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement | HTMLTextAreaElement>) => {
    const { name, value } = e.target;
    setLocalItem((prev) => prev ? { ...prev, [name]: value } : null);
  };

  const handlePriceChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const value = parseFloat(e.target.value);
    setLocalItem((prev) => prev ? { ...prev, price: isNaN(value) ? 0 : value * 100 } : null);
  };

  const handleTaxChange = ({ selectedItems }: { selectedItems: { id: string }[] }) => {
    const selectedTaxIds = selectedItems.map(item => item.id).join(',');
    setLocalItem((prev) => prev ? { ...prev, taxIds: selectedTaxIds } : null);
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (localItem) {
      onSave(localItem);
    }
  };

  if (!isOpen || !localItem) return null;

  return (
    <Modal open={isOpen} onRequestClose={onClose} modalHeading={item?.id ? "Edit Item" : "Add New Item"} primaryButtonText={item?.id ? "Save Changes" : "Add Item"}>
      <Form onSubmit={handleSubmit}>
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
          initialSelectedItems={
            selectedTaxes
              ? selectedTaxes.map(tax => ({ id: tax }))
              : []
          }
          onChange={handleTaxChange}
        />
      </Form>
    </Modal>
  );
};

export default SaveItemModal;
