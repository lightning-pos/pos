'use client'
import React, { useState, useEffect, useCallback } from 'react'
import { Content } from '@carbon/react'
import { invoke } from '@tauri-apps/api/core'
import DataTable from '@/components/ui/DataTable'

interface SalesReport {
    id: string
    date: string
    totalAmount: number
    itemsSold: number
    averageOrderValue: number
}

const SalesReport = () => {
    return (
        <h1>Sales Report</h1>
    )
}

export default SalesReport
