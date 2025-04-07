'use client'
import React, { useState, useEffect } from 'react'
import {
    Modal,
    TextInput,
    TextArea,
    Form,
    MultiSelect,
    InlineLoading,
    Tag
} from '@carbon/react'
import {
    UpdateTaxGroupDocument,
    GetAllTaxesDocument,
    AssignTaxToGroupDocument,
    RemoveTaxFromGroupDocument,
    TaxGroup
} from '@/lib/graphql/graphql'
import type { Tax } from '@/lib/graphql/graphql'
import { gql } from '@/lib/graphql/execute'

interface EditTaxGroupModalProps {
    open: boolean
    taxGroup: TaxGroup
    onRequestClose: () => void
    onRequestSubmit: () => void
}

const EditTaxGroupModal: React.FC<EditTaxGroupModalProps> = ({
    open,
    taxGroup,
    onRequestClose,
    onRequestSubmit
}) => {
    const [name, setName] = useState(taxGroup.name || '')
    const [description, setDescription] = useState(taxGroup.description || '')
    const [selectedTaxIds, setSelectedTaxIds] = useState<string[]>([])
    const [taxes, setTaxes] = useState<Tax[]>([])
    const [loading, setLoading] = useState(false)
    const [fetchingTaxes, setFetchingTaxes] = useState(false)
    const [error, setError] = useState('')

    useEffect(() => {
        if (open) {
            setName(taxGroup.name || '')
            setDescription(taxGroup.description || '')
            setSelectedTaxIds(taxGroup.taxes?.map(tax => tax.id) || [])
            fetchTaxes()
        }
    }, [open, taxGroup])

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

    const handleClose = () => {
        setError('')
        onRequestClose()
    }

    const handleSubmit = async () => {
        if (!name.trim()) {
            setError('Name is required')
            return
        }

        setLoading(true)
        try {
            // Update the tax group basic info
            await gql(UpdateTaxGroupDocument, {
                input: {
                    id: taxGroup.id,
                    name,
                    description: description || null
                }
            })

            // Handle tax assignments
            const currentTaxIds = taxGroup.taxes?.map(tax => tax.id) || []
            const taxesToAdd = selectedTaxIds.filter(id => !currentTaxIds.includes(id))
            const taxesToRemove = currentTaxIds.filter(id => !selectedTaxIds.includes(id))

            // Add new taxes
            for (const taxId of taxesToAdd) {
                await gql(AssignTaxToGroupDocument, {
                    taxGroupId: taxGroup.id,
                    taxId
                })
            }

            // Remove taxes
            for (const taxId of taxesToRemove) {
                await gql(RemoveTaxFromGroupDocument, {
                    taxGroupId: taxGroup.id,
                    taxId
                })
            }

            onRequestSubmit()
        } catch (error) {
            console.error('Error updating tax group:', error)
            setError('Failed to update tax group')
        } finally {
            setLoading(false)
        }
    }

    return (
        <Modal
            open={open}
            modalHeading="Edit Tax Group"
            primaryButtonText="Save"
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
                            const rateStr = String(tax.rate);
                            return {
                                id: tax.id,
                                text: `${tax.name} (${rateStr}%)`
                            };
                        })}
                        initialSelectedItems={taxes
                            .filter(tax => selectedTaxIds.includes(tax.id))
                            .map(tax => {
                                const rateStr = String(tax.rate);
                                return {
                                    id: tax.id,
                                    text: `${tax.name} (${rateStr}%)`
                                };
                            })}
                        onChange={({ selectedItems }) => {
                            const newSelectedIds = selectedItems?.map(item => item.id) || [];
                            setSelectedTaxIds(newSelectedIds);
                        }}
                        itemToString={(item) => (item ? item.text : '')}
                        className="mb-4"
                    />
                )}
                <div className="mt-4">
                    <p className="text-sm font-medium mb-2">Current Taxes:</p>
                    <div className="flex flex-wrap gap-2">
                        {taxGroup.taxes && taxGroup.taxes.length > 0 ? (
                            taxGroup.taxes.map(tax => (
                                <Tag key={tax.id} type="blue">
                                    {tax.name} ({String(tax.rate)}%)
                                </Tag>
                            ))
                        ) : (
                            <span className="text-sm text-gray-500">No taxes assigned</span>
                        )}
                    </div>
                </div>
            </Form>
        </Modal>
    )
}

export default EditTaxGroupModal
