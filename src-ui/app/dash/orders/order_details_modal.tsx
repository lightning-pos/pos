import React from 'react';
import { Modal, DataTable, Table, TableHead, TableRow, TableHeader, TableBody, TableCell, ModalProps } from '@carbon/react';

interface SalesOrderState {
    DRAFT: 'DRAFT'
    COMPLETED: 'COMPLETED'
    CANCELLED: 'CANCELLED'
}

interface SalesOrder {
    id: string
    customerId: string
    customerName: string
    customerPhoneNumber: string
    orderDate: string
    netAmount: number
    discAmount: number
    taxableAmount: number
    taxAmount: number
    totalAmount: number
    state: keyof SalesOrderState
    createdAt: string
    updatedAt: string
    customer: {
        id: string
        fullName: string
        phone: string
    }
    items: SalesOrderItem[]
}

interface SalesOrderItem {
    id: string
    orderId: string
    itemId: string
    itemName: string
    quantity: number
    priceAmount: number
    taxAmount: number
    createdAt: string
    updatedAt: string
}

interface OrderDetailsModalProps extends ModalProps {
    order: SalesOrder
    orderItems: SalesOrderItem[]
}

const formatPrice = (price: number): string => {
    return new Intl.NumberFormat('en-IN', {
        style: 'currency',
        currency: 'INR'
    }).format(price);
};

const OrderDetailsModal: React.FC<OrderDetailsModalProps> = ({
    open,
    onRequestClose,
    order,
    orderItems
}) => {
    const headers = [
        { key: 'itemName', header: 'Item' },
        { key: 'quantity', header: 'Quantity' },
        { key: 'priceAmount', header: 'Price' },
        { key: 'taxAmount', header: 'Tax' },
    ];

    return (
        <Modal
            open={open}
            onRequestClose={onRequestClose}
            modalHeading={`Order Details - ${order?.id}`}
            primaryButtonText={"Cancel Order"}
            danger
        >
            {order && (
                <>
                    <div className="mb-4">
                        <p><strong>Customer:</strong> {order.customerName}</p>
                        <p><strong>Phone:</strong> {order.customerPhoneNumber}</p>
                        <p><strong>Date:</strong> {new Date(order.createdAt).toLocaleString()}</p>
                        <p><strong>Status:</strong> {order.state}</p>
                    </div>
                    <DataTable rows={orderItems} headers={headers}>
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
                                                    {cell.info.header === 'priceAmount' || cell.info.header === 'taxAmount'
                                                        ? formatPrice(cell.value)
                                                        : cell.value}
                                                </TableCell>
                                            ))}
                                        </TableRow>
                                    ))}
                                </TableBody>
                            </Table>
                        )}
                    </DataTable>
                    <div className="mt-4">
                        <p><strong>Subtotal:</strong> {formatPrice(order.netAmount ?? 0)}</p>
                        <p><strong>Tax:</strong> {formatPrice(order.taxAmount ?? 0)}</p>
                        <p><strong>Total:</strong> {formatPrice(order.totalAmount ?? 0)}</p>
                    </div>
                </>
            )}
        </Modal>
    );
};

export default OrderDetailsModal;
