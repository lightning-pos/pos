import React from 'react';
import { Modal, DataTable, Table, TableHead, TableRow, TableHeader, TableBody, TableCell } from '@carbon/react';
import { Order, OrderItem } from '@/lib/powersync/app_schema';
import { db } from '@/components/providers/system_provider';

interface OrderDetailsModalProps {
  isOpen: boolean;
  onClose: () => void;
  order: Order | null;
}

const OrderDetailsModal: React.FC<OrderDetailsModalProps> = ({ isOpen, onClose, order }) => {
  const [orderItems, setOrderItems] = React.useState<OrderItem[]>([]);

  React.useEffect(() => {
    const fetchOrderItems = async () => {
      if (order) {
        const items = await db
          .selectFrom('order_items')
          .selectAll()
          .where('order_id', '=', order.id)
          .execute();
        setOrderItems(items);
      }
    };
    fetchOrderItems();
  }, [order]);

  const headers = [
    { key: 'item_name', header: 'Item' },
    { key: 'quantity', header: 'Quantity' },
    { key: 'price', header: 'Price' },
    { key: 'tax', header: 'Tax' },
  ];

  return (
    <Modal
      open={isOpen}
      onRequestClose={onClose}
      modalHeading={`Order Details - ${order?.id}`}
      primaryButtonText="Close"
      onRequestSubmit={onClose}
    >
      {order && (
        <>
          <div className="mb-4">
            <p><strong>Customer:</strong> {order.customer_name}</p>
            <p><strong>Phone:</strong> {order.customer_phone_number}</p>
            <p><strong>Date:</strong> {new Date(order.created_at).toLocaleString()}</p>
            <p><strong>Status:</strong> {order.status}</p>
            <p><strong>Payment Method:</strong> {order.payment_method}</p>
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
            <p><strong>Subtotal:</strong> Rs. {order.subtotal.toFixed(2)}</p>
            <p><strong>Tax:</strong> Rs. {order.tax.toFixed(2)}</p>
            <p><strong>Total:</strong> Rs. {order.total_amount.toFixed(2)}</p>
          </div>
        </>
      )}
    </Modal>
  );
};

export default OrderDetailsModal;
