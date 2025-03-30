'use client'
import { useState, useEffect } from 'react'
import {
    Content,
    Dropdown,
    Tile,
    DataTable,
    Table,
    TableHead,
    TableRow,
    TableHeader,
    TableBody,
    TableCell,
    Pagination,
    Tag
} from '@carbon/react'

import {
    LineChart,
    GroupedBarChart,
    DonutChart,
    StackedBarChart,
    GaugeChart
} from '@carbon/charts-react'
import {
    LineChartOptions,
    BarChartOptions,
    DonutChartOptions,
    StackedBarChartOptions,
    GaugeChartOptions,
    ScaleTypes
} from '@carbon/charts/interfaces'

// Mock Data
const MONTHS = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec']

const mockExpensesByMonth = MONTHS.map((month, i) => ({
    month,
    actual: Math.floor(10000 + Math.random() * 15000),
    budget: Math.floor(15000 + Math.random() * 5000),
}))

const mockCostCenters = [
    { id: '1', name: 'Marketing', actual: 45000, budget: 50000 },
    { id: '2', name: 'Operations', actual: 72000, budget: 65000 },
    { id: '3', name: 'Sales', actual: 38000, budget: 40000 },
    { id: '4', name: 'R&D', actual: 55000, budget: 60000 },
    { id: '5', name: 'Admin', actual: 25000, budget: 30000 },
]

const mockCategories = [
    { id: '1', name: 'Payroll', actual: 80000, budget: 85000 },
    { id: '2', name: 'Travel', actual: 25000, budget: 20000 },
    { id: '3', name: 'Software', actual: 35000, budget: 30000 },
    { id: '4', name: 'Office Supplies', actual: 15000, budget: 18000 },
    { id: '5', name: 'Marketing', actual: 30000, budget: 35000 },
]

const mockVendors = [
    { id: '1', name: 'Acme Corp', amount: 45000, transactions: 23 },
    { id: '2', name: 'Tech Solutions Inc', amount: 38000, transactions: 12 },
    { id: '3', name: 'Office Depot', amount: 15000, transactions: 45 },
    { id: '4', name: 'Travel Agency XYZ', amount: 22000, transactions: 18 },
    { id: '5', name: 'Marketing Partners', amount: 19000, transactions: 8 },
]

const mockTransactions = Array(50).fill(0).map((_, i) => ({
    id: `${i + 1}`,
    date: new Date(2023, Math.floor(Math.random() * 12), Math.floor(Math.random() * 28) + 1),
    vendor: mockVendors[Math.floor(Math.random() * mockVendors.length)].name,
    category: mockCategories[Math.floor(Math.random() * mockCategories.length)].name,
    costCenter: mockCostCenters[Math.floor(Math.random() * mockCostCenters.length)].name,
    amount: Math.floor(500 + Math.random() * 5000),
    description: `Transaction #${i + 1} description`
}))

// Calculate summary metrics
const totalActualExpense = mockCostCenters.reduce((sum, cc) => sum + cc.actual, 0)
const totalBudgetExpense = mockCostCenters.reduce((sum, cc) => sum + cc.budget, 0)
const budgetUtilizationRate = (totalActualExpense / totalBudgetExpense * 100).toFixed(1)
const previousPeriodExpense = totalActualExpense * 0.9 // Mock 10% growth
const growthRateValue = ((totalActualExpense - previousPeriodExpense) / previousPeriodExpense * 100)
const growthRate = growthRateValue.toFixed(1)

// Type definitions for chart data
interface LineChartDataItem {
    group: string;
    date: string;
    value: number;
}

interface BarChartDataItem {
    group: string;
    key: string;
    value: number;
}

interface DonutChartDataItem {
    group: string;
    value: number;
}

interface GaugeChartDataItem {
    group: string;
    value: number;
}

const PurchasesDashboard = () => {
    const [timeFilter, setTimeFilter] = useState('month')
    const [costCenterFilter, setCostCenterFilter] = useState('all')
    const [page, setPage] = useState(1)
    const [pageSize, setPageSize] = useState(10)

    // Prepare chart data for Carbon Charts

    // Line chart data - Expense trends
    const lineChartData: LineChartDataItem[] = [];
    mockExpensesByMonth.forEach(item => {
        lineChartData.push({
            group: 'Actual',
            date: item.month,
            value: item.actual
        });
        lineChartData.push({
            group: 'Budget',
            date: item.month,
            value: item.budget
        });
    });

    const lineChartOptions: LineChartOptions = {
        title: 'Expense Trends Over Time',
        axes: {
            bottom: {
                title: 'Month',
                mapsTo: 'date',
                scaleType: ScaleTypes.LABELS
            },
            left: {
                title: 'Amount ($)',
                mapsTo: 'value',
                scaleType: ScaleTypes.LINEAR
            }
        },
        height: '300px',
        color: {
            scale: {
                'Actual': '#0f62fe',
                'Budget': '#da1e28'
            }
        },
        curve: 'curveMonotoneX'
    };

    // Grouped bar chart - Cost Centers
    const barChartData: BarChartDataItem[] = [];
    mockCostCenters.forEach(item => {
        barChartData.push({
            group: 'Actual',
            key: item.name,
            value: item.actual
        });
        barChartData.push({
            group: 'Budget',
            key: item.name,
            value: item.budget
        });
    });

    const barChartOptions: BarChartOptions = {
        title: 'Cost Center Expenses vs Budget',
        axes: {
            left: {
                title: 'Amount ($)',
                mapsTo: 'value',
                scaleType: ScaleTypes.LINEAR
            },
            bottom: {
                title: 'Cost Center',
                mapsTo: 'key',
                scaleType: ScaleTypes.LABELS
            }
        },
        height: '300px',
        color: {
            scale: {
                'Actual': '#0f62fe',
                'Budget': '#da1e28'
            }
        }
    };

    // Donut chart - Categories
    const donutChartData: DonutChartDataItem[] = mockCategories.map(item => ({
        group: item.name,
        value: item.actual
    }));

    const donutChartOptions: DonutChartOptions = {
        title: 'Expense Categories',
        resizable: true,
        height: '300px',
        donut: {
            center: {
                label: 'Categories'
            },
            alignment: 'center'
        }
    };

    // Stacked bar chart - Vendors
    const stackedBarData: DonutChartDataItem[] = mockVendors.map(item => ({
        group: item.name,
        value: item.amount
    }));

    const stackedBarOptions: StackedBarChartOptions = {
        title: 'Top Vendors by Spend',
        axes: {
            left: {
                title: 'Amount ($)',
                mapsTo: 'value',
                scaleType: ScaleTypes.LINEAR
            },
            bottom: {
                title: 'Vendor',
                mapsTo: 'group',
                scaleType: ScaleTypes.LABELS
            }
        },
        height: '300px'
    };

    // Gauge chart - Budget utilization
    const gaugeChartData: GaugeChartDataItem[] = [
        {
            group: 'Budget Utilization',
            value: parseFloat(budgetUtilizationRate)
        }
    ];

    const gaugeChartOptions: GaugeChartOptions = {
        title: 'Budget Utilization Rate',
        height: '250px',
        resizable: true,
        gauge: {
            type: 'semi',
            status: {
                ranges: [
                    {
                        range: [0, 70],
                        status: 'success'
                    },
                    {
                        range: [70, 90],
                        status: 'warning'
                    },
                    {
                        range: [90, 100],
                        status: 'danger'
                    }
                ]
            } as any // Type cast to any to work around type mismatch
        }
    };

    // Calculate metrics for display
    const overBudgetCostCenters = mockCostCenters.filter(cc => cc.actual > cc.budget)
    const avgExpensePerCostCenter = (totalActualExpense / mockCostCenters.length).toFixed(2)

    // Transaction pagination
    const paginatedTransactions = mockTransactions
        .sort((a, b) => b.date.getTime() - a.date.getTime())
        .slice((page - 1) * pageSize, page * pageSize)

    return (
        <Content className='min-h-[calc(100dvh-3rem)] p-4'>
            <h1 className="text-2xl font-bold mb-6">Expense Dashboard</h1>

            {/* Filters */}
            <div className="mb-6 flex gap-4">
                <Dropdown
                    id="time-filter"
                    titleText="Time Period"
                    label="Monthly"
                    items={['Monthly', 'Quarterly', 'Yearly']}
                    onChange={(e: { selectedItem: string }) => setTimeFilter(e.selectedItem.toLowerCase())}
                />
                <Dropdown
                    id="cost-center-filter"
                    titleText="Cost Center"
                    label="All Cost Centers"
                    items={['All Cost Centers', ...mockCostCenters.map(cc => cc.name)]}
                    onChange={(e: { selectedItem: string }) => setCostCenterFilter(e.selectedItem === 'All Cost Centers' ? 'all' : e.selectedItem)}
                />
            </div>

            {/* Summary Metrics */}
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
                <Tile>
                    <h3 className="text-sm mb-1">Total Expenses</h3>
                    <p className="text-3xl font-medium">${totalActualExpense.toLocaleString()}</p>
                    <p className={`text-sm ${parseFloat(growthRate) > 0 ? 'text-red-600' : 'text-green-600'}`}>
                        {parseFloat(growthRate) > 0 ? '↑' : '↓'} {Math.abs(parseFloat(growthRate))}% vs prev period
                    </p>
                </Tile>
                <Tile>
                    <h3 className="text-sm mb-1">Budget Utilization</h3>
                    <p className="text-3xl font-medium">{budgetUtilizationRate}%</p>
                    <p className="text-sm">
                        ${totalActualExpense.toLocaleString()} of ${totalBudgetExpense.toLocaleString()}
                    </p>
                </Tile>
                <Tile>
                    <h3 className="text-sm mb-1">Avg. per Cost Center</h3>
                    <p className="text-3xl font-medium">${Number(avgExpensePerCostCenter).toLocaleString()}</p>
                    <p className="text-sm">Across {mockCostCenters.length} cost centers</p>
                </Tile>
                <Tile>
                    <h3 className="text-sm mb-1">Over Budget Centers</h3>
                    <p className="text-3xl font-medium">{overBudgetCostCenters.length}</p>
                    <p className="text-sm text-red-600">
                        {overBudgetCostCenters.length > 0 ? 'Action required' : 'All within budget'}
                    </p>
                </Tile>
            </div>

            {/* Charts Row 1 */}
            <div className="grid grid-cols-1 lg:grid-cols-3 gap-4 mb-6">
                <div className="lg:col-span-2">
                    <Tile className="h-full">
                        <LineChart
                            data={lineChartData}
                            options={lineChartOptions}
                        />
                    </Tile>
                </div>
                <div className="lg:col-span-1">
                    <Tile className="h-full">
                        <GaugeChart
                            data={gaugeChartData}
                            options={gaugeChartOptions}
                        />
                    </Tile>
                </div>
            </div>

            {/* Charts Row 2 */}
            <div className="grid grid-cols-1 lg:grid-cols-2 gap-4 mb-6">
                <Tile>
                    <GroupedBarChart
                        data={barChartData}
                        options={barChartOptions}
                    />
                </Tile>
                <Tile>
                    <DonutChart
                        data={donutChartData}
                        options={donutChartOptions}
                    />
                </Tile>
            </div>

            {/* Vendors Chart */}
            <div className="mb-6">
                <Tile>
                    <StackedBarChart
                        data={stackedBarData}
                        options={stackedBarOptions}
                    />
                </Tile>
            </div>

            {/* Transactions Table */}
            <Tile className="mb-6">
                <h3 className="text-lg font-semibold mb-4">Recent Transactions</h3>
                <DataTable rows={paginatedTransactions.map(t => ({
                    ...t,
                    date: t.date.toLocaleDateString(),
                    amount: `$${t.amount.toLocaleString()}`
                }))} headers={[
                    { header: 'Date', key: 'date' },
                    { header: 'Vendor', key: 'vendor' },
                    { header: 'Category', key: 'category' },
                    { header: 'Cost Center', key: 'costCenter' },
                    { header: 'Amount', key: 'amount' },
                    { header: 'Description', key: 'description' },
                ]}>
                    {({ rows, headers, getHeaderProps, getRowProps, getTableProps }: any) => (
                        <Table {...getTableProps()}>
                            <TableHead>
                                <TableRow>
                                    {headers.map((header: any) => (
                                        <TableHeader key={header.key} {...getHeaderProps({ header })}>
                                            {header.header}
                                        </TableHeader>
                                    ))}
                                </TableRow>
                            </TableHead>
                            <TableBody>
                                {rows.map((row: any) => (
                                    <TableRow key={row.id} {...getRowProps({ row })}>
                                        {row.cells.map((cell: any) => (
                                            <TableCell key={cell.id}>{cell.value}</TableCell>
                                        ))}
                                    </TableRow>
                                ))}
                            </TableBody>
                        </Table>
                    )}
                </DataTable>
                <div className="mt-4">
                    <Pagination
                        totalItems={mockTransactions.length}
                        backwardText="Previous page"
                        forwardText="Next page"
                        pageSize={pageSize}
                        pageSizes={[10, 20, 30, 40, 50]}
                        itemsPerPageText="Items per page:"
                        onChange={({ page, pageSize }: { page: number, pageSize: number }) => {
                            setPage(page);
                            setPageSize(pageSize);
                        }}
                    />
                </div>
            </Tile>

            {/* Anomalies and Alerts */}
            <Tile className="mb-6">
                <h3 className="text-lg font-semibold mb-4">Alerts & Anomalies</h3>
                <ul className="space-y-3">
                    {overBudgetCostCenters.map(cc => (
                        <li key={cc.id} className="flex items-center">
                            <Tag type="red" size="sm">Over Budget</Tag>
                            <span className="ml-2">
                                {cc.name} cost center is ${(cc.actual - cc.budget).toLocaleString()}
                                ({((cc.actual - cc.budget) / cc.budget * 100).toFixed(1)}%) over budget
                            </span>
                        </li>
                    ))}
                    {mockCategories.filter(c => c.actual > c.budget).map(c => (
                        <li key={c.id} className="flex items-center">
                            <Tag type="red" size="sm">Over Budget</Tag>
                            <span className="ml-2">
                                {c.name} category is ${(c.actual - c.budget).toLocaleString()}
                                ({((c.actual - c.budget) / c.budget * 100).toFixed(1)}%) over budget
                            </span>
                        </li>
                    ))}
                    <li className="flex items-center">
                        <Tag type="blue" size="sm">Insight</Tag>
                        <span className="ml-2">
                            Travel expenses increased by 25% compared to previous quarter
                        </span>
                    </li>
                </ul>
            </Tile>

            {/* Recommendations */}
            <Tile>
                <h3 className="text-lg font-semibold mb-4">Recommendations</h3>
                <ul className="space-y-3">
                    <li className="flex items-center">
                        <Tag type="green" size="sm">Cost Saving</Tag>
                        <span className="ml-2">
                            Consolidate software subscriptions to reduce redundancy - potential savings of $5,000/year
                        </span>
                    </li>
                    <li className="flex items-center">
                        <Tag type="green" size="sm">Cost Saving</Tag>
                        <span className="ml-2">
                            Renegotiate vendor contracts with Acme Corp - potential savings of $8,000/year
                        </span>
                    </li>
                    <li className="flex items-center">
                        <Tag type="purple" size="sm">Efficiency</Tag>
                        <span className="ml-2">
                            Implement digital receipts to reduce processing time and errors
                        </span>
                    </li>
                </ul>
            </Tile>
        </Content>
    )
}

export default PurchasesDashboard
