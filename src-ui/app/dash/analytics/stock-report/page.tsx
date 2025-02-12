'use client'
import React, { useState, useEffect, useCallback } from 'react'
import { Content } from '@carbon/react'
import { invoke } from '@tauri-apps/api/core'
import DataTable from '@/components/ui/DataTable'

interface StockReport {
    id: string
    productName: string
    currentStock: number
    reorderPoint: number
    lastRestockDate: string
    averageDailySales: number
}

const StockReport = () => {
    return (
        <h1>Stock Report</h1>
    )
}

export default StockReport
