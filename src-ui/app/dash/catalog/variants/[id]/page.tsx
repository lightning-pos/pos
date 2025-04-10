'use client'
import React, { useState, useEffect, useCallback, use } from 'react'
import { useRouter } from 'next/navigation'
import { Content, Button, Breadcrumb, BreadcrumbItem } from '@carbon/react'
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
    params: Promise<{
        id: string
    }>
}

const VariantTypePage: React.FC<VariantTypePageProps> = ({ params }) => {
    const router = useRouter()

    // Use React.use() to unwrap the params Promise
    const unwrappedParams = use(params)
    const variantTypeId = unwrappedParams.id
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
            <div className="flex items-center mb-4">
                <Button
                    kind="ghost"
                    size="sm"
                    renderIcon={ArrowLeft}
                    iconDescription="Back"
                    onClick={() => router.push('/dash/catalog/variants')}
                    hasIconOnly
                    className="mr-2"
                />
                <Breadcrumb>
                    <BreadcrumbItem onClick={() => router.push('/dash/catalog/variants')}>
                        Variants
                    </BreadcrumbItem>
                    <BreadcrumbItem isCurrentPage>
                        {variantType?.name || 'Loading...'}
                    </BreadcrumbItem>
                </Breadcrumb>
            </div>

            <div className="flex-grow" style={{ height: 'calc(100vh - 12rem)' }}>
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
                    variantTypeName={variantType?.name}
                />
            </div>
        </Content>
    )
}

export default VariantTypePage
