'use client';

import React, { useState, useEffect, useCallback } from 'react';
import { Content } from '@carbon/react';
import DataTable from '@/components/ui/DataTable';
import { formatDateYMD } from '@/lib/util/date_format';
import { gql } from '@/lib/graphql/execute';
import { GetPaymentMethodsDocument, PaymentMethodState, PaymentMethod } from '@/lib/graphql/graphql';
import AddPaymentMethodModal from '@/app/dash/settings/payment-methods/add_payment_method_modal';
import EditPaymentMethodModal from '@/app/dash/settings/payment-methods/edit_payment_method_modal';
import DeletePaymentMethodModal from '@/app/dash/settings/payment-methods/delete_payment_method_modal';

// Define the table row structure
interface TableRow extends PaymentMethod {
    status: string;
    formattedCreatedAt: string;
}

// Define the state tag colors based on the state value
const stateTagColors = {
    [PaymentMethodState.Active]: 'green',
    [PaymentMethodState.Inactive]: 'gray',
};

export default function PaymentMethodsPage() {
    // State variables
    const [paymentMethods, setPaymentMethods] = useState<TableRow[]>([]);
    const [totalPaymentMethods, setTotalPaymentMethods] = useState(0);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);
    const [currentPage, setCurrentPage] = useState(1);
    const [pageSize, setPageSize] = useState(10);

    // Modal states
    const [isAddModalOpen, setIsAddModalOpen] = useState(false);
    const [isEditModalOpen, setIsEditModalOpen] = useState(false);
    const [isDeleteModalOpen, setIsDeleteModalOpen] = useState(false);
    const [selectedPaymentMethod, setSelectedPaymentMethod] = useState<PaymentMethod | null>(null);

    // Format payment method data for table
    const formatPaymentMethodData = (data: PaymentMethod[]): TableRow[] => {
        return data.map(item => ({
            ...item,
            status: item.state,
            formattedCreatedAt: formatDateYMD(item.createdAt)
        }));
    };

    // Data fetching function
    const fetchPaymentMethods = useCallback(async () => {
        try {
            setLoading(true);
            setError(null);

            const offset = (currentPage - 1) * pageSize;
            const result = await gql(GetPaymentMethodsDocument, { first: pageSize, offset });

            if (result.paymentMethods) {
                setPaymentMethods(formatPaymentMethodData(result.paymentMethods.map((item) => ({
                    ...item,
                    description: item.description || null,
                }))));
                setTotalPaymentMethods(result.totalPaymentMethods);
            }
        } catch (err) {
            setError('Failed to load payment methods. Please try again.');
            console.error('Error fetching payment methods:', err);
        } finally {
            setLoading(false);
        }
    }, [currentPage, pageSize]);

    // Load payment methods on page init and when pagination changes
    useEffect(() => {
        fetchPaymentMethods();
    }, [fetchPaymentMethods]);

    // Event handlers
    const handlePaymentMethodAdded = () => {
        fetchPaymentMethods();
        setIsAddModalOpen(false);
    };

    const handlePaymentMethodUpdated = () => {
        fetchPaymentMethods();
        setIsEditModalOpen(false);
        setSelectedPaymentMethod(null);
    };

    const handlePaymentMethodDeleted = () => {
        fetchPaymentMethods();
        setIsDeleteModalOpen(false);
        setSelectedPaymentMethod(null);
    };

    const headers = [
        { key: 'code', header: 'Code' },
        { key: 'name', header: 'Name' },
        { key: 'description', header: 'Description' },
        { key: 'status', header: 'Status' },
        { key: 'formattedCreatedAt', header: 'Created' }
    ];

    return (
        <Content className='min-h-[calc(100dvh-3rem)] p-0 flex flex-col'>
            <div className="p-4 flex-grow flex flex-col" style={{ minHeight: 'calc(100vh - 12rem)' }}>
                <DataTable<TableRow>
                    title="Payment Methods"
                    description="Manage payment methods for sales and purchase transactions in your POS system."
                    headers={headers}
                    tableRows={paymentMethods}
                    loading={loading}
                    totalItems={totalPaymentMethods}
                    currentPage={currentPage}
                    pageSize={pageSize}
                    pageSizes={[10, 20, 30, 40, 50]}
                    onPageChange={(page, size) => {
                        setCurrentPage(page);
                        setPageSize(size);
                    }}
                    onAddClick={() => {
                        setIsAddModalOpen(true);
                    }}
                    onEditClick={(paymentMethod: TableRow) => {
                        setSelectedPaymentMethod(paymentMethod);
                        setIsEditModalOpen(true);
                    }}
                    onDeleteClick={(paymentMethod: TableRow) => {
                        setSelectedPaymentMethod(paymentMethod);
                        setIsDeleteModalOpen(true);
                    }}
                />
            </div>

            <AddPaymentMethodModal
                isOpen={isAddModalOpen}
                onClose={() => setIsAddModalOpen(false)}
                onSave={handlePaymentMethodAdded}
            />

            {selectedPaymentMethod && (
                <>
                    <EditPaymentMethodModal
                        isOpen={isEditModalOpen}
                        paymentMethod={selectedPaymentMethod}
                        onClose={() => {
                            setIsEditModalOpen(false);
                            setSelectedPaymentMethod(null);
                        }}
                        onSave={handlePaymentMethodUpdated}
                    />

                    <DeletePaymentMethodModal
                        isOpen={isDeleteModalOpen}
                        paymentMethod={selectedPaymentMethod}
                        onClose={() => {
                            setIsDeleteModalOpen(false);
                            setSelectedPaymentMethod(null);
                        }}
                        onDelete={handlePaymentMethodDeleted}
                    />
                </>
            )}
        </Content>
    );
}
