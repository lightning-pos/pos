'use client'
import React, { useState, useEffect } from 'react'
import { Tile } from '@carbon/react'
import { gql } from '@/lib/graphql/execute'
import { GetAnalyticsOverviewDocument, AnalyticsOverview } from '@/lib/graphql/graphql'

const AnalyticsOverviewPage = () => {
    const [overview, setOverview] = useState<AnalyticsOverview | null>(null)
    const [loading, setLoading] = useState(false)
    const days = 7

    useEffect(() => {
        const fetchOverview = async () => {
            try {
                setLoading(true)
                const result = await gql(GetAnalyticsOverviewDocument, { days })
                if (result.analyticsOverview) {
                    setOverview(result.analyticsOverview)
                }
            } catch (error) {
                console.error('Error fetching analytics overview:', error)
            } finally {
                setLoading(false)
            }
        }

        fetchOverview()
    }, [days])

    const formatCurrency = (amount: number) => {
        return new Intl.NumberFormat('en-IN', {
            style: 'currency',
            currency: 'INR',
            minimumFractionDigits: 0,
            maximumFractionDigits: 0
        }).format(amount / 100)
    }

    if (loading) {
        return <div>Loading...</div>
    }

    return (
        <>
            <h1 className='text-2xl font-bold mb-6'>Analytics Overview</h1>
            {overview ? (
                <div className='grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4'>
                    <Tile>
                        <h3 className='text-lg'>{days} Days Sales</h3>
                        <p className='text-3xl font-medium mt-4'>{formatCurrency(overview.totalSales)}</p>
                    </Tile>
                    <Tile>
                        <h3 className='text-lg'>{days} Days Orders</h3>
                        <p className='text-3xl font-medium mt-4'>{overview.totalOrders}</p>
                    </Tile>
                    <Tile>
                        <h3 className='text-lg'>Total Customers</h3>
                        <p className='text-3xl font-medium mt-4'>{overview.totalCustomers}</p>
                    </Tile>
                    <Tile>
                        <h3 className='text-lg'>Total Products</h3>
                        <p className='text-3xl font-medium mt-4'>{overview.totalProducts}</p>
                    </Tile>
                </div>
            ) : (
                <div>No data available</div>
            )}
        </>
    )
}

export default AnalyticsOverviewPage
