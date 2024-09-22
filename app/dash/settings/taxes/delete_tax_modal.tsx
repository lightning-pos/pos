import React from 'react'
import { Modal } from '@carbon/react'
import { useTaxes } from './taxes_context'

const DeleteTaxModal = () => {
    const {
        editingTax,
        isDeleteModalOpen,
        handleDeleteTax,
        setEditingTax,
        setIsDeleteModalOpen
    } = useTaxes()

    return (
        <Modal
            open={isDeleteModalOpen}
            onRequestClose={() => {
                setIsDeleteModalOpen(false)
                setEditingTax(null)
            }}
            modalHeading="Delete Tax"
            primaryButtonText="Delete"
            secondaryButtonText="Cancel"
            danger
            onRequestSubmit={handleDeleteTax}
        >
            <p>Are you sure you want to delete the tax &quot;{editingTax?.name}&quot;? This action cannot be undone.</p>
        </Modal>
    )
}

export default DeleteTaxModal
