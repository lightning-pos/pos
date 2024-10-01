import React, { useState } from 'react'
import { Modal, TextInput, Form, ModalProps } from '@carbon/react'
import { Tax, taxesTable } from '@/lib/db/sqlite/schema'
import { useDb } from '@/components/providers/drizzle_provider'
import { uid } from 'uid'

const AddTaxModal: React.FC<ModalProps> = ({
  open,
  onRequestSubmit,
  onRequestClose,
}) => {
  const db = useDb()
  const [newTax, setNewTax] = useState<Partial<Tax>>({
    name: '',
    rate: 0,
    description: ''
  })

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target
    setNewTax(prev => ({ ...prev, [name]: name === 'rate' ? parseFloat(value) : value }))
  }

  const handleSaveTax = async (e: React.FormEvent) => {
    e.preventDefault()
    try {
      await db.insert(taxesTable).values({
        id: uid(),
        name: newTax.name ?? '',  // Provide a default value
        rate: newTax.rate ?? 0,   // Provide a default value
        description: newTax.description,
        createdAt: new Date(),
        updatedAt: new Date()
      })
      onRequestSubmit?.(e as React.FormEvent<HTMLFormElement>)
    } catch (error) {
      console.error('Error saving tax:', error)
    }
  }

  return (
    <Modal
      open={open}
      onRequestClose={onRequestClose}
      modalHeading="Add New Tax"
      primaryButtonText="Save"
      onRequestSubmit={handleSaveTax}
    >
      <Form onSubmit={handleSaveTax} className='flex flex-col gap-4'>
        <TextInput
          id="tax-name"
          name="name"
          labelText="Tax Name"
          value={newTax.name || ''}
          onChange={handleInputChange}
          required
        />
        <TextInput
          id="tax-rate"
          name="rate"
          labelText="Tax Rate (%)"
          type="number"
          value={newTax.rate || ''}
          onChange={handleInputChange}
          required
        />
        <TextInput
          id="tax-description"
          name="description"
          labelText="Description"
          value={newTax.description || ''}
          onChange={handleInputChange}
        />
      </Form>
    </Modal>
  )
}

export default AddTaxModal
