import React from 'react';
import { Modal, DataTable, Table, TableHead, TableRow, TableHeader, TableBody, TableCell, ModalProps } from '@carbon/react';
import { PurchaseOrder } from '@/lib/db/sqlite/schema';
import { money } from '@/lib/util/money';

interface PurchaseDetailsModalProps extends ModalProps {
  purchase: PurchaseOrder
}

const PurchaseDetailsModal: React.FC<PurchaseDetailsModalProps> = ({
  open,
  onRequestClose,
  purchase
}) => {
  return (
    <Modal
      open={open}
      onRequestClose={onRequestClose}
      modalHeading={`Purchase Details - ${purchase?.id}`}
      primaryButtonText="Close"
      onRequestSubmit={onRequestClose}
    >
      {purchase && (
        <>
          <div className="mb-4">
            <p><strong>Supplier:</strong> {purchase.supplierName}</p>
            <p><strong>Date:</strong> {new Date(purchase.createdAt).toLocaleString()}</p>
            <p><strong>Status:</strong> {purchase.state}</p>
          </div>
          <div className="mt-4">
            <p><strong>Net Amount:</strong> {money(purchase.netAmount ?? 0, 'INR').format()}</p>
            <p><strong>Tax Amount:</strong> {money(purchase.taxAmount ?? 0, 'INR').format()}</p>
            <p><strong>Total Amount:</strong> {money(purchase.totalAmount ?? 0, 'INR').format()}</p>
          </div>
        </>
      )}
    </Modal>
  );
};

export default PurchaseDetailsModal;
