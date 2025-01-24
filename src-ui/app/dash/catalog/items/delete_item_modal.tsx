import React from 'react'
import { Modal, ModalProps } from '@carbon/react'
import { itemsTable } from '@/lib/db/sqlite/schema'
import { eq } from 'drizzle-orm'
import { useDb } from '@/components/providers/drizzle_provider';

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
  const db = useDb()

  const deleteItem = async (e: React.FormEvent<HTMLFormElement>) => {
    await db.delete(itemsTable)
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
