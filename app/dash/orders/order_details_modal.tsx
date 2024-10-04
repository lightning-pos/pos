import React from 'react';
import { Modal, DataTable, Table, TableHead, TableRow, TableHeader, TableBody, TableCell, ModalProps } from '@carbon/react';
import { Order, OrderItem } from '@/lib/db/sqlite/schema';

interface OrderDetailsModalProps extends ModalProps {
  order: Order
  orderItems: OrderItem[]
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
            {/* <p><strong>Payment Method:</strong> {order.paymentMethod}</p> */}
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
                          {cell.info.header === 'price' || cell.info.header === 'tax'
                            ? `Rs. ${Number(cell.value).toFixed(2)}`
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
            <p><strong>Subtotal:</strong> Rs. {order.netAmount ? order.netAmount.toFixed(2) : 0}</p>
            <p><strong>Tax:</strong> Rs. {order.taxAmount ? order.taxAmount.toFixed(2) : 0}</p>
            <p><strong>Total:</strong> Rs. {order.totalAmount ? order.totalAmount.toFixed(2) : 0}</p>
          </div>
        </>
      )}
    </Modal>
  );
};

export default OrderDetailsModal;
