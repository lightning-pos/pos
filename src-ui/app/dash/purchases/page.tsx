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
    Tag,
    Button
} from '@carbon/react'
import { ArrowRight } from '@carbon/icons-react'
// Carbon Charts
import '@carbon/charts/styles.css'
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
import { gql } from '@/lib/graphql/execute'
import {
    GetExpensesDocument,
    GetPurchaseCategoriesForExpensesDocument,
    GetCostCentersForExpensesDocument,
    Exact,
    InputMaybe
} from '@/lib/graphql/graphql'
import { formatCurrency } from '@/lib/util/number_format'
import { formatDateForDisplay } from '@/lib/util/date_format'
import Link from 'next/link'

// A helper function to parse date strings from the API
const parseISODate = (dateString: string): Date | null => {
    const date = new Date(dateString);
    return isNaN(date.getTime()) ? null : date;
};

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

// Define the structure for the expense data
interface Expense {
    id: string;
    title: string;
    amount: string;
    expenseDate: string;
    categoryId: string;
    costCenterId: string;
    description?: string | null;
    category: {
        id: string;
        name: string;
    };
    costCenter: {
        id: string;
        name: string;
        code: string;
    };
}

// Define the structure for the category data
interface Category {
    id: string;
    name: string;
}

// Define the structure for the cost center data
interface CostCenter {
    id: string;
    name: string;
    code: string;
    state: string;
}

const PurchasesDashboard = () => {
    // State for expense data
    const [expenses, setExpenses] = useState<Expense[]>([])
    const [categories, setCategories] = useState<Category[]>([])
    const [costCenters, setCostCenters] = useState<CostCenter[]>([])
    const [totalExpenses, setTotalExpenses] = useState(0)
    const [loading, setLoading] = useState(true)

    // State for filters
    const [timeFilter, setTimeFilter] = useState('month')
    const [costCenterFilter, setCostCenterFilter] = useState('all')
    const [selectedCostCenterId, setSelectedCostCenterId] = useState<string | null>(null)
    const [dateRange, setDateRange] = useState<{ startDate: string | null; endDate: string | null }>({
        startDate: null,
        endDate: null
    })

    // State for pagination
    const [page, setPage] = useState(1)
    const [pageSize, setPageSize] = useState(10)

    // Calculate date range based on time filter
    useEffect(() => {
        const now = new Date()
        const endDate = now.toISOString()
        let startDate: string

        switch (timeFilter) {
            case 'month':
                // Last 30 days
                const lastMonth = new Date()
                lastMonth.setDate(now.getDate() - 30)
                startDate = lastMonth.toISOString()
                break
            case 'quarter':
                // Last 90 days
                const lastQuarter = new Date()
                lastQuarter.setDate(now.getDate() - 90)
                startDate = lastQuarter.toISOString()
                break
            case 'year':
                // Last 365 days
                const lastYear = new Date()
                lastYear.setDate(now.getDate() - 365)
                startDate = lastYear.toISOString()
                break
            default:
                // Last 30 days is default
                const defaultLast = new Date()
                defaultLast.setDate(now.getDate() - 30)
                startDate = defaultLast.toISOString()
        }

        setDateRange({ startDate, endDate })
    }, [timeFilter])

    // Update costCenterId when filter changes
    useEffect(() => {
        if (costCenterFilter === 'all') {
            setSelectedCostCenterId(null)
        } else {
            const selectedCostCenter = costCenters.find(cc => cc.name === costCenterFilter)
            setSelectedCostCenterId(selectedCostCenter?.id || null)
        }
    }, [costCenterFilter, costCenters])

    // Fetch expense data with filters
    useEffect(() => {
        const fetchExpenseData = async () => {
            setLoading(true)
            try {
                // Fetch expenses with pagination and filters
                const expenseResult = await gql(GetExpensesDocument, {
                    first: 100,
                    offset: 0,
                    costCenterId: selectedCostCenterId,
                    startDate: dateRange.startDate,
                    endDate: dateRange.endDate
                })
                setExpenses(expenseResult.expenses)
                setTotalExpenses(expenseResult.totalExpenses)

                // Fetch categories
                const categoryResult = await gql(GetPurchaseCategoriesForExpensesDocument)
                setCategories(categoryResult.allPurchaseCategories)

                // Fetch cost centers
                const costCenterResult = await gql(GetCostCentersForExpensesDocument)
                setCostCenters(costCenterResult.allCostCenters)
            } catch (error) {
                console.error('Error fetching expense data:', error)
            } finally {
                setLoading(false)
            }
        }

        fetchExpenseData()
    }, [selectedCostCenterId, dateRange.startDate, dateRange.endDate])

    // Calculate summary metrics
    const totalActualExpense = expenses.reduce((sum, expense) => sum + parseFloat(expense.amount), 0)

    // Get expense by month for chart
    const expensesByMonth = () => {
        const monthData: { [key: string]: number } = {}

        // Initialize with all months
        const months = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec']
        months.forEach(month => {
            monthData[month] = 0
        })

        // Fill with actual data
        expenses.forEach(expense => {
            const date = parseISODate(expense.expenseDate)
            if (date) {
                const month = months[date.getMonth()]
                monthData[month] += parseFloat(expense.amount)
            }
        })

        return monthData
    }

    // Get expenses by category for chart
    const expensesByCategory = () => {
        const categoryData: { [key: string]: number } = {}

        expenses.forEach(expense => {
            const categoryName = expense.category?.name || 'Uncategorized'
            if (!categoryData[categoryName]) {
                categoryData[categoryName] = 0
            }
            categoryData[categoryName] += parseFloat(expense.amount)
        })

        return categoryData
    }

    // Get expenses by cost center for chart
    const expensesByCostCenter = () => {
        const costCenterData: { [key: string]: number } = {}

        expenses.forEach(expense => {
            const costCenterName = expense.costCenter?.name || 'Uncategorized'
            if (!costCenterData[costCenterName]) {
                costCenterData[costCenterName] = 0
            }
            costCenterData[costCenterName] += parseFloat(expense.amount)
        })

        return costCenterData
    }

    // Calculate average expense per cost center
    const avgExpensePerCostCenter = costCenters.length
        ? (totalActualExpense / costCenters.length).toFixed(2)
        : "0.00"

    // Get recent transactions for display
    const recentTransactions = expenses
        .sort((a, b) => new Date(b.expenseDate).getTime() - new Date(a.expenseDate).getTime())
        .slice(0, 5)

    // Create chart data for Carbon Charts

    // Line chart data - Monthly expense trends
    const monthlyData = expensesByMonth()
    const lineChartData: LineChartDataItem[] = Object.keys(monthlyData).map(month => ({
        group: 'Expenses',
        date: month,
        value: monthlyData[month]
    }))

    const lineChartOptions: LineChartOptions = {
        title: 'Monthly Expense Trends',
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
                'Expenses': '#0f62fe'
            }
        },
        curve: 'curveMonotoneX'
    }

    // Donut chart - Categories
    const categoryData = expensesByCategory()
    const donutChartData: DonutChartDataItem[] = Object.keys(categoryData).map(category => ({
        group: category,
        value: categoryData[category]
    }))

    const donutChartOptions: DonutChartOptions = {
        title: 'Expense Categories',
        resizable: true,
        height: '300px',
        donut: {
            center: {
                label: 'Categories'
            },
            alignment: 'center'
        },
        legend: {
            alignment: 'center'
        }
    }

    // Bar chart - Cost Centers
    const costCenterData = expensesByCostCenter()
    const barChartData: BarChartDataItem[] = Object.keys(costCenterData).map(costCenter => ({
        group: 'Expenses',
        key: costCenter,
        value: costCenterData[costCenter]
    }))

    const barChartOptions: BarChartOptions = {
        title: 'Expenses by Cost Center',
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
        height: '300px'
    }

    return (
        <Content className='min-h-[calc(100dvh-3rem)] p-4'>
            <div className="flex justify-between items-center mb-6">
                <h1 className="text-2xl font-bold">Expense Dashboard</h1>
                <Link href="/dash/purchases/expenses">
                    <Button renderIcon={ArrowRight}>Go to Expenses</Button>
                </Link>
            </div>

            {/* Filters */}
            <div className="mb-6 flex gap-4">
                <Dropdown
                    id="time-filter"
                    titleText="Time Period"
                    label="Monthly"
                    items={['month', 'quarter', 'year']}
                    itemToString={(item: string) => {
                        switch (item) {
                            case 'month': return 'Last 30 Days';
                            case 'quarter': return 'Last 90 Days';
                            case 'year': return 'Last 365 Days';
                            default: return item;
                        }
                    }}
                    onChange={(e: { selectedItem: string }) => setTimeFilter(e.selectedItem)}
                />
                <Dropdown
                    id="cost-center-filter"
                    titleText="Cost Center"
                    label="All Cost Centers"
                    items={['all', ...costCenters.map(cc => cc.name)]}
                    itemToString={(item: string) => item === 'all' ? 'All Cost Centers' : item}
                    onChange={(e: { selectedItem: string }) => setCostCenterFilter(e.selectedItem)}
                />
            </div>

            {loading ? (
                <div className="flex justify-center items-center h-64">
                    <p>Loading expense data...</p>
                </div>
            ) : (
                <>
                    {/* Summary Metrics */}
                    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
                        <Tile>
                            <h3 className="text-sm mb-1">Total Expenses</h3>
                            <p className="text-3xl font-medium">{formatCurrency(totalActualExpense)}</p>
                            <p className="text-sm">
                                From {expenses.length} recorded expenses
                            </p>
                        </Tile>
                        <Tile>
                            <h3 className="text-sm mb-1">Avg. per Cost Center</h3>
                            <p className="text-3xl font-medium">{formatCurrency(Number(avgExpensePerCostCenter))}</p>
                            <p className="text-sm">Across {costCenters.length} cost centers</p>
                        </Tile>
                        <Tile>
                            <h3 className="text-sm mb-1">Categories</h3>
                            <p className="text-3xl font-medium">{categories.length}</p>
                            <p className="text-sm">Expense categories available</p>
                        </Tile>
                        <Tile>
                            <h3 className="text-sm mb-1">Recent Activity</h3>
                            <p className="text-3xl font-medium">{recentTransactions.length}</p>
                            <p className="text-sm">New expenses in the last period</p>
                        </Tile>
                    </div>

                    {expenses.length > 0 ? (
                        <>
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
                                        <DonutChart
                                            data={donutChartData}
                                            options={donutChartOptions}
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
                                    <h3 className="text-lg font-semibold mb-4">Recent Transactions</h3>
                                    <Table>
                                        <TableHead>
                                            <TableRow>
                                                <TableHeader>Title</TableHeader>
                                                <TableHeader>Amount</TableHeader>
                                                <TableHeader>Date</TableHeader>
                                                <TableHeader>Category</TableHeader>
                                            </TableRow>
                                        </TableHead>
                                        <TableBody>
                                            {recentTransactions.map(expense => (
                                                <TableRow key={expense.id}>
                                                    <TableCell>{expense.title}</TableCell>
                                                    <TableCell>{formatCurrency(Number(expense.amount))}</TableCell>
                                                    <TableCell>{formatDateForDisplay(expense.expenseDate)}</TableCell>
                                                    <TableCell>{expense.category?.name || 'Uncategorized'}</TableCell>
                                                </TableRow>
                                            ))}
                                        </TableBody>
                                    </Table>
                                </Tile>
                            </div>
                        </>
                    ) : (
                        <Tile className="mb-6 p-6">
                            <h3 className="text-lg font-semibold mb-4">No Expenses Found</h3>
                            <p className="mb-4">You haven't added any expenses yet. Start tracking your expenses to see detailed analytics.</p>
                            <Link href="/dash/purchases/expenses">
                                <Button>Go to Expenses</Button>
                            </Link>
                        </Tile>
                    )}
                </>
            )}
        </Content>
    )
}

export default PurchasesDashboard
