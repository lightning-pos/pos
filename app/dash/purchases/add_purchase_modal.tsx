import React, { useState } from 'react';
import { Modal, TextInput, NumberInput, Button, ModalProps } from '@carbon/react';
import { useDb } from '@/components/providers/drizzle_provider';
import { purchaseOrdersTable, purchaseOrderItemsTable } from '@/lib/db/sqlite/schema';
import { uid } from 'uid';

interface AddPurchaseModalProps extends ModalProps { }

const AddPurchaseModal: React.FC<AddPurchaseModalProps> = ({ open, onRequestClose }) => {
  const db = useDb();
  const [supplierName, setSupplierName] = useState('');
  const [items, setItems] = useState([{ name: '', quantity: 1, price: 0 }]);

  const handleAddItem = () => {
    setItems([...items, { name: '', quantity: 1, price: 0 }]);
  };

  const handleItemChange = (index: number, field: string, value: string | number) => {
    const newItems = [...items];
    newItems[index] = { ...newItems[index], [field]: value };
    setItems(newItems);
  };

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    const purchaseId = uid();
    const totalAmount = items.reduce((sum, item) => sum + item.quantity * item.price, 0);

    await db.insert(purchaseOrdersTable).values({
      id: purchaseId,
      supplierName,
      totalAmount,
      state: 'open',
    });

    for (const item of items) {
      await db.insert(purchaseOrderItemsTable).values({
        id: uid(),
        purchaseOrderId: purchaseId,
        itemName: item.name,
        quantity: item.quantity,
        priceAmount: item.price,
      });
    }

    onRequestClose?.(e);
  };

  return (
    <Modal
      open={open}
      onRequestClose={onRequestClose}
      modalHeading="Add New Purchase"
      primaryButtonText="Add Purchase"
      secondaryButtonText="Cancel"
      onRequestSubmit={handleSubmit}
    >
      <TextInput
        id="supplierName"
        labelText="Supplier Name"
        value={supplierName}
        onChange={(e) => setSupplierName(e.target.value)}
        className="mb-4"
      />
      {items.map((item, index) => (
        <div key={index} className="mb-4">
          <TextInput
            id={`itemName-${index}`}
            labelText="Item Name"
            value={item.name}
            onChange={(e) => handleItemChange(index, 'name', e.target.value)}
            className="mb-2"
          />
          <NumberInput
            id={`quantity-${index}`}
            label="Quantity"
            value={item.quantity}
            onChange={(e) => handleItemChange(index, 'quantity', parseInt(e.target.value))}
            min={1}
            className="mb-2"
          />
          <NumberInput
            id={`price-${index}`}
            label="Price"
            value={item.price}
            onChange={(e) => handleItemChange(index, 'price', parseFloat(e.target.value))}
            min={0}
            step={0.01}
          />
        </div>
      ))}
      <Button onClick={handleAddItem} kind="tertiary" className="mt-4">
        Add Another Item
      </Button>
    </Modal>
  );
};

export default AddPurchaseModal;
