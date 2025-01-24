import React from 'react';
import { Modal, DataTable, Table, TableHead, TableRow, TableHeader, TableBody, TableCell, ModalProps } from '@carbon/react';
import { Order, OrderItem } from '@/lib/db/sqlite/schema';
import { money } from '@/lib/util/money';

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
                          {cell.info.header === 'priceAmount' || cell.info.header === 'taxAmount'
                            ? money(cell.value, 'INR').format()
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
            <p><strong>Subtotal:</strong> {money(order.netAmount ?? 0, 'INR').format()}</p>
            <p><strong>Tax:</strong> {money(order.taxAmount ?? 0, 'INR').format()}</p>
            <p><strong>Total:</strong> {money(order.totalAmount ?? 0, 'INR').format()}</p>
          </div>
        </>
      )}
    </Modal>
  );
};

export default OrderDetailsModal;
