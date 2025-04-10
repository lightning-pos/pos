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
    Button,
    DatePicker,
    DatePickerInput
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
    const date = new Date(dateString)
    return isNaN(date.getTime()) ? null : date
}

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

// A helper function to get the start of the month
const getStartOfMonth = (date: Date): Date => {
    return new Date(date.getFullYear(), date.getMonth(), 1)
}

// A helper function to get the end of the month
const getEndOfMonth = (date: Date): Date => {
    return new Date(date.getFullYear(), date.getMonth() + 1, 0)
}

// Helper function to get start of week (Monday)
const getStartOfWeek = (date: Date): Date => {
    const day = date.getDay()
    const diff = date.getDate() - day + (day === 0 ? -6 : 1) // Adjust when day is Sunday
    return new Date(date.setDate(diff))
}

// Format date for chart display based on grouping
const formatDateForChart = (date: Date, grouping: string): string => {
    if (grouping === 'daily') {
        return `${date.getDate()}/${date.getMonth() + 1}`
    } else if (grouping === 'weekly') {
        const startOfWeek = getStartOfWeek(new Date(date))
        return `W${Math.ceil(startOfWeek.getDate() / 7)}-${startOfWeek.getMonth() + 1}`
    } else {
        // Monthly
        const months = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec']
        return months[date.getMonth()]
    }
}

// Get all days between two dates
const getDaysInRange = (startDate: Date, endDate: Date): Date[] => {
    const days: Date[] = []
    const currentDate = new Date(startDate)
    while (currentDate <= endDate) {
        days.push(new Date(currentDate))
        currentDate.setDate(currentDate.getDate() + 1)
    }
    return days
}

// Get all weeks between two dates
const getWeeksInRange = (startDate: Date, endDate: Date): Date[] => {
    const weeks: Date[] = []
    const currentDate = getStartOfWeek(new Date(startDate))

    while (currentDate <= endDate) {
        weeks.push(new Date(currentDate))
        currentDate.setDate(currentDate.getDate() + 7)
    }
    return weeks
}

// Get all months between two dates
const getMonthsInRange = (startDate: Date, endDate: Date): Date[] => {
    const months: Date[] = []
    const currentDate = new Date(startDate.getFullYear(), startDate.getMonth(), 1)

    while (currentDate <= endDate) {
        months.push(new Date(currentDate))
        currentDate.setMonth(currentDate.getMonth() + 1)
    }
    return months
}

const PurchasesDashboard = () => {
    // State for expense data
    const [expenses, setExpenses] = useState<Expense[]>([])
    const [categories, setCategories] = useState<Category[]>([])
    const [costCenters, setCostCenters] = useState<CostCenter[]>([])
    const [totalExpenses, setTotalExpenses] = useState(0)
    const [loading, setLoading] = useState(true)

    // State for filters
    const [timeFilter, setTimeFilter] = useState('this-month')
    const [groupBy, setGroupBy] = useState('monthly')
    const [costCenterFilter, setCostCenterFilter] = useState('all')
    const [selectedCostCenterId, setSelectedCostCenterId] = useState<string | null>(null)
    const [dateRange, setDateRange] = useState<{ startDate: string | null; endDate: string | null }>({
        startDate: null,
        endDate: null
    })
    const [customDateRange, setCustomDateRange] = useState<{ start: Date | null; end: Date | null }>({
        start: null,
        end: null
    })
    const [showCustomDatePicker, setShowCustomDatePicker] = useState(false)

    // State for pagination
    const [page, setPage] = useState(1)
    const [pageSize, setPageSize] = useState(10)

    // Calculate date range based on time filter
    useEffect(() => {
        const now = new Date()
        let startDate: Date
        let endDate: Date

        switch (timeFilter) {
        case 'this-month':
            // Current month (1st of month to today)
            startDate = getStartOfMonth(now)
            endDate = now
            break
        case 'last-month':
            // Last month (1st to last day of previous month)
            const lastMonth = new Date(now.getFullYear(), now.getMonth() - 1, 1)
            startDate = getStartOfMonth(lastMonth)
            endDate = getEndOfMonth(lastMonth)
            break
        case 'last-3-months':
            // Last 3 months (from 3 months ago to today)
            const threeMonthsAgo = new Date(now)
            threeMonthsAgo.setMonth(now.getMonth() - 2)
            startDate = getStartOfMonth(threeMonthsAgo)
            endDate = now
            break
        case 'last-12-months':
            // Last 12 months (from 12 months ago to today)
            const twelveMonthsAgo = new Date(now)
            twelveMonthsAgo.setMonth(now.getMonth() - 11)
            startDate = getStartOfMonth(twelveMonthsAgo)
            endDate = now
            break
        case 'custom':
            // Custom date range
            setShowCustomDatePicker(true)
            if (customDateRange.start && customDateRange.end) {
                startDate = customDateRange.start
                endDate = customDateRange.end
            } else {
                // Default to this month if custom range is not set
                startDate = getStartOfMonth(now)
                endDate = now
            }
            break
        default:
            // This month is default
            startDate = getStartOfMonth(now)
            endDate = now
        }

        if (timeFilter !== 'custom' || (customDateRange.start && customDateRange.end)) {
            setDateRange({
                startDate: startDate.toISOString(),
                endDate: endDate.toISOString()
            })
            setShowCustomDatePicker(timeFilter === 'custom')
        }
    }, [timeFilter, customDateRange])

    // Handle custom date range changes
    const handleCustomDateChange = (dates: Date[]) => {
        if (dates.length === 2) {
            setCustomDateRange({
                start: dates[0],
                end: dates[1]
            })
        }
    }

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

        // Only fetch if we have valid date range
        if (dateRange.startDate && dateRange.endDate) {
            fetchExpenseData()
        }
    }, [selectedCostCenterId, dateRange.startDate, dateRange.endDate])

    // Calculate summary metrics - using calculated sum from expenses array
    const totalActualExpense = expenses.reduce((sum, expense) => sum + parseFloat(expense.amount), 0)

    // Calculate average expense per cost center
    const avgExpensePerCostCenter = costCenters.length
        ? (totalActualExpense / costCenters.length).toFixed(2)
        : '0.00'

    // Get expense data grouped by selected option
    const getGroupedExpenseData = () => {
        const groupedData: { [key: string]: number } = {}

        if (!dateRange.startDate || !dateRange.endDate) {
            return groupedData
        }

        const startDate = new Date(dateRange.startDate)
        const endDate = new Date(dateRange.endDate)

        // Initialize all possible dates/weeks/months in the range with zero values
        if (groupBy === 'daily') {
            getDaysInRange(startDate, endDate).forEach(date => {
                const key = formatDateForChart(date, groupBy)
                groupedData[key] = 0
            })
        } else if (groupBy === 'weekly') {
            getWeeksInRange(startDate, endDate).forEach(date => {
                const key = formatDateForChart(date, groupBy)
                groupedData[key] = 0
            })
        } else { // monthly
            getMonthsInRange(startDate, endDate).forEach(date => {
                const key = formatDateForChart(date, groupBy)
                groupedData[key] = 0
            })
        }

        // Fill with actual data
        expenses.forEach(expense => {
            const date = parseISODate(expense.expenseDate)
            if (date) {
                const key = formatDateForChart(date, groupBy)
                if (key in groupedData) {
                    groupedData[key] += parseFloat(expense.amount)
                }
            }
        })

        return groupedData
    }

    // Replace expensesByMonth with getGroupedExpenseData
    const monthlyData = getGroupedExpenseData()
    const lineChartData: LineChartDataItem[] = Object.keys(monthlyData)
        .sort((a, b) => {
            if (groupBy === 'monthly') {
                const months = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec']
                return months.indexOf(a) - months.indexOf(b)
            } else if (groupBy === 'weekly') {
                // Extract week number and month for sorting
                const [weekA, monthA] = a.substring(1).split('-').map(Number)
                const [weekB, monthB] = b.substring(1).split('-').map(Number)
                return monthA === monthB ? weekA - weekB : monthA - monthB
            } else {
                // For daily, convert DD/MM to sortable format
                const [dayA, monthA] = a.split('/').map(Number)
                const [dayB, monthB] = b.split('/').map(Number)
                return monthA === monthB ? dayA - dayB : monthA - monthB
            }
        })
        .map(period => ({
            group: 'Expenses',
            date: period,
            value: monthlyData[period]
        }))

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

    // Get recent transactions for display
    const recentTransactions = expenses
        .sort((a, b) => new Date(b.expenseDate).getTime() - new Date(a.expenseDate).getTime())
        .slice(0, 5)

    // Create chart data for Carbon Charts

    // Line chart options
    const lineChartOptions: LineChartOptions = {
        title: 'Expense Trends',
        axes: {
            bottom: {
                title: groupBy === 'daily' ? 'Day' : groupBy === 'weekly' ? 'Week' : 'Month',
                mapsTo: 'date',
                scaleType: ScaleTypes.LABELS
            },
            left: {
                title: 'Amount (₹)',
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
                title: 'Amount (₹)',
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
                <h1 className="text-2xl font-bold">Purchase / Expense - Dashboard</h1>
            </div>

            {/* Filters */}
            <div className="mb-6 flex flex-wrap gap-4">
                <Dropdown
                    id="time-filter"
                    titleText="Time Period"
                    label="This Month"
                    items={['this-month', 'last-month', 'last-3-months', 'last-12-months', 'custom']}
                    itemToString={(item: string) => {
                        switch (item) {
                        case 'this-month': return 'This Month'
                        case 'last-month': return 'Last Month'
                        case 'last-3-months': return 'Last 3 Months'
                        case 'last-12-months': return 'Last 12 Months'
                        case 'custom': return 'Custom Range'
                        default: return item
                        }
                    }}
                    onChange={(e: { selectedItem: string }) => setTimeFilter(e.selectedItem)}
                />
                <Dropdown
                    id="group-by"
                    titleText="Group By"
                    label="Monthly"
                    items={['daily', 'weekly', 'monthly']}
                    itemToString={(item: string) => {
                        switch (item) {
                        case 'daily': return 'Daily'
                        case 'weekly': return 'Weekly'
                        case 'monthly': return 'Monthly'
                        default: return item
                        }
                    }}
                    onChange={(e: { selectedItem: string }) => setGroupBy(e.selectedItem)}
                />
                <Dropdown
                    id="cost-center-filter"
                    titleText="Cost Center"
                    label="All Cost Centers"
                    items={['all', ...costCenters.map(cc => cc.name)]}
                    itemToString={(item: string) => item === 'all' ? 'All Cost Centers' : item}
                    onChange={(e: { selectedItem: string }) => setCostCenterFilter(e.selectedItem)}
                />

                {showCustomDatePicker && (
                    <DatePicker
                        datePickerType="range"
                        dateFormat="d/m/Y"
                        onChange={handleCustomDateChange}
                        className="ml-4"
                    >
                        <DatePickerInput
                            id="date-picker-input-start"
                            placeholder="dd/mm/yyyy"
                            labelText="Start Date"
                            size="md"
                        />
                        <DatePickerInput
                            id="date-picker-input-end"
                            placeholder="dd/mm/yyyy"
                            labelText="End Date"
                            size="md"
                        />
                    </DatePicker>
                )}
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
