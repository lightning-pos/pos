import React from 'react';
import { Modal, DataTable, Table, TableHead, TableRow, TableHeader, TableBody, TableCell, ModalProps } from '@carbon/react';
import { SalesOrder, SalesOrderItem } from '@/lib/graphql/graphql';
import { formatCurrency } from '@/lib/util/number_format';

interface OrderDetailsModalProps extends ModalProps {
    order: SalesOrder
    orderItems: SalesOrderItem[]
}

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
        { key: 'totalAmount', header: 'Total' },
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
                    <div className="mt-4">
                        <p><strong>Subtotal:</strong> {formatCurrency(parseFloat(order.netAmount))}</p>
                        <p><strong>Tax:</strong> {formatCurrency(parseFloat(order.taxAmount))}</p>
                        <p><strong>Total:</strong> {formatCurrency(parseFloat(order.totalAmount))}</p>
                    </div>
                </>
            )}
        </Modal>
    );
};

export default OrderDetailsModal;
