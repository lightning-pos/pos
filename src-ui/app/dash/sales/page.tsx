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

// Define the structure for the sales order data
interface SalesOrder {
    id: string;
    customerName: string;
    customerPhoneNumber: string;
    totalAmount: string;
    orderDate: string;
    state: string;
    paymentMethod: string;
}

// Define the structure for the product category data
interface ProductCategory {
    id: string;
    name: string;
}

// Define the structure for the customer data
interface Customer {
    id: string;
    name: string;
    phoneNumber: string;
    totalOrders: number;
    totalSpent: number;
}

// A helper function to get the start of the month
const getStartOfMonth = (date: Date): Date => {
    return new Date(date.getFullYear(), date.getMonth(), 1);
};

// A helper function to get the end of the month
const getEndOfMonth = (date: Date): Date => {
    return new Date(date.getFullYear(), date.getMonth() + 1, 0);
};

// Helper function to get start of week (Monday)
const getStartOfWeek = (date: Date): Date => {
    const day = date.getDay();
    const diff = date.getDate() - day + (day === 0 ? -6 : 1); // Adjust when day is Sunday
    return new Date(date.setDate(diff));
};

// Format date for chart display based on grouping
const formatDateForChart = (date: Date, grouping: string): string => {
    if (grouping === 'daily') {
        return `${date.getDate()}/${date.getMonth() + 1}`;
    } else if (grouping === 'weekly') {
        const startOfWeek = getStartOfWeek(new Date(date));
        return `W${Math.ceil(startOfWeek.getDate() / 7)}-${startOfWeek.getMonth() + 1}`;
    } else {
        // Monthly
        const months = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'];
        return months[date.getMonth()];
    }
};

// Get all days between two dates
const getDaysInRange = (startDate: Date, endDate: Date): Date[] => {
    const days: Date[] = [];
    const currentDate = new Date(startDate);
    while (currentDate <= endDate) {
        days.push(new Date(currentDate));
        currentDate.setDate(currentDate.getDate() + 1);
    }
    return days;
};

// Get all weeks between two dates
const getWeeksInRange = (startDate: Date, endDate: Date): Date[] => {
    const weeks: Date[] = [];
    const currentDate = getStartOfWeek(new Date(startDate));

    while (currentDate <= endDate) {
        weeks.push(new Date(currentDate));
        currentDate.setDate(currentDate.getDate() + 7);
    }
    return weeks;
};

// Get all months between two dates
const getMonthsInRange = (startDate: Date, endDate: Date): Date[] => {
    const months: Date[] = [];
    const currentDate = new Date(startDate.getFullYear(), startDate.getMonth(), 1);

    while (currentDate <= endDate) {
        months.push(new Date(currentDate));
        currentDate.setMonth(currentDate.getMonth() + 1);
    }
    return months;
};

// Mock data generator functions
const generateMockSalesOrders = (count: number): SalesOrder[] => {
    const states = ['Completed', 'Pending', 'Cancelled'];
    const paymentMethods = ['Cash', 'Card', 'UPI', 'Bank Transfer'];
    const customers = [
        { name: 'John Doe', phone: '+91 9876543210' },
        { name: 'Jane Smith', phone: '+91 9876543211' },
        { name: 'Alice Johnson', phone: '+91 9876543212' },
        { name: 'Bob Williams', phone: '+91 9876543213' },
        { name: 'Charlie Brown', phone: '+91 9876543214' }
    ];

    return Array.from({ length: count }).map((_, index) => {
        const orderDate = new Date();
        orderDate.setDate(orderDate.getDate() - Math.floor(Math.random() * 90)); // Random date within last 90 days

        const customer = customers[Math.floor(Math.random() * customers.length)];
        const amount = (Math.random() * 5000 + 500).toFixed(2);

        return {
            id: `ORD${1000 + index}`,
            customerName: customer.name,
            customerPhoneNumber: customer.phone,
            totalAmount: amount,
            orderDate: orderDate.toISOString(),
            state: states[Math.floor(Math.random() * states.length)],
            paymentMethod: paymentMethods[Math.floor(Math.random() * paymentMethods.length)]
        };
    });
};

const generateMockProductCategories = (): ProductCategory[] => {
    const categories = [
        'Electronics', 'Clothing', 'Groceries', 'Home Appliances',
        'Beauty Products', 'Toys', 'Books', 'Sports Equipment'
    ];

    return categories.map((name, index) => ({
        id: `CAT${100 + index}`,
        name
    }));
};

const generateMockCustomers = (count: number): Customer[] => {
    const names = [
        'John Doe', 'Jane Smith', 'Alice Johnson', 'Bob Williams',
        'Charlie Brown', 'David Lee', 'Emily Davis', 'Frank Miller',
        'Grace Wilson', 'Henry Taylor', 'Isabella Moore', 'Jack Anderson'
    ];

    return Array.from({ length: count }).map((_, index) => {
        const totalOrders = Math.floor(Math.random() * 15) + 1;
        const totalSpent = (Math.random() * 20000 + 1000).toFixed(2);

        return {
            id: `CUST${100 + index}`,
            name: names[index % names.length],
            phoneNumber: `+91 98765432${index < 10 ? '0' + index : index}`,
            totalOrders,
            totalSpent: parseFloat(totalSpent)
        };
    });
};

const SalesDashboard = () => {
    // State for mock sales data
    const [salesOrders, setSalesOrders] = useState<SalesOrder[]>([]);
    const [categories, setCategories] = useState<ProductCategory[]>([]);
    const [customers, setCustomers] = useState<Customer[]>([]);
    const [totalSales, setTotalSales] = useState(0);
    const [loading, setLoading] = useState(true);

    // State for filters
    const [timeFilter, setTimeFilter] = useState('this-month');
    const [groupBy, setGroupBy] = useState('monthly');
    const [paymentMethodFilter, setPaymentMethodFilter] = useState('all');
    const [dateRange, setDateRange] = useState<{ startDate: string | null; endDate: string | null }>({
        startDate: null,
        endDate: null
    });
    const [customDateRange, setCustomDateRange] = useState<{ start: Date | null; end: Date | null }>({
        start: null,
        end: null
    });
    const [showCustomDatePicker, setShowCustomDatePicker] = useState(false);

    // Load mock data
    useEffect(() => {
        setLoading(true);

        // Generate mock data
        const mockOrders = generateMockSalesOrders(100);
        const mockCategories = generateMockProductCategories();
        const mockCustomers = generateMockCustomers(10);

        // Calculate total sales
        const total = mockOrders.reduce((sum, order) => sum + parseFloat(order.totalAmount), 0);

        setSalesOrders(mockOrders);
        setCategories(mockCategories);
        setCustomers(mockCustomers);
        setTotalSales(total);
        setLoading(false);
    }, []);

    // Calculate date range based on time filter
    useEffect(() => {
        const now = new Date();
        let startDate: Date;
        let endDate: Date;

        switch (timeFilter) {
            case 'this-month':
                startDate = getStartOfMonth(now);
                endDate = now;
                break;
            case 'last-month':
                const lastMonth = new Date(now.getFullYear(), now.getMonth() - 1, 1);
                startDate = getStartOfMonth(lastMonth);
                endDate = getEndOfMonth(lastMonth);
                break;
            case 'last-3-months':
                const threeMonthsAgo = new Date(now);
                threeMonthsAgo.setMonth(now.getMonth() - 2);
                startDate = getStartOfMonth(threeMonthsAgo);
                endDate = now;
                break;
            case 'last-12-months':
                const twelveMonthsAgo = new Date(now);
                twelveMonthsAgo.setMonth(now.getMonth() - 11);
                startDate = getStartOfMonth(twelveMonthsAgo);
                endDate = now;
                break;
            case 'custom':
                setShowCustomDatePicker(true);
                if (customDateRange.start && customDateRange.end) {
                    startDate = customDateRange.start;
                    endDate = customDateRange.end;
                } else {
                    startDate = getStartOfMonth(now);
                    endDate = now;
                }
                break;
            default:
                startDate = getStartOfMonth(now);
                endDate = now;
        }

        if (timeFilter !== 'custom' || (customDateRange.start && customDateRange.end)) {
            setDateRange({
                startDate: startDate.toISOString(),
                endDate: endDate.toISOString()
            });
            setShowCustomDatePicker(timeFilter === 'custom');
        }
    }, [timeFilter, customDateRange]);

    // Handle custom date range changes
    const handleCustomDateChange = (dates: Date[]) => {
        if (dates.length === 2) {
            setCustomDateRange({
                start: dates[0],
                end: dates[1]
            });
        }
    };

    // Filter orders based on date range and payment method
    const filteredOrders = salesOrders.filter(order => {
        const orderDate = new Date(order.orderDate);
        const startDate = dateRange.startDate ? new Date(dateRange.startDate) : null;
        const endDate = dateRange.endDate ? new Date(dateRange.endDate) : null;

        const isInDateRange = (!startDate || orderDate >= startDate) &&
            (!endDate || orderDate <= endDate);

        const matchesPaymentMethod = paymentMethodFilter === 'all' ||
            order.paymentMethod === paymentMethodFilter;

        return isInDateRange && matchesPaymentMethod;
    });

    // Calculate total filtered sales
    const totalFilteredSales = filteredOrders.reduce((sum, order) =>
        sum + parseFloat(order.totalAmount), 0);

    // Get average order value
    const avgOrderValue = filteredOrders.length > 0 ?
        totalFilteredSales / filteredOrders.length : 0;

    // Get sales data grouped by selected option
    const getGroupedSalesData = () => {
        const groupedData: { [key: string]: number } = {};

        if (!dateRange.startDate || !dateRange.endDate) {
            return groupedData;
        }

        const startDate = new Date(dateRange.startDate);
        const endDate = new Date(dateRange.endDate);

        // Initialize all possible dates/weeks/months in the range with zero values
        if (groupBy === 'daily') {
            getDaysInRange(startDate, endDate).forEach(date => {
                const key = formatDateForChart(date, groupBy);
                groupedData[key] = 0;
            });
        } else if (groupBy === 'weekly') {
            getWeeksInRange(startDate, endDate).forEach(date => {
                const key = formatDateForChart(date, groupBy);
                groupedData[key] = 0;
            });
        } else { // monthly
            getMonthsInRange(startDate, endDate).forEach(date => {
                const key = formatDateForChart(date, groupBy);
                groupedData[key] = 0;
            });
        }

        // Fill with actual data
        filteredOrders.forEach(order => {
            const date = parseISODate(order.orderDate);
            if (date) {
                const key = formatDateForChart(date, groupBy);
                if (key in groupedData) {
                    groupedData[key] += parseFloat(order.totalAmount);
                }
            }
        });

        return groupedData;
    };

    // Get sales by payment method for chart
    const salesByPaymentMethod = () => {
        const paymentData: { [key: string]: number } = {};

        filteredOrders.forEach(order => {
            if (!paymentData[order.paymentMethod]) {
                paymentData[order.paymentMethod] = 0;
            }
            paymentData[order.paymentMethod] += parseFloat(order.totalAmount);
        });

        return paymentData;
    };

    // Get top customers by total spent
    const topCustomers = () => {
        const customerSpending: { [key: string]: number } = {};

        filteredOrders.forEach(order => {
            const customerKey = `${order.customerName} (${order.customerPhoneNumber})`;
            if (!customerSpending[customerKey]) {
                customerSpending[customerKey] = 0;
            }
            customerSpending[customerKey] += parseFloat(order.totalAmount);
        });

        return Object.entries(customerSpending)
            .map(([customer, total]) => ({ customer, total }))
            .sort((a, b) => b.total - a.total)
            .slice(0, 5);
    };

    // Create chart data for Carbon Charts
    const monthlyData = getGroupedSalesData();
    const lineChartData: LineChartDataItem[] = Object.keys(monthlyData)
        .sort((a, b) => {
            if (groupBy === 'monthly') {
                const months = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'];
                return months.indexOf(a) - months.indexOf(b);
            } else if (groupBy === 'weekly') {
                // Extract week number and month for sorting
                const [weekA, monthA] = a.substring(1).split('-').map(Number);
                const [weekB, monthB] = b.substring(1).split('-').map(Number);
                return monthA === monthB ? weekA - weekB : monthA - monthB;
            } else {
                // For daily, convert DD/MM to sortable format
                const [dayA, monthA] = a.split('/').map(Number);
                const [dayB, monthB] = b.split('/').map(Number);
                return monthA === monthB ? dayA - dayB : monthA - monthB;
            }
        })
        .map(period => ({
            group: 'Sales',
            date: period,
            value: monthlyData[period]
        }));

    // Line chart options
    const lineChartOptions: LineChartOptions = {
        title: 'Sales Trends',
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
                'Sales': '#0f62fe'
            }
        },
        curve: 'curveMonotoneX'
    };

    // Donut chart - Payment Methods
    const paymentMethodData = salesByPaymentMethod();
    const donutChartData: DonutChartDataItem[] = Object.keys(paymentMethodData).map(method => ({
        group: method,
        value: paymentMethodData[method]
    }));

    const donutChartOptions: DonutChartOptions = {
        title: 'Sales by Payment Method',
        resizable: true,
        height: '300px',
        donut: {
            center: {
                label: 'Payment Methods'
            },
            alignment: 'center'
        },
        legend: {
            alignment: 'center'
        }
    };

    // Get recent transactions for display
    const recentOrders = filteredOrders
        .sort((a, b) => new Date(b.orderDate).getTime() - new Date(a.orderDate).getTime())
        .slice(0, 5);

    // Calculate order status distribution
    const orderStatusDistribution = () => {
        const statusData: { [key: string]: number } = {
            'Completed': 0,
            'Pending': 0,
            'Cancelled': 0
        };

        filteredOrders.forEach(order => {
            statusData[order.state]++;
        });

        return Object.entries(statusData).map(([status, count]) => ({
            group: 'Orders',
            key: status,
            value: count
        }));
    };

    const barChartOptions: BarChartOptions = {
        title: 'Order Status Distribution',
        axes: {
            left: {
                title: 'Number of Orders',
                mapsTo: 'value',
                scaleType: ScaleTypes.LINEAR
            },
            bottom: {
                title: 'Status',
                mapsTo: 'key',
                scaleType: ScaleTypes.LABELS
            }
        },
        height: '300px'
    };

    return (
        <Content className='min-h-[calc(100dvh-3rem)] p-4'>
            <div className="flex justify-between items-center mb-6">
                <h1 className="text-2xl font-bold">Sales Dashboard</h1>
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
                            case 'this-month': return 'This Month';
                            case 'last-month': return 'Last Month';
                            case 'last-3-months': return 'Last 3 Months';
                            case 'last-12-months': return 'Last 12 Months';
                            case 'custom': return 'Custom Range';
                            default: return item;
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
                            case 'daily': return 'Daily';
                            case 'weekly': return 'Weekly';
                            case 'monthly': return 'Monthly';
                            default: return item;
                        }
                    }}
                    onChange={(e: { selectedItem: string }) => setGroupBy(e.selectedItem)}
                />
                <Dropdown
                    id="payment-method-filter"
                    titleText="Payment Method"
                    label="All Methods"
                    items={['all', 'Cash', 'Card', 'UPI', 'Bank Transfer']}
                    itemToString={(item: string) => item === 'all' ? 'All Methods' : item}
                    onChange={(e: { selectedItem: string }) => setPaymentMethodFilter(e.selectedItem)}
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
                    <p>Loading sales data...</p>
                </div>
            ) : (
                <>
                    {/* Summary Metrics */}
                    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
                        <Tile>
                            <h3 className="text-sm mb-1">Total Sales</h3>
                            <p className="text-3xl font-medium">{formatCurrency(totalFilteredSales)}</p>
                            <p className="text-sm">
                                From {filteredOrders.length} orders
                            </p>
                        </Tile>
                        <Tile>
                            <h3 className="text-sm mb-1">Avg. Order Value</h3>
                            <p className="text-3xl font-medium">{formatCurrency(avgOrderValue)}</p>
                            <p className="text-sm">Per transaction</p>
                        </Tile>
                        <Tile>
                            <h3 className="text-sm mb-1">Active Customers</h3>
                            <p className="text-3xl font-medium">{customers.length}</p>
                            <p className="text-sm">Unique customers</p>
                        </Tile>
                        <Tile>
                            <h3 className="text-sm mb-1">Product Categories</h3>
                            <p className="text-3xl font-medium">{categories.length}</p>
                            <p className="text-sm">Available for sale</p>
                        </Tile>
                    </div>

                    {filteredOrders.length > 0 ? (
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
                                        data={orderStatusDistribution()}
                                        options={barChartOptions}
                                    />
                                </Tile>
                                <Tile>
                                    <h3 className="text-lg font-semibold mb-4">Recent Orders</h3>
                                    <Table>
                                        <TableHead>
                                            <TableRow>
                                                <TableHeader>Order ID</TableHeader>
                                                <TableHeader>Customer</TableHeader>
                                                <TableHeader>Amount</TableHeader>
                                                <TableHeader>Date</TableHeader>
                                                <TableHeader>Status</TableHeader>
                                            </TableRow>
                                        </TableHead>
                                        <TableBody>
                                            {recentOrders.map(order => (
                                                <TableRow key={order.id}>
                                                    <TableCell>{order.id}</TableCell>
                                                    <TableCell>{order.customerName}</TableCell>
                                                    <TableCell>{formatCurrency(Number(order.totalAmount))}</TableCell>
                                                    <TableCell>{formatDateForDisplay(order.orderDate)}</TableCell>
                                                    <TableCell>{order.state}</TableCell>
                                                </TableRow>
                                            ))}
                                        </TableBody>
                                    </Table>
                                </Tile>
                            </div>

                            {/* Top Customers */}
                            <div className="mb-6">
                                <Tile>
                                    <h3 className="text-lg font-semibold mb-4">Top Customers</h3>
                                    <Table>
                                        <TableHead>
                                            <TableRow>
                                                <TableHeader>Customer</TableHeader>
                                                <TableHeader>Total Spent</TableHeader>
                                            </TableRow>
                                        </TableHead>
                                        <TableBody>
                                            {topCustomers().map((customer, index) => (
                                                <TableRow key={index}>
                                                    <TableCell>{customer.customer}</TableCell>
                                                    <TableCell>{formatCurrency(customer.total)}</TableCell>
                                                </TableRow>
                                            ))}
                                        </TableBody>
                                    </Table>
                                </Tile>
                            </div>
                        </>
                    ) : (
                        <Tile className="mb-6 p-6">
                            <h3 className="text-lg font-semibold mb-4">No Sales Orders Found</h3>
                            <p className="mb-4">No sales orders match your current filter criteria. Try adjusting your filters.</p>
                            <Link href="/dash/sales/orders">
                                <Button>View All Orders</Button>
                            </Link>
                        </Tile>
                    )}
                </>
            )}
        </Content>
    )
}

export default SalesDashboard
