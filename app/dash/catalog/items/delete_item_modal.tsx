import React from 'react'
import { Modal, ModalProps } from '@carbon/react'
import { itemsTable } from '@/lib/pglite/schema'
import { eq } from 'drizzle-orm'
import { drizzleDb } from '@/components/providers/system_provider';

interface DeleteItemModalProps extends ModalProps {
  itemId: string;
  itemName: string;
}

const DeleteItemModal: React.FC<DeleteItemModalProps> = ({
  open,
  onRequestClose,
  onRequestSubmit,
  itemId,
  itemName
}) => {
  const deleteItem = async (e: React.FormEvent<HTMLFormElement>) => {
    await drizzleDb.delete(itemsTable)
      .where(eq(itemsTable.id, itemId))
      .execute();

    onRequestSubmit?.(e);
  }


  return (
    <Modal
      open={open}
      modalHeading="Delete Item"
      primaryButtonText="Delete"
      onRequestClose={onRequestClose}
      onRequestSubmit={deleteItem}
      danger
    >
      <p>Are you sure you want to delete the item &quot;{itemName}&quot;? This action cannot be undone.</p>
    </Modal>
  )
}

export default DeleteItemModal
