import React, { useState, useEffect } from 'react';
import {
    Modal,
    DataTable,
    Table,
    TableHead,
    TableRow,
    TableHeader,
    TableBody,
    TableCell,
    ModalProps,
    Button,
    Accordion,
    AccordionItem,
    Select,
    SelectItem,
    NumberInput,
    TextInput,
    FormGroup
} from '@carbon/react';
import { SalesOrder, SalesOrderItem, SalesOrderState } from '@/lib/graphql/graphql';
import { formatCurrency } from '@/lib/util/number_format';
import { gql } from '@/lib/graphql/execute';
import { CreateSalesOrderPaymentDocument, GetSalesOrdersPaymentMethodsDocument, VoidSalesOrderDocument } from '@/lib/graphql/graphql';
import { Add, TrashCan, Warning } from '@carbon/icons-react';

interface OrderDetailsModalProps extends ModalProps {
    order: SalesOrder
    orderItems: SalesOrderItem[]
    onRequestClose?: () => void
}

interface PaymentMethod {
    id: string;
    name: string;
    code: string;
}

const OrderDetailsModal: React.FC<OrderDetailsModalProps> = ({
    open,
    onRequestClose = () => { },
    order,
    orderItems
}) => {
    const [isVoiding, setIsVoiding] = useState(false);
    const [paymentMethods, setPaymentMethods] = useState<PaymentMethod[]>([]);
    const [loading, setLoading] = useState(false);
    const [showAddPayment, setShowAddPayment] = useState(false);

    // New payment form state
    const [newPaymentMethodId, setNewPaymentMethodId] = useState('');
    const [newPaymentAmount, setNewPaymentAmount] = useState(0);
    const [newPaymentReference, setNewPaymentReference] = useState('');
    const [newPaymentNotes, setNewPaymentNotes] = useState('');

    // Calculate totals
    const totalAmount = parseFloat(order.totalAmount);
    const totalPaidAmount = parseFloat(order.totalPaidAmount?.toString() || '0');
    const remainingAmount = totalAmount - totalPaidAmount;

    useEffect(() => {
        if (open) {
            fetchPaymentMethods();

            // Set default payment amount to remaining amount
            setNewPaymentAmount(remainingAmount);
        }
    }, [open, remainingAmount]);

    const fetchPaymentMethods = async () => {
        try {
            const result = await gql(GetSalesOrdersPaymentMethodsDocument);
            if (result.paymentMethods) {
                setPaymentMethods(result.paymentMethods as PaymentMethod[]);
            }
        } catch (error) {
            console.error('Error fetching payment methods:', error);
        }
    };

    const handleVoidOrder = async () => {
        if (!confirm('Are you sure you want to cancel this order? This action cannot be undone.')) {
            return;
        }

        setIsVoiding(true);
        try {
            await gql(VoidSalesOrderDocument, { id: order.id });
            alert('Order has been cancelled successfully.');
            onRequestClose();
        } catch (error) {
            console.error('Error voiding order:', error);
            alert('Failed to cancel the order. Please try again.');
        } finally {
            setIsVoiding(false);
        }
    };

    const handleAddPayment = async () => {
        if (!newPaymentMethodId || newPaymentAmount <= 0) {
            alert('Please select a payment method and enter a valid amount.');
            return;
        }

        if (newPaymentAmount > remainingAmount) {
            alert(`Payment amount cannot exceed the remaining amount (${formatCurrency(remainingAmount)}).`);
            return;
        }

        setLoading(true);
        try {
            const payment = {
                orderId: order.id,
                paymentMethodId: newPaymentMethodId,
                paymentDate: new Date().toISOString(),
                amount: newPaymentAmount.toString(),
                referenceNumber: newPaymentReference || undefined,
                notes: newPaymentNotes || undefined
            };

            await gql(CreateSalesOrderPaymentDocument, { payment });

            alert('Payment added successfully.');
            resetPaymentForm();
            onRequestClose(); // Close and refresh
        } catch (error) {
            console.error('Error adding payment:', error);
            alert('Failed to add payment. Please try again.');
        } finally {
            setLoading(false);
        }
    };

    const resetPaymentForm = () => {
        setNewPaymentMethodId('');
        setNewPaymentAmount(remainingAmount);
        setNewPaymentReference('');
        setNewPaymentNotes('');
        setShowAddPayment(false);
    };

    const itemsHeaders = [
        { key: 'itemName', header: 'Item' },
        { key: 'quantity', header: 'Quantity' },
        { key: 'priceAmount', header: 'Price' },
        { key: 'taxAmount', header: 'Tax' },
        { key: 'totalAmount', header: 'Total' },
    ];

    const paymentsHeaders = [
        { key: 'method', header: 'Method' },
        { key: 'amount', header: 'Amount' },
        { key: 'date', header: 'Date' },
        { key: 'reference', header: 'Reference' },
        { key: 'status', header: 'Status' },
    ];

    const paymentsRows = order.payments ? order.payments.map((payment, index) => {
        const method = paymentMethods.find(m => m.id === payment.paymentMethodId);
        return {
            id: payment.id,
            method: method ? method.name : payment.paymentMethodId,
            amount: formatCurrency(parseFloat(payment.amount)),
            date: new Date(payment.paymentDate).toLocaleString(),
            reference: payment.referenceNumber || '-',
            status: payment.state,
        };
    }) : [];

    return (
        <Modal
            open={open}
            onRequestClose={onRequestClose}
            modalHeading={`Order Details - ${order?.id}`}
            primaryButtonText={order.orderState === SalesOrderState.Completed ? "Cancel Order" : "Close"}
            secondaryButtonText="Close"
            primaryButtonDisabled={isVoiding || order.orderState !== SalesOrderState.Completed}
            danger={order.orderState === SalesOrderState.Completed}
            onRequestSubmit={order.orderState === SalesOrderState.Completed ? handleVoidOrder : onRequestClose}
            onSecondarySubmit={onRequestClose}
            size="lg"
        >
            {order && (
                <>
                    <div className="mb-4 grid grid-cols-1 md:grid-cols-2 gap-4">
                        <div>
                            <h5 className="font-bold text-sm">Customer Information</h5>
                            <p><strong>Name:</strong> {order.customerName}</p>
                            <p><strong>Phone:</strong> {order.customerPhoneNumber}</p>
                            <p><strong>Date:</strong> {new Date(order.orderDate).toLocaleString()}</p>
                            <p><strong>Created:</strong> {new Date(order.createdAt).toLocaleString()}</p>
                        </div>
                        <div>
                            <h5 className="font-bold text-sm">Order Status</h5>
                            <p><strong>Status:</strong>
                                <span className={`ml-2 px-2 py-1 rounded text-sm ${order.orderState === SalesOrderState.Completed ? 'bg-green-100 text-green-800' :
                                    order.orderState === SalesOrderState.Cancelled ? 'bg-red-100 text-red-800' :
                                        'bg-yellow-100 text-yellow-800'
                                    }`}>
                                    {order.orderState}
                                </span>
                            </p>
                            <p><strong>Total Amount:</strong> {formatCurrency(parseFloat(order.totalAmount))}</p>
                            <p><strong>Paid Amount:</strong> {formatCurrency(totalPaidAmount)}</p>
                            <p><strong>Remaining:</strong> {formatCurrency(remainingAmount)}</p>
                        </div>
                    </div>

                    <Accordion className="mb-6">
                        <AccordionItem title="Order Items" open>
                            <DataTable rows={orderItems} headers={itemsHeaders}>
                                {({ rows, headers, getTableProps, getHeaderProps, getRowProps }) => (
                                    <Table {...getTableProps()}>
                                        <TableHead>
                                            <TableRow>
                                                {headers.map((header) => (
                                                    <TableHeader {...getHeaderProps({ header })} key={header.key}>
                                                        {header.header}
                                                    </TableHeader>
                                                ))}
                                            </TableRow>
                                        </TableHead>
                                        <TableBody>
                                            {rows.map((row) => (
                                                <TableRow {...getRowProps({ row })} key={row.id}>
                                                    {row.cells.map((cell) => (
                                                        <TableCell key={cell.id}>
                                                            {cell.info.header === 'priceAmount' || cell.info.header === 'taxAmount' || cell.info.header === 'totalAmount'
                                                                ? formatCurrency(parseFloat(cell.value))
                                                                : cell.value}
                                                        </TableCell>
                                                    ))}
                                                </TableRow>
                                            ))}
                                        </TableBody>
                                    </Table>
                                )}
                            </DataTable>
                        </AccordionItem>

                        <AccordionItem title="Payment Information" open>
                            <div className="flex justify-between items-center mb-4">
                                <div>
                                    <p><strong>Subtotal:</strong> {formatCurrency(parseFloat(order.netAmount))}</p>
                                    <p><strong>Discount:</strong> {formatCurrency(parseFloat(order.discAmount))}</p>
                                    <p><strong>Tax:</strong> {formatCurrency(parseFloat(order.taxAmount))}</p>
                                    <p><strong>Total:</strong> {formatCurrency(parseFloat(order.totalAmount))}</p>
                                </div>

                                {order.orderState === SalesOrderState.Completed && remainingAmount > 0 && (
                                    <Button
                                        kind="tertiary"
                                        size="sm"
                                        renderIcon={Add}
                                        onClick={() => setShowAddPayment(!showAddPayment)}
                                    >
                                        {showAddPayment ? 'Cancel' : 'Add Payment'}
                                    </Button>
                                )}
                            </div>

                            {showAddPayment && (
                                <div className="mb-6 p-4 border border-gray-200 rounded">
                                    <h5 className="font-bold text-sm mb-4">Add Payment</h5>
                                    <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
                                        <FormGroup legendText="Payment Method">
                                            <Select
                                                id="payment-method-select"
                                                labelText="Select Method"
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
                                        </FormGroup>
                                        <FormGroup legendText="Amount">
                                            <NumberInput
                                                id="payment-amount"
                                                label="Payment Amount"
                                                min={0}
                                                max={remainingAmount}
                                                step={0.01}
                                                value={newPaymentAmount}
                                                onChange={(e: any) => setNewPaymentAmount(parseFloat(e.target.value) || 0)}
                                            />
                                        </FormGroup>
                                    </div>

                                    <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
                                        <FormGroup legendText="Reference">
                                            <TextInput
                                                id="payment-reference"
                                                labelText="Reference Number"
                                                value={newPaymentReference}
                                                onChange={(e) => setNewPaymentReference(e.target.value)}
                                            />
                                        </FormGroup>
                                        <FormGroup legendText="Notes">
                                            <TextInput
                                                id="payment-notes"
                                                labelText="Payment Notes"
                                                value={newPaymentNotes}
                                                onChange={(e) => setNewPaymentNotes(e.target.value)}
                                            />
                                        </FormGroup>
                                    </div>

                                    <div className="flex justify-end">
                                        <Button
                                            kind="secondary"
                                            className="mr-2"
                                            onClick={resetPaymentForm}
                                        >
                                            Cancel
                                        </Button>
                                        <Button
                                            kind="primary"
                                            onClick={handleAddPayment}
                                            disabled={loading || !newPaymentMethodId || newPaymentAmount <= 0}
                                        >
                                            {loading ? 'Processing...' : 'Add Payment'}
                                        </Button>
                                    </div>
                                </div>
                            )}

                            {paymentsRows.length > 0 ? (
                                <DataTable rows={paymentsRows} headers={paymentsHeaders}>
                                    {({ rows, headers, getTableProps, getHeaderProps, getRowProps }) => (
                                        <Table {...getTableProps()}>
                                            <TableHead>
                                                <TableRow>
                                                    {headers.map((header) => (
                                                        <TableHeader {...getHeaderProps({ header })} key={header.key}>
                                                            {header.header}
                                                        </TableHeader>
                                                    ))}
                                                </TableRow>
                                            </TableHead>
                                            <TableBody>
                                                {rows.map((row) => (
                                                    <TableRow {...getRowProps({ row })} key={row.id}>
                                                        {row.cells.map((cell) => (
                                                            <TableCell key={cell.id}>
                                                                {cell.value}
                                                            </TableCell>
                                                        ))}
                                                    </TableRow>
                                                ))}
                                            </TableBody>
                                        </Table>
                                    )}
                                </DataTable>
                            ) : (
                                <div className="text-gray-500 p-4 text-center">
                                    No payments recorded for this order.
                                </div>
                            )}
                        </AccordionItem>
                    </Accordion>

                    {order.orderState === SalesOrderState.Cancelled && (
                        <div className="bg-red-50 border border-red-200 text-red-800 p-4 rounded flex items-center">
                            <Warning className="mr-2" />
                            This order has been cancelled and cannot be modified.
                        </div>
                    )}
                </>
            )}
        </Modal>
    );
};

export default OrderDetailsModal;
