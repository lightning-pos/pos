import React from 'react'
import { Modal, TextInput, Form } from '@carbon/react'
import { useTaxes } from './taxes_context'
import { taxesTable } from '@/lib/pglite/schema'

type TaxSchema = typeof taxesTable.$inferSelect

const SaveTaxModal = () => {
  const {
    editingTax,
    isModalOpen,
    handleSaveTax,
    setEditingTax,
    setIsModalOpen
  } = useTaxes()

  return (
    <Modal
      open={isModalOpen}
      onRequestClose={() => {
        setIsModalOpen(false)
        setEditingTax(null)
      }}
      modalHeading={editingTax?.id ? "Edit Tax" : "Add New Tax"}
      primaryButtonText="Save"
      onRequestSubmit={handleSaveTax}
    >
      <Form onSubmit={handleSaveTax} className='flex flex-col gap-4'>
        <TextInput
          id="tax-name"
          labelText="Tax Name"
          value={editingTax?.name || ''}
          onChange={(e) => setEditingTax(prev => prev ? { ...prev, name: e.target.value } : null)}
          required
        />
        <TextInput
          id="tax-rate"
          labelText="Tax Rate (%)"
          type="number"
          value={editingTax?.rate || ''}
          onChange={(e) => setEditingTax(prev => prev ? { ...prev, rate: parseFloat(e.target.value) } : null)}
          required
        />
        <TextInput
          id="tax-description"
          labelText="Description"
          value={editingTax?.description || ''}
          onChange={(e) => setEditingTax(prev => prev ? { ...prev, description: e.target.value } : null)}
        />
      </Form>
    </Modal>
  )
}

export default SaveTaxModal
