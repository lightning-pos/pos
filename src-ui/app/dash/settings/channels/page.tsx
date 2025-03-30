'use client'
import React, { useState, useEffect, useCallback } from 'react'
import { Content } from '@carbon/react'
import DataTable from '@/components/ui/DataTable'
import AddChannelModal from '@/app/dash/settings/channels/add_channel_modal'
import EditChannelModal from '@/app/dash/settings/channels/edit_channel_modal'
import DeleteChannelModal from '@/app/dash/settings/channels/delete_channel_modal'
import { gql } from '@/lib/graphql/execute'
import { GetChannelsDocument, CreateChannelDocument, UpdateChannelDocument, DeleteChannelDocument, Channel } from '@/lib/graphql/graphql'


// Define the table row structure
interface TableRow extends Channel {
    status: string
}

const ChannelsPage = () => {
    // State declarations
    const [channels, setChannels] = useState<TableRow[]>([])
    const [totalChannels, setTotalChannels] = useState(0)
    const [loading, setLoading] = useState(true)
    const [newChannel, setNewChannel] = useState<Partial<Channel>>({})
    const [editingChannel, setEditingChannel] = useState<Channel | null>(null)
    const [isAddModalOpen, setIsAddModalOpen] = useState(false)
    const [isEditModalOpen, setIsEditModalOpen] = useState(false)
    const [isDeleteModalOpen, setIsDeleteModalOpen] = useState(false)
    const [currentPage, setCurrentPage] = useState(1)
    const [pageSize, setPageSize] = useState(10)

    // Format channel data for table
    const formatChannelData = (channels: Channel[]): TableRow[] => {
        return channels.map(channel => ({
            ...channel,
            status: channel.isActive ? 'Active' : 'Inactive'
        }));
    };

    // Fetch channels
    const fetchChannels = useCallback(async () => {
        setLoading(true)
        try {
            // Note: The actual implementation will use pagination parameters
            // once backend supports it
            const result = await gql(GetChannelsDocument)
            if (result.channels) {
                setChannels(formatChannelData(result.channels))
                setTotalChannels(result.channels.length)
            }
        } catch (error) {
            console.error('Error fetching channels:', error)
        } finally {
            setLoading(false)
        }
    }, [])

    // Fetch channels on component mount
    useEffect(() => {
        fetchChannels()
    }, [fetchChannels])

    // Create channel
    const handleCreateChannel = async () => {
        try {
            await gql(CreateChannelDocument, {
                input: {
                    name: newChannel.name || '',
                    description: newChannel.description,
                    isActive: newChannel.isActive ?? true
                }
            })
            setIsAddModalOpen(false)
            setNewChannel({})
            fetchChannels()
        } catch (error) {
            console.error('Error creating channel:', error)
        }
    }

    // Update channel
    const handleUpdateChannel = async () => {
        if (!editingChannel?.id) return

        try {
            await gql(UpdateChannelDocument, {
                input: {
                    id: editingChannel.id,
                    name: editingChannel.name,
                    description: editingChannel.description,
                    isActive: editingChannel.isActive
                }
            })
            setIsEditModalOpen(false)
            setEditingChannel(null)
            fetchChannels()
        } catch (error) {
            console.error('Error updating channel:', error)
        }
    }

    // Delete channel
    const handleDeleteChannel = async (id: string) => {
        try {
            await gql(DeleteChannelDocument, { id })
            setIsDeleteModalOpen(false)
            setEditingChannel(null)
            fetchChannels()
        } catch (error) {
            console.error('Error deleting channel:', error)
        }
    }

    const headers = [
        { key: 'name', header: 'Name' },
        { key: 'description', header: 'Description' },
        { key: 'status', header: 'Status' }
    ]

    return (
        <Content className='min-h-[calc(100dvh-3rem)] p-0 flex flex-col'>
            <div className="p-4 flex-grow flex flex-col" style={{ minHeight: 'calc(100vh - 12rem)' }}>
                <DataTable<TableRow>
                    title="Sales Channels"
                    description="Manage the sales channels through which your business sells products and services."
                    headers={headers}
                    tableRows={channels}
                    loading={loading}
                    totalItems={totalChannels}
                    currentPage={currentPage}
                    pageSize={pageSize}
                    pageSizes={[10, 20, 30, 40, 50]}
                    onPageChange={(page, size) => {
                        setCurrentPage(page)
                        setPageSize(size)
                        // In the future, this will pass pagination to fetchChannels
                    }}
                    onAddClick={() => {
                        setNewChannel({
                            isActive: true
                        })
                        setIsAddModalOpen(true)
                    }}
                    onEditClick={(channel: TableRow) => {
                        setEditingChannel(channel)
                        setIsEditModalOpen(true)
                    }}
                    onDeleteClick={(channel: TableRow) => {
                        setEditingChannel(channel)
                        setIsDeleteModalOpen(true)
                    }}
                />
            </div>

            <AddChannelModal
                isOpen={isAddModalOpen}
                channel={newChannel}
                onClose={() => {
                    setIsAddModalOpen(false)
                    setNewChannel({})
                }}
                setChannel={setNewChannel}
                onSave={handleCreateChannel}
            />

            {editingChannel && (
                <EditChannelModal
                    isOpen={isEditModalOpen}
                    channel={editingChannel}
                    onClose={() => {
                        setIsEditModalOpen(false)
                        setEditingChannel(null)
                    }}
                    setChannel={setEditingChannel}
                    onSave={handleUpdateChannel}
                />
            )}

            <DeleteChannelModal
                isOpen={isDeleteModalOpen}
                channelId={editingChannel?.id || ''}
                channelName={editingChannel?.name || ''}
                onClose={() => {
                    setIsDeleteModalOpen(false)
                    setEditingChannel(null)
                }}
                onDelete={() => editingChannel?.id && handleDeleteChannel(editingChannel.id)}
            />
        </Content>
    )
}

export default ChannelsPage
