'use client'
import React, { useState, useEffect, useCallback } from 'react'
import { useRouter } from 'next/navigation'
import { Content, Button, Breadcrumb, BreadcrumbItem, Grid, Column, Tile, Stack } from '@carbon/react'
import { ArrowLeft } from '@carbon/icons-react'
import VariantValuesList from './variant_values_list'
import { gql } from '@/lib/graphql/execute'
import {
    GetVariantTypeDocument,
    GetVariantValuesDocument,
    VariantType,
    VariantValue,
} from '@/lib/graphql/graphql'

interface VariantTypePageProps {
    params: {
        id: string
    }
}

const VariantTypePage: React.FC<VariantTypePageProps> = ({ params }) => {
    const router = useRouter()

    // NOTE: We're aware of the Next.js warning about accessing params directly.
    // Since this is a client-side only project without server components,
    // we're not using React.use() at this time. This will be addressed in a future update.
    const variantTypeId = params.id
    const [variantType, setVariantType] = useState<VariantType | null>(null)
    const [variantValues, setVariantValues] = useState<VariantValue[]>([])
    const [loading, setLoading] = useState(false)
    const [currentPage, setCurrentPage] = useState(1)
    const [pageSize, setPageSize] = useState(10)
    const [pageSizes] = useState([10, 20, 30, 40, 50])

    const fetchVariantType = useCallback(async () => {
        setLoading(true)
        try {
            const result = await gql(GetVariantTypeDocument, {
                id: variantTypeId
            })
            setVariantType(result.variantType as unknown as VariantType)
        } catch (error) {
            console.error('Error fetching variant type:', error)
        } finally {
            setLoading(false)
        }
    }, [variantTypeId])

    const fetchVariantValues = useCallback(async (page: number, size: number) => {
        setLoading(true)
        try {
            const offset = (page - 1) * size

            const result = await gql(GetVariantValuesDocument, {
                variantTypeId: variantTypeId,
                first: size,
                offset: offset
            })

            setVariantValues(result.variantValues as unknown as VariantValue[])
        } catch (error) {
            console.error('Error fetching variant values:', error)
        } finally {
            setLoading(false)
        }
    }, [variantTypeId])

    useEffect(() => {
        fetchVariantType()
        fetchVariantValues(currentPage, pageSize)
    }, [fetchVariantType, fetchVariantValues, currentPage, pageSize])

    const handleRefresh = () => {
        fetchVariantType()
        fetchVariantValues(currentPage, pageSize)
    }

    return (
        <Content className="min-h-[calc(100dvh-3rem)] p-4 flex flex-col">
            <Breadcrumb className="mb-4">
                <BreadcrumbItem onClick={() => router.push('/dash/catalog/variants')}>
                    Variants
                </BreadcrumbItem>
                <BreadcrumbItem isCurrentPage>
                    {variantType?.name || 'Loading...'}
                </BreadcrumbItem>
            </Breadcrumb>

            <Button
                kind="ghost"
                renderIcon={ArrowLeft}
                iconDescription="Back"
                onClick={() => router.push('/dash/catalog/variants')}
                className="mb-4 self-start"
            >
                Back to Variant Types
            </Button>

            {variantType && (
                <Grid className="mb-4">
                    <Column lg={16} md={8} sm={4}>
                        <Tile>
                            <Stack gap={4}>
                                <h2 className="text-2xl font-bold">{variantType.name}</h2>
                                {variantType.description && (
                                    <p className="text-gray-600">{variantType.description}</p>
                                )}
                            </Stack>
                        </Tile>
                    </Column>
                </Grid>
            )}

            <div className="flex-grow" style={{ height: "calc(100vh - 20rem)" }}>
                <VariantValuesList
                    variantTypeId={variantTypeId}
                    variantValues={variantValues}
                    loading={loading}
                    totalItems={variantType?.values?.length || 0}
                    currentPage={currentPage}
                    pageSize={pageSize}
                    pageSizes={pageSizes}
                    onPageChange={(page, size) => {
                        setCurrentPage(page)
                        setPageSize(size)
                        fetchVariantValues(page, size)
                    }}
                    onRefresh={handleRefresh}
                />
            </div>
        </Content>
    )
}

export default VariantTypePage
