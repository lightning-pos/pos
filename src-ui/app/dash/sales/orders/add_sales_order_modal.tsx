'use client'
import React, { useState, useEffect } from 'react'
import {
    Modal,
    TextInput,
    NumberInput,
    DatePicker,
    DatePickerInput,
    Button,
    FormGroup,
    Select,
    SelectItem,
    Accordion,
    AccordionItem,
    DataTable,
    TableContainer,
    Table,
    TableHead,
    TableRow,
    TableHeader,
    TableBody,
    TableCell,
    FormLabel,
    InlineNotification
} from '@carbon/react'
import { Add, TrashCan, Save } from '@carbon/icons-react'
import { formatCurrency } from '@/lib/util/number_format'
import { gql } from '@/lib/graphql/execute'
import {
    SalesOrdersCreateSalesOrderDocument,
    CreateSalesOrderPaymentDocument,
    GetSalesOrdersCustomersDocument,
    GetSalesOrdersPaymentMethodsDocument,
    GetSalesOrdersCostCentersDocument,
    CreateCustomerDocument,
    SalesOrderState,
    SalesOrderPaymentState,
    SalesOrderItemInput,
    SalesOrderNewInput,
    SalesOrderPaymentNewInput,
    CustomerNewInput,
    Customer,
    PaymentMethod,
    CostCenter
} from '@/lib/graphql/graphql'
import { formatToLocalDateTime } from '@/lib/util/date_format'

interface SalesOrderModalProps {
    isOpen: boolean
    onClose: () => void
    onSave: () => void
}

const AddSalesOrderModal: React.FC<SalesOrderModalProps> = ({ isOpen, onClose, onSave }) => {
    const [customers, setCustomers] = useState<Array<Pick<Customer, 'id' | 'fullName' | 'phone' | 'email'>>>([])
    const [paymentMethods, setPaymentMethods] = useState<Array<PaymentMethod>>([])
    const [costCenters, setCostCenters] = useState<Array<CostCenter>>([])
    const [loading, setLoading] = useState(false)
    const [notification, setNotification] = useState<{
        type: 'success' | 'error',
        title: string,
        subtitle?: string,
        show: boolean
    } | null>(null)

    // Form state
    const [customerId, setCustomerId] = useState('')
    const [customerName, setCustomerName] = useState('')
    const [customerPhone, setCustomerPhone] = useState('')
    const [orderDate, setOrderDate] = useState<Date>(new Date())
    const [costCenterId, setCostCenterId] = useState('')
    const [items, setItems] = useState<Array<SalesOrderItemInput>>([])
    const [payments, setPayments] = useState<Array<Omit<SalesOrderPaymentNewInput, 'orderId' | 'paymentDate' | 'state'>>>([])

    // New item form state
    const [newItemName, setNewItemName] = useState('')
    const [newItemQuantity, setNewItemQuantity] = useState(1)
    const [newItemPrice, setNewItemPrice] = useState(0)
    const [newItemTax, setNewItemTax] = useState(0)

    // New payment form state
    const [newPaymentMethodId, setNewPaymentMethodId] = useState('')
    const [newPaymentAmount, setNewPaymentAmount] = useState(0)
    const [newPaymentReference, setNewPaymentReference] = useState('')
    const [newPaymentNotes, setNewPaymentNotes] = useState('')

    // Calculate totals
    const totalNetAmount = items.reduce((sum, item) => sum + (parseFloat(item.priceAmount) * item.quantity), 0)
    const totalTaxAmount = items.reduce((sum, item) => sum + (parseFloat(item.taxAmount) * item.quantity), 0)
    const totalAmount = totalNetAmount + totalTaxAmount
    const totalPaidAmount = payments.reduce((sum, payment) => sum + parseFloat(payment.amount), 0)
    const remainingAmount = totalAmount - totalPaidAmount

    useEffect(() => {
        if (isOpen) {
            fetchData()
        }
    }, [isOpen])

    const showNotification = (type: 'success' | 'error', title: string, subtitle?: string) => {
        setNotification({ type, title, subtitle, show: true })
        setTimeout(() => setNotification(null), 5000)
    }

    const fetchData = async () => {
        setLoading(true)
        try {
            const [customersResult, paymentMethodsResult, costCentersResult] = await Promise.all([
                gql(GetSalesOrdersCustomersDocument),
                gql(GetSalesOrdersPaymentMethodsDocument),
                gql(GetSalesOrdersCostCentersDocument)
            ])

            if (customersResult.customers) {
                setCustomers(customersResult.customers)
            }

            if (paymentMethodsResult.paymentMethods) {
                setPaymentMethods(paymentMethodsResult.paymentMethods as PaymentMethod[])
            }

            if (costCentersResult.costCenters) {
                setCostCenters(costCentersResult.costCenters as CostCenter[])
                // Set default cost center if available
                if (costCentersResult.costCenters.length > 0) {
                    setCostCenterId(costCentersResult.costCenters[0].id)
                }
            }
        } catch (error) {
            console.error('Error fetching data:', error)
            showNotification('error', 'Error', 'Failed to load data')
        } finally {
            setLoading(false)
        }
    }

    const handleCustomerChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
        const selectedId = e.target.value
        setCustomerId(selectedId)

        const selectedCustomer = customers.find(c => c.id === selectedId)
        if (selectedCustomer) {
            setCustomerName(selectedCustomer.fullName)
            setCustomerPhone(selectedCustomer.phone || '')
        }
    }

    const createNewCustomer = async (name: string, phone: string): Promise<string | null> => {
        try {
            // Check if a customer with this phone already exists
            const existingCustomer = customers.find(c => c.phone === phone)
            if (existingCustomer) {
                return existingCustomer.id
            }

            // Create a new customer
            const customerInput: CustomerNewInput = {
                fullName: name,
                phone: phone,
            }

            const result = await gql(CreateCustomerDocument, { input: customerInput })
            if (result.createCustomer) {
                // Add to local customers state
                setCustomers(prev => [...prev, {
                    id: result.createCustomer.id,
                    fullName: result.createCustomer.fullName,
                    phone: result.createCustomer.phone,
                    email: result.createCustomer.email
                }])
                return result.createCustomer.id
            }
            return null
        } catch (error) {
            console.error('Error creating customer:', error)
            showNotification('error', 'Error', 'Failed to create customer')
            return null
        }
    }

    const handleAddItem = () => {
        if (!newItemName || newItemQuantity <= 0 || newItemPrice <= 0) return

        const newItem: SalesOrderItemInput = {
            itemName: newItemName,
            quantity: newItemQuantity,
            priceAmount: newItemPrice.toString(),
            discAmount: '0', // No discount for now
            taxableAmount: newItemPrice.toString(),
            taxAmount: newItemTax.toString(),
            totalAmount: ((newItemPrice + newItemTax) * newItemQuantity).toString(),
            sku: undefined // Optional field
        }

        setItems([...items, newItem])

        // Reset form
        setNewItemName('')
        setNewItemQuantity(1)
        setNewItemPrice(0)
        setNewItemTax(0)
    }

    const handleRemoveItem = (index: number) => {
        const updatedItems = [...items]
        updatedItems.splice(index, 1)
        setItems(updatedItems)
    }

    const handleAddPayment = () => {
        if (!newPaymentMethodId || newPaymentAmount <= 0) return

        const newPayment: Omit<SalesOrderPaymentNewInput, 'orderId' | 'paymentDate' | 'state'> = {
            paymentMethodId: newPaymentMethodId,
            amount: newPaymentAmount.toString(),
            referenceNumber: newPaymentReference || undefined,
            notes: newPaymentNotes || undefined
        }

        setPayments([...payments, newPayment])

        // Reset form
        setNewPaymentMethodId('')
        setNewPaymentAmount(remainingAmount > 0 ? remainingAmount : 0)
        setNewPaymentReference('')
        setNewPaymentNotes('')
    }

    const handleRemovePayment = (index: number) => {
        const updatedPayments = [...payments]
        updatedPayments.splice(index, 1)
        setPayments(updatedPayments)
    }

    const handleSubmit = async () => {
        // Check if we have either a selected customer (customerId) OR manually entered customer details (name and phone)
        if ((!customerId && (!customerName || !customerPhone)) || !costCenterId || items.length === 0) {
            showNotification('error', 'Required fields missing', 'Please fill in all required fields and add at least one item')
            return
        }

        try {
            setLoading(true)

            // If no customerId but we have name and phone, create a new customer
            let effectiveCustomerId = customerId
            if (!effectiveCustomerId && customerName && customerPhone) {
                const newCustomerId = await createNewCustomer(customerName, customerPhone)
                if (newCustomerId) {
                    effectiveCustomerId = newCustomerId
                } else {
                    showNotification('error', 'Error', 'Failed to create customer')
                    setLoading(false)
                    return
                }
            }

            // Format the orderDate correctly for the GraphQL API
            const formattedOrderDate = formatToLocalDateTime(orderDate)

            // Create the sales order
            const orderInput: SalesOrderNewInput = {
                customerId: effectiveCustomerId,
                customerName,
                customerPhoneNumber: customerPhone,
                orderDate: formattedOrderDate,
                netAmount: totalNetAmount.toString(),
                discAmount: '0', // No discount for now
                taxableAmount: totalNetAmount.toString(),
                taxAmount: totalTaxAmount.toString(),
                totalAmount: totalAmount.toString(),
                channelId: '00000000-0000-0000-0000-000000000000', // Required field
                locationId: '00000000-0000-0000-0000-000000000000', // Required field
                costCenterId,
                items
            }

            console.log('Submitting order:', { order: orderInput })
            const result = await gql(SalesOrdersCreateSalesOrderDocument, { order: orderInput })

            if (result.createSalesOrder) {
                const orderId = result.createSalesOrder.id

                // Create payments if any
                if (payments.length > 0) {
                    const paymentPromises = payments.map(payment =>
                        gql(CreateSalesOrderPaymentDocument, {
                            payment: {
                                ...payment,
                                orderId,
                                paymentDate: formatToLocalDateTime(new Date()),
                                state: SalesOrderPaymentState.Paid
                            }
                        })
                    )

                    await Promise.all(paymentPromises)
                }

                showNotification('success', 'Order Created', 'Sales order created successfully')
                resetForm()
                onSave()
            } else {
                showNotification('error', 'Error', 'Failed to create order: No response from server')
            }
        } catch (error) {
            console.error('Error creating sales order:', error)
            showNotification('error', 'Error', `Failed to create sales order: ${(error as Error).message}`)
        } finally {
            setLoading(false)
        }
    }

    const resetForm = () => {
        setCustomerId('')
        setCustomerName('')
        setCustomerPhone('')
        setOrderDate(new Date())
        setCostCenterId(costCenters.length > 0 ? costCenters[0].id : '')
        setItems([])
        setPayments([])
        setNewItemName('')
        setNewItemQuantity(1)
        setNewItemPrice(0)
        setNewItemTax(0)
        setNewPaymentMethodId('')
        setNewPaymentAmount(0)
        setNewPaymentReference('')
        setNewPaymentNotes('')
    }

    const itemsHeaders = [
        { key: 'itemName', header: 'Item' },
        { key: 'quantity', header: 'Qty' },
        { key: 'priceAmount', header: 'Price' },
        { key: 'taxAmount', header: 'Tax' },
        { key: 'totalAmount', header: 'Total' },
        { key: 'actions', header: 'Actions' }
    ]

    const itemsRows = items.map((item, index) => ({
        id: `item-${index}`,
        itemName: item.itemName,
        quantity: item.quantity,
        priceAmount: formatCurrency(parseFloat(item.priceAmount)),
        taxAmount: formatCurrency(parseFloat(item.taxAmount)),
        totalAmount: formatCurrency(parseFloat(item.totalAmount)),
        actions: (
            <Button
                kind="ghost"
                renderIcon={TrashCan}
                iconDescription="Remove item"
                hasIconOnly
                size="sm"
                onClick={() => handleRemoveItem(index)}
            />
        )
    }))

    const paymentsHeaders = [
        { key: 'method', header: 'Method' },
        { key: 'amount', header: 'Amount' },
        { key: 'reference', header: 'Reference' },
        { key: 'actions', header: 'Actions' }
    ]

    const paymentsRows = payments.map((payment, index) => {
        const method = paymentMethods.find(m => m.id === payment.paymentMethodId)
        return {
            id: `payment-${index}`,
            method: method ? method.name : payment.paymentMethodId,
            amount: formatCurrency(parseFloat(payment.amount)),
            reference: payment.referenceNumber || '-',
            actions: (
                <Button
                    kind="ghost"
                    renderIcon={TrashCan}
                    iconDescription="Remove payment"
                    hasIconOnly
                    size="sm"
                    onClick={() => handleRemovePayment(index)}
                />
            )
        }
    })

    return (
        <Modal
            open={isOpen}
            onRequestClose={onClose}
            modalHeading="Create New Sales Order"
            modalLabel="Sales"
            primaryButtonText="Create Order"
            secondaryButtonText="Cancel"
            primaryButtonDisabled={loading || items.length === 0}
            onRequestSubmit={handleSubmit}
            onSecondarySubmit={onClose}
            size="lg"
        >
            {notification && notification.show && (
                <div className="mb-4">
                    <InlineNotification
                        kind={notification.type}
                        title={notification.title}
                        subtitle={notification.subtitle}
                        onClose={() => setNotification(null)}
                    />
                </div>
            )}

            <div className="space-y-6">
                <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <FormGroup legendText="Customer Information">
                        <Select
                            id="customer-select"
                            labelText="Customer"
                            value={customerId}
                            onChange={handleCustomerChange}
                        >
                            <SelectItem text="Select a customer" value="" />
                            {customers.map(customer => (
                                <SelectItem
                                    key={customer.id}
                                    text={`${customer.fullName} ${customer.phone ? `(${customer.phone})` : ''}`}
                                    value={customer.id}
                                />
                            ))}
                        </Select>

                        <TextInput
                            id="customer-name"
                            labelText="Customer Name"
                            value={customerName}
                            onChange={(e) => setCustomerName(e.target.value)}
                        />

                        <TextInput
                            id="customer-phone"
                            labelText="Phone Number"
                            value={customerPhone}
                            onChange={(e) => setCustomerPhone(e.target.value)}
                        />
                    </FormGroup>

                    <FormGroup legendText="Order Information">
                        <DatePicker dateFormat="Y-m-d" datePickerType="single" value={orderDate}>
                            <DatePickerInput
                                id="order-date"
                                labelText="Order Date"
                                placeholder="yyyy-mm-dd"
                                onChange={(e: React.ChangeEvent<HTMLInputElement>) => {
                                    const date = new Date(e.target.value)
                                    if (!isNaN(date.getTime())) {
                                        setOrderDate(date)
                                    }
                                }}
                            />
                        </DatePicker>

                        <Select
                            id="cost-center-select"
                            labelText="Cost Center"
                            value={costCenterId}
                            onChange={(e) => setCostCenterId(e.target.value)}
                        >
                            <SelectItem text="Select cost center" value="" />
                            {costCenters.map(center => (
                                <SelectItem
                                    key={center.id}
                                    text={`${center.name} (${center.code})`}
                                    value={center.id}
                                />
                            ))}
                        </Select>
                    </FormGroup>
                </div>

                <Accordion>
                    <AccordionItem title="Order Items" open>
                        <div className="mb-4 p-4 border border-gray-200 rounded">
                            <div className="grid grid-cols-1 md:grid-cols-4 gap-4 mb-4">
                                <TextInput
                                    id="new-item-name"
                                    labelText="Item Name"
                                    value={newItemName}
                                    onChange={(e) => setNewItemName(e.target.value)}
                                />

                                <NumberInput
                                    id="new-item-quantity"
                                    label="Quantity"
                                    min={1}
                                    value={newItemQuantity}
                                    onChange={(e: any) => setNewItemQuantity(parseInt(e.target.value) || 1)}
                                />

                                <NumberInput
                                    id="new-item-price"
                                    label="Price"
                                    min={0}
                                    step={0.01}
                                    value={newItemPrice}
                                    onChange={(e: any) => setNewItemPrice(parseFloat(e.target.value) || 0)}
                                />

                                <NumberInput
                                    id="new-item-tax"
                                    label="Tax"
                                    min={0}
                                    step={0.01}
                                    value={newItemTax}
                                    onChange={(e: any) => setNewItemTax(parseFloat(e.target.value) || 0)}
                                />
                            </div>

                            <Button
                                kind="primary"
                                size="sm"
                                renderIcon={Add}
                                onClick={handleAddItem}
                                disabled={!newItemName || newItemQuantity <= 0 || newItemPrice < 0}
                            >
                                Add Item
                            </Button>
                        </div>

                        {items.length > 0 ? (
                            <DataTable rows={itemsRows} headers={itemsHeaders}>
                                {({ rows, headers, getTableProps, getHeaderProps, getRowProps }) => (
                                    <TableContainer>
                                        <Table {...getTableProps()}>
                                            <TableHead>
                                                <TableRow>
                                                    {headers.map((header, i) => {
                                                        const { key, ...rest } = getHeaderProps({ header })
                                                        return (
                                                            <TableHeader
                                                                key={`header-${i}`}
                                                                {...rest}
                                                            >
                                                                {header.header}
                                                            </TableHeader>
                                                        )
                                                    })}
                                                </TableRow>
                                            </TableHead>
                                            <TableBody>
                                                {rows.map((row, i) => {
                                                    const { key, ...rest } = getRowProps({ row })
                                                    return (
                                                        <TableRow
                                                            key={`row-${i}`}
                                                            {...rest}
                                                        >
                                                            {row.cells.map((cell, j) => (
                                                                <TableCell key={`cell-${i}-${j}`}>
                                                                    {cell.value}
                                                                </TableCell>
                                                            ))}
                                                        </TableRow>
                                                    )
                                                })}
                                            </TableBody>
                                        </Table>
                                    </TableContainer>
                                )}
                            </DataTable>
                        ) : (
                            <div className="text-gray-500 p-4 text-center">
                                No items added. Add at least one item to create an order.
                            </div>
                        )}
                    </AccordionItem>

                    <AccordionItem title="Payments" open={items.length > 0}>
                        {items.length > 0 ? (
                            <>
                                <div className="flex justify-between mb-4">
                                    <div>
                                        <p><strong>Total Amount:</strong> {formatCurrency(totalAmount)}</p>
                                        <p><strong>Total Paid:</strong> {formatCurrency(totalPaidAmount)}</p>
                                        <p><strong>Remaining:</strong> {formatCurrency(remainingAmount)}</p>
                                    </div>
                                </div>

                                <div className="mb-4 p-4 border border-gray-200 rounded">
                                    <div className="grid grid-cols-1 md:grid-cols-4 gap-4 mb-4">
                                        <Select
                                            id="payment-method-select"
                                            labelText="Payment Method"
                                            value={newPaymentMethodId}
                                            onChange={(e) => setNewPaymentMethodId(e.target.value)}
                                        >
                                            <SelectItem text="Select payment method" value="" />
                                            {paymentMethods.map(method => (
                                                <SelectItem
                                                    key={method.id}
                                                    text={`${method.name} (${method.code})`}
                                                    value={method.id}
                                                />
                                            ))}
                                        </Select>

                                        <NumberInput
                                            id="payment-amount"
                                            label="Amount"
                                            min={0}
                                            max={remainingAmount > 0 ? remainingAmount : totalAmount}
                                            step={0.01}
                                            value={newPaymentAmount || (remainingAmount > 0 ? remainingAmount : 0)}
                                            onChange={(e: any) => setNewPaymentAmount(parseFloat(e.target.value) || 0)}
                                        />

                                        <TextInput
                                            id="payment-reference"
                                            labelText="Reference Number"
                                            value={newPaymentReference}
                                            onChange={(e) => setNewPaymentReference(e.target.value)}
                                        />

                                        <TextInput
                                            id="payment-notes"
                                            labelText="Notes"
                                            value={newPaymentNotes}
                                            onChange={(e) => setNewPaymentNotes(e.target.value)}
                                        />
                                    </div>

                                    <Button
                                        kind="primary"
                                        size="sm"
                                        renderIcon={Add}
                                        onClick={handleAddPayment}
                                        disabled={!newPaymentMethodId || newPaymentAmount <= 0 || newPaymentAmount > remainingAmount}
                                    >
                                        Add Payment
                                    </Button>
                                </div>

                                {payments.length > 0 ? (
                                    <DataTable rows={paymentsRows} headers={paymentsHeaders}>
                                        {({ rows, headers, getTableProps, getHeaderProps, getRowProps }) => (
                                            <TableContainer>
                                                <Table {...getTableProps()}>
                                                    <TableHead>
                                                        <TableRow>
                                                            {headers.map((header, i) => {
                                                                const { key, ...rest } = getHeaderProps({ header })
                                                                return (
                                                                    <TableHeader
                                                                        key={`payment-header-${i}`}
                                                                        {...rest}
                                                                    >
                                                                        {header.header}
                                                                    </TableHeader>
                                                                )
                                                            })}
                                                        </TableRow>
                                                    </TableHead>
                                                    <TableBody>
                                                        {rows.map((row, i) => {
                                                            const { key, ...rest } = getRowProps({ row })
                                                            return (
                                                                <TableRow
                                                                    key={`payment-row-${i}`}
                                                                    {...rest}
                                                                >
                                                                    {row.cells.map((cell, j) => (
                                                                        <TableCell key={`payment-cell-${i}-${j}`}>
                                                                            {cell.value}
                                                                        </TableCell>
                                                                    ))}
                                                                </TableRow>
                                                            )
                                                        })}
                                                    </TableBody>
                                                </Table>
                                            </TableContainer>
                                        )}
                                    </DataTable>
                                ) : (
                                    <div className="text-gray-500 p-4 text-center">
                                        No payments added. Add at least one payment for this order.
                                    </div>
                                )}
                            </>
                        ) : (
                            <div className="text-gray-500 p-4 text-center">
                                Add items to the order first.
                            </div>
                        )}
                    </AccordionItem>
                </Accordion>
            </div>
        </Modal>
    )
}

export default AddSalesOrderModal
