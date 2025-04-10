'use client'
import React, { useState, useEffect } from 'react'
import {
    Modal,
    TextInput,
    TextArea,
    Form,
    MultiSelect,
    InlineLoading
} from '@carbon/react'
import { CreateTaxGroupDocument, GetAllTaxesDocument } from '@/lib/graphql/graphql'
import type { Tax } from '@/lib/graphql/graphql'
import { gql } from '@/lib/graphql/execute'

interface AddTaxGroupModalProps {
    open: boolean
    onRequestClose: () => void
    onRequestSubmit: () => void
}

const AddTaxGroupModal: React.FC<AddTaxGroupModalProps> = ({
    open,
    onRequestClose,
    onRequestSubmit
}) => {
    const [name, setName] = useState('')
    const [description, setDescription] = useState('')
    const [selectedTaxIds, setSelectedTaxIds] = useState<string[]>([])
    const [taxes, setTaxes] = useState<Tax[]>([])
    const [loading, setLoading] = useState(false)
    const [fetchingTaxes, setFetchingTaxes] = useState(false)
    const [error, setError] = useState('')

    useEffect(() => {
        if (open) {
            fetchTaxes()
        }
    }, [open])

    const fetchTaxes = async () => {
        setFetchingTaxes(true)
        try {
            const result = await gql(GetAllTaxesDocument)
            setTaxes(result.taxes as Tax[])
        } catch (error) {
            console.error('Error fetching taxes:', error)
        } finally {
            setFetchingTaxes(false)
        }
    }

    const resetForm = () => {
        setName('')
        setDescription('')
        setSelectedTaxIds([])
        setError('')
    }

    const handleClose = () => {
        resetForm()
        onRequestClose()
    }

    const handleSubmit = async () => {
        if (!name.trim()) {
            setError('Name is required')
            return
        }

        setLoading(true)
        try {
            await gql(CreateTaxGroupDocument, {
                input: {
                    name,
                    description: description || null,
                    taxIds: selectedTaxIds.length > 0 ? selectedTaxIds : null
                }
            })
            resetForm()
            onRequestSubmit()
        } catch (error) {
            console.error('Error creating tax group:', error)
            setError('Failed to create tax group')
        } finally {
            setLoading(false)
        }
    }

    return (
        <Modal
            open={open}
            modalHeading="Add Tax Group"
            primaryButtonText="Create"
            secondaryButtonText="Cancel"
            onRequestClose={handleClose}
            onRequestSubmit={handleSubmit}
            primaryButtonDisabled={loading || !name.trim()}
        >
            <Form>
                {error && <div className="text-red-500 mb-4">{error}</div>}
                <TextInput
                    id="name"
                    labelText="Name"
                    placeholder="Enter tax group name"
                    value={name}
                    onChange={(e) => setName(e.target.value)}
                    required
                    className="mb-4"
                />
                <TextArea
                    id="description"
                    labelText="Description"
                    placeholder="Enter description (optional)"
                    value={description}
                    onChange={(e) => setDescription(e.target.value)}
                    className="mb-4"
                />
                {fetchingTaxes ? (
                    <InlineLoading description="Loading taxes..." />
                ) : (
                    <MultiSelect
                        id="taxes"
                        titleText="Taxes"
                        label="Taxes"
                        helperText="Select taxes to include in this group"
                        items={taxes.map(tax => {
                            const rateStr = String(tax.rate)
                            return {
                                id: tax.id,
                                text: `${tax.name} (${rateStr}%)`
                            }
                        })}
                        initialSelectedItems={taxes
                            .filter(tax => selectedTaxIds.includes(tax.id))
                            .map(tax => {
                                const rateStr = String(tax.rate)
                                return {
                                    id: tax.id,
                                    text: `${tax.name} (${rateStr}%)`
                                }
                            })}
                        onChange={({ selectedItems }) => {
                            const newSelectedIds = selectedItems?.map(item => item.id) || []
                            setSelectedTaxIds(newSelectedIds)
                        }}
                        itemToString={(item) => (item ? item.text : '')}
                        className="mb-4"
                    />
                )}
            </Form>
        </Modal>
    )
}

export default AddTaxGroupModal
