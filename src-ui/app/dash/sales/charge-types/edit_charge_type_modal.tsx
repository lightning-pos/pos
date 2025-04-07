'use client'
import React, { useState, useEffect } from 'react'
import { Modal, TextInput, Form, TextArea } from '@carbon/react'
import { SalesChargeType, SalesChargeTypeUpdateInput } from '@/lib/graphql/graphql'

interface EditChargeTypeModalProps {
  isOpen: boolean
  chargeType: SalesChargeType
  onClose: () => void
  onSave: (chargeType: SalesChargeTypeUpdateInput) => void
}

const EditChargeTypeModal: React.FC<EditChargeTypeModalProps> = ({
  isOpen,
  chargeType,
  onClose,
  onSave
}) => {
  const [editedChargeType, setEditedChargeType] = useState<{
    name: string
    description: string | null
  }>({
    name: '',
    description: null
  })

  const [nameError, setNameError] = useState('')

  useEffect(() => {
    if (chargeType) {
      setEditedChargeType({
        name: chargeType.name,
        description: chargeType.description || null
      })
    }
  }, [chargeType])

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
    const { name, value } = e.target
    setEditedChargeType(prev => ({ ...prev, [name]: value }))
    
    if (name === 'name' && !value.trim()) {
      setNameError('Name is required')
    } else if (name === 'name') {
      setNameError('')
    }
  }

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault()
    
    if (!editedChargeType.name.trim()) {
      setNameError('Name is required')
      return
    }
    
    const updateInput: SalesChargeTypeUpdateInput = {
      id: chargeType.id,
      name: editedChargeType.name !== chargeType.name ? editedChargeType.name : undefined,
      description: editedChargeType.description !== chargeType.description 
        ? editedChargeType.description === '' 
          ? null 
          : editedChargeType.description 
        : undefined
    }
    
    onSave(updateInput)
  }

  return (
    <Modal
      open={isOpen}
      modalHeading="Edit Charge Type"
      primaryButtonText="Save"
      secondaryButtonText="Cancel"
      onRequestSubmit={handleSubmit}
      onRequestClose={onClose}
    >
      <Form onSubmit={handleSubmit}>
        <div className="space-y-4 pt-4">
          <TextInput
            id="name"
            name="name"
            labelText="Name"
            placeholder="Enter charge type name"
            value={editedChargeType.name}
            onChange={handleInputChange}
            invalid={!!nameError}
            invalidText={nameError}
            required
          />
          
          <TextArea
            id="description"
            name="description"
            labelText="Description"
            placeholder="Enter description (optional)"
            value={editedChargeType.description || ''}
            onChange={handleInputChange}
          />
        </div>
      </Form>
    </Modal>
  )
}

export default EditChargeTypeModal
