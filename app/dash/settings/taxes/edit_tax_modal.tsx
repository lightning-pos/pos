import React, { useState, useEffect } from 'react'
import { Modal, TextInput, Form, ModalProps } from '@carbon/react'
import { Tax, taxesTable } from '@/lib/db/sqlite/schema'
import { useDb } from '@/components/providers/drizzle_provider'
import { eq } from 'drizzle-orm'

interface EditTaxModalProps extends ModalProps {
  tax: Tax
}

const EditTaxModal: React.FC<EditTaxModalProps> = ({
  open,
  onRequestClose,
  onRequestSubmit,
  tax
}) => {
  const db = useDb()
  const [editingTax, setEditingTax] = useState<Tax | null>(null)

  useEffect(() => {
    setEditingTax(tax)
  }, [tax])

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target
    setEditingTax(prev => prev ? { ...prev, [name]: name === 'rate' ? parseFloat(value) : value } : null)
  }

  const handleSaveTax = async (e: React.FormEvent) => {
    e.preventDefault()
    if (!editingTax) return
    try {
      await db.update(taxesTable)
        .set({
          name: editingTax.name,
          rate: editingTax.rate,
          description: editingTax.description
        })
        .where(eq(taxesTable.id, editingTax.id))
      onRequestClose?.(e as React.FormEvent<HTMLFormElement>)
    } catch (error) {
      console.error('Error updating tax:', error)
    }
  }

  if (!editingTax) return null

  return (
    <Modal
      open={open}
      onRequestClose={onRequestClose}
      modalHeading="Edit Tax"
      primaryButtonText="Save Changes"
      onRequestSubmit={handleSaveTax}
    >
      <Form onSubmit={handleSaveTax} className='flex flex-col gap-4'>
        <TextInput
          id="tax-name"
          name="name"
          labelText="Tax Name"
          value={editingTax.name || ''}
          onChange={handleInputChange}
          required
        />
        <TextInput
          id="tax-rate"
          name="rate"
          labelText="Tax Rate (%)"
          type="number"
          value={editingTax.rate || ''}
          onChange={handleInputChange}
          required
        />
        <TextInput
          id="tax-description"
          name="description"
          labelText="Description"
          value={editingTax.description || ''}
          onChange={handleInputChange}
        />
      </Form>
    </Modal>
  )
}

export default EditTaxModal
