'use client'
import React, { useState, useEffect, useCallback } from 'react'
import { Tile } from '@carbon/react'
import { invoke } from '@tauri-apps/api/core'

interface AnalyticsOverview {
    totalSales: number
    totalOrders: number
    totalCustomers: number
    totalProducts: number
}

const AnalyticsOverview = () => {
    const [overview, setOverview] = useState<AnalyticsOverview | null>(null)
    const days = 7

    const fetchOverview = useCallback(async () => {
        try {
            const result: Array<{ analyticsOverview: AnalyticsOverview }> = await invoke('graphql', {
                query: `#graphql
                    query {
                        analyticsOverview(days: ${days}) {
                            totalSales
                            totalOrders
                            totalCustomers
                            totalProducts
                        }
                    }
                `
            })
            setOverview(result[0].analyticsOverview)
        } catch (error) {
            console.error('Error fetching analytics overview:', error)
        }
    }, [])

    useEffect(() => {
        fetchOverview()
    }, [fetchOverview])

    return (
        <>
            <h1 className='text-2xl font-bold mb-6'>Analytics Overview</h1>
            {overview ? (
                <div className='grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4'>
                    <Tile>
                        <h3 className='text-lg'>{days} Days Sales</h3>
                        <p className='text-3xl font-medium mt-4'>Rs. {overview.totalSales / 100}</p>
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

export default AnalyticsOverview
