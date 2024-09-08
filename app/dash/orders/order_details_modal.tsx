import React from 'react';
import { Modal, Button } from '@carbon/react';
import { db } from '@/components/providers/system_provider';
import { Order, OrderItem } from '@/lib/powersync/app_schema';

interface PaymentMethod {
  method: string;
  amount: number;
}

interface OrderDetailsModalProps {
  isOpen: boolean;
  onClose: () => void;
  order: Order;
  orderItems: OrderItem[];
}

const OrderDetailsModal: React.FC<OrderDetailsModalProps> = ({ isOpen, onClose, order, orderItems }) => {
  const formatPaymentMethods = (paymentMethodJson: string): string => {
    try {
      const paymentMethods: PaymentMethod[] = JSON.parse(paymentMethodJson);
      return paymentMethods
        .filter(pm => pm.amount > 0)
        .map(pm => `${pm.method}: Rs. ${pm.amount.toFixed(2)}`)
        .join(', ');
    } catch (error) {
      console.error('Error parsing payment method JSON:', error);
      return paymentMethodJson;
    }
  };

  const handleCancelOrder = async () => {
    await db.updateTable('orders')
      .set({ status: 'cancelled' })
      .where('id', '=', order.id)
      .execute();
    onClose();
  };

  return (
    <Modal
      modalHeading="Order Details"
      open={isOpen}
      onRequestClose={onClose}
      passiveModal
    >
      <div className="mb-4">
        <h3 className="text-md font-bold mb-2">Order Summary</h3>
        <p>Order ID: {order.id}</p>
        <p>Total Amount: Rs. {(order.total_amount ?? 0).toFixed(2)}</p>
        <p>Payment Method: {formatPaymentMethods(order.payment_method || '')}</p>
        <p>Status: {order.status}</p>
        <p>Created At: {new Date(order.created_at ?? 0).toLocaleString()}</p>
      </div>

      <div className="mb-4">
        <h3 className="text-md font-bold mb-2">Order Items</h3>
        {orderItems.map((item) => (
          <div key={item.id} className="flex justify-between">
            <span>{item.item_name} x {item.quantity}</span>
            <span>Rs. {(item.price ?? 0).toFixed(2)}</span>
          </div>
        ))}
      </div>

      {order.status !== 'cancelled' && (
        <Button kind="danger" onClick={handleCancelOrder}>
          Cancel Order
        </Button>
      )}
    </Modal>
  );
};

export default OrderDetailsModal;
