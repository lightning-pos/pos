'use client'
import React, { useState, useEffect, useCallback, use } from 'react'
import { useRouter } from 'next/navigation'
import {
    Content,
    Button,
    Breadcrumb,
    BreadcrumbItem,
    Grid,
    Column,
    Tile,
    Stack,
    Tabs,
    Tab,
    TabList,
    TabPanels,
    TabPanel,
    Loading,
    InlineNotification,
} from '@carbon/react'
import { ArrowLeft } from '@carbon/icons-react'
import SimpleItemVariants from './simple_item_variants'
import SimpleItemDiscounts from './simple_item_discounts'
import { gql } from '@/lib/graphql/execute'
import {
    GetItemDocument,
    Item,
} from '@/lib/graphql/graphql'
import { formatCurrency } from '@/lib/util/number_format'

interface ItemDetailPageProps {
    params: Promise<{
        id: string
    }>
}

const ItemDetailPage: React.FC<ItemDetailPageProps> = ({ params }) => {
    const router = useRouter()

    // Use React.use() to unwrap the params Promise
    const unwrappedParams = use(params)
    const itemId = unwrappedParams.id

    const [item, setItem] = useState<Item | null>(null)
    const [loading, setLoading] = useState(false)
    const [error, setError] = useState<string | null>(null)

    const fetchItem = useCallback(async () => {
        setLoading(true)
        setError(null)
        try {
            // Using the dedicated getItem query
            const result = await gql(GetItemDocument, {
                id: itemId
            })

            if (result.item) {
                setItem(result.item as Item)
            } else {
                setError('Item not found')
            }
        } catch (err) {
            console.error('Error fetching item:', err)
            setError('Failed to load item details. Please try again.')
        } finally {
            setLoading(false)
        }
    }, [itemId])

    useEffect(() => {
        fetchItem()
    }, [fetchItem])

    if (loading && !item) {
        return (
            <Content className="min-h-[calc(100dvh-3rem)] p-4 flex flex-col items-center justify-center">
                <Loading description="Loading item details" withOverlay={false} />
            </Content>
        )
    }

    if (error) {
        return (
            <Content className="min-h-[calc(100dvh-3rem)] p-4 flex flex-col">
                <InlineNotification
                    kind="error"
                    title="Error"
                    subtitle={error}
                    onCloseButtonClick={() => setError(null)}
                />
                <div className="flex items-center mt-4">
                    <Button
                        kind="ghost"
                        size="sm"
                        renderIcon={ArrowLeft}
                        iconDescription="Back"
                        onClick={() => router.push('/dash/catalog/items')}
                        hasIconOnly
                        className="mr-2"
                    />
                    <span>Back to Items</span>
                </div>
            </Content>
        )
    }

    if (!item) {
        return (
            <Content className="min-h-[calc(100dvh-3rem)] p-4 flex flex-col">
                <InlineNotification
                    kind="error"
                    title="Not Found"
                    subtitle="The requested item could not be found."
                />
                <div className="flex items-center mt-4">
                    <Button
                        kind="ghost"
                        size="sm"
                        renderIcon={ArrowLeft}
                        iconDescription="Back"
                        onClick={() => router.push('/dash/catalog/items')}
                        hasIconOnly
                        className="mr-2"
                    />
                    <span>Back to Items</span>
                </div>
            </Content>
        )
    }

    return (
        <Content className="min-h-[calc(100dvh-3rem)] p-4 flex flex-col">
            <div className="flex items-center mb-4">
                <Button
                    kind="ghost"
                    size="sm"
                    renderIcon={ArrowLeft}
                    iconDescription="Back"
                    onClick={() => router.push('/dash/catalog/items')}
                    hasIconOnly
                    className="mr-2"
                />
                <Breadcrumb>
                    <BreadcrumbItem onClick={() => router.push('/dash/catalog/items')}>
                        Items
                    </BreadcrumbItem>
                    <BreadcrumbItem isCurrentPage>
                        {item.name}
                    </BreadcrumbItem>
                </Breadcrumb>
            </div>

            <Grid className="mb-4 p-0">
                <Column lg={16} md={8} sm={4} className='m-0 p-0'>
                    <Tile>
                        <Stack gap={4}>
                            <h2 className="text-2xl font-bold">{item.name}</h2>
                            {item.description && (
                                <p className="text-gray-600">{item.description}</p>
                            )}
                            <div className="flex flex-wrap gap-4">
                                <div>
                                    <span className="text-sm text-gray-500">Price:</span>
                                    <div className="text-lg font-semibold">{formatCurrency(parseFloat(item.price))}</div>
                                </div>
                                <div>
                                    <span className="text-sm text-gray-500">Category:</span>
                                    <div className="text-lg">{item.category.name}</div>
                                </div>
                                <div>
                                    <span className="text-sm text-gray-500">Nature:</span>
                                    <div className="text-lg">{item.nature}</div>
                                </div>
                                <div>
                                    <span className="text-sm text-gray-500">State:</span>
                                    <div className="text-lg">{item.state}</div>
                                </div>
                            </div>
                        </Stack>
                    </Tile>
                </Column>
            </Grid>
        </Content>
    )
}

export default ItemDetailPage
