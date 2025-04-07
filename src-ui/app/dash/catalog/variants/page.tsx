'use client'
import React, { useState, useEffect, useCallback } from 'react'
import { Content } from '@carbon/react'
import VariantTypesList from '@/app/dash/catalog/variants/variant_types_list'
import { gql } from '@/lib/graphql/execute'
import {
    GetVariantTypesDocument,
    GetTotalVariantTypesDocument,
    VariantType,
} from '@/lib/graphql/graphql'

const VariantsPage = () => {
    const [variantTypes, setVariantTypes] = useState<VariantType[]>([])
    const [totalVariantTypes, setTotalVariantTypes] = useState(0)
    const [loading, setLoading] = useState(false)
    const [currentPage, setCurrentPage] = useState(1)
    const [pageSize, setPageSize] = useState(10)
    const [pageSizes] = useState([10, 20, 30, 40, 50])

    const fetchVariantTypes = useCallback(async (page: number, size: number) => {
        setLoading(true)
        try {
            const offset = (page - 1) * size

            const result = await gql(GetVariantTypesDocument, {
                first: size,
                offset: offset
            })

            setVariantTypes(result.variantTypes as unknown as VariantType[])

            // Fetch total count
            const totalResult = await gql(GetTotalVariantTypesDocument)
            setTotalVariantTypes(totalResult.totalVariantTypes)
        } catch (error) {
            console.error('Error fetching variant types:', error)
        } finally {
            setLoading(false)
        }
    }, [])

    useEffect(() => {
        fetchVariantTypes(currentPage, pageSize)
    }, [fetchVariantTypes, currentPage, pageSize])

    const handleRefresh = () => {
        fetchVariantTypes(currentPage, pageSize)
    }

    return (
        <Content className="min-h-[calc(100dvh-3rem)] p-0 flex flex-col">
            <div className="p-4 flex-grow flex flex-col" style={{ height: "calc(100vh - 12rem)" }}>
                <VariantTypesList
                    variantTypes={variantTypes}
                    loading={loading}
                    totalItems={totalVariantTypes}
                    currentPage={currentPage}
                    pageSize={pageSize}
                    pageSizes={pageSizes}
                    onPageChange={(page, size) => {
                        setCurrentPage(page)
                        setPageSize(size)
                        fetchVariantTypes(page, size)
                    }}
                    onRefresh={handleRefresh}
                />
            </div>
        </Content>
    )
}

export default VariantsPage
