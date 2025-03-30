'use client';

import React, { useState, useEffect, useCallback } from 'react';
import { Content } from '@carbon/react';
import DataTable from '@/components/ui/DataTable';
import { formatDateYMD } from '@/lib/util/date_format';
import { gql } from '@/lib/graphql/execute';
import { GetCostCentersDocument, CostCenterState, CostCenter } from '@/lib/graphql/graphql';
import AddCostCenterModal from '@/app/dash/settings/cost-centers/add_cost_center_modal';
import EditCostCenterModal from '@/app/dash/settings/cost-centers/edit_cost_center_modal';
import DeleteCostCenterModal from '@/app/dash/settings/cost-centers/delete_cost_center_modal';

// Define the table row structure
interface TableRow extends CostCenter {
    status: string;
    formattedCreatedAt: string;
}

// Define the state tag colors based on the state value
const stateTagColors = {
    [CostCenterState.Active]: 'green',
    [CostCenterState.Inactive]: 'gray',
};

export default function CostCentersPage() {
    // State variables
    const [costCenters, setCostCenters] = useState<TableRow[]>([]);
    const [totalCostCenters, setTotalCostCenters] = useState(0);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);
    const [currentPage, setCurrentPage] = useState(1);
    const [pageSize, setPageSize] = useState(10);

    // Modal states
    const [isAddModalOpen, setIsAddModalOpen] = useState(false);
    const [isEditModalOpen, setIsEditModalOpen] = useState(false);
    const [isDeleteModalOpen, setIsDeleteModalOpen] = useState(false);
    const [selectedCostCenter, setSelectedCostCenter] = useState<CostCenter | null>(null);

    // Format cost center data for table
    const formatCostCenterData = (data: CostCenter[]): TableRow[] => {
        return data.map(item => ({
            ...item,
            status: item.state,
            formattedCreatedAt: formatDateYMD(item.createdAt)
        }));
    };

    // Data fetching function
    const fetchCostCenters = useCallback(async () => {
        try {
            setLoading(true);
            setError(null);

            const offset = (currentPage - 1) * pageSize;
            const result = await gql(GetCostCentersDocument, { first: pageSize, offset });

            if (result.costCenters) {
                setCostCenters(formatCostCenterData(result.costCenters.map((item) => ({
                    ...item,
                    description: item.description || null,
                }))));
                setTotalCostCenters(result.totalCostCenters);
            }
        } catch (err) {
            setError('Failed to load cost centers. Please try again.');
            console.error('Error fetching cost centers:', err);
        } finally {
            setLoading(false);
        }
    }, [currentPage, pageSize]);

    // Load cost centers on page init and when pagination changes
    useEffect(() => {
        fetchCostCenters();
    }, [fetchCostCenters]);

    // Event handlers
    const handleCostCenterAdded = () => {
        fetchCostCenters();
        setIsAddModalOpen(false);
    };

    const handleCostCenterUpdated = () => {
        fetchCostCenters();
        setIsEditModalOpen(false);
        setSelectedCostCenter(null);
    };

    const handleCostCenterDeleted = () => {
        fetchCostCenters();
        setIsDeleteModalOpen(false);
        setSelectedCostCenter(null);
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
                    title="Cost Centers"
                    description="Manage cost centers to track expenses and allocate costs to different departments or projects."
                    headers={headers}
                    tableRows={costCenters}
                    loading={loading}
                    totalItems={totalCostCenters}
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
                    onEditClick={(costCenter: TableRow) => {
                        setSelectedCostCenter(costCenter);
                        setIsEditModalOpen(true);
                    }}
                    onDeleteClick={(costCenter: TableRow) => {
                        setSelectedCostCenter(costCenter);
                        setIsDeleteModalOpen(true);
                    }}
                />
            </div>

            <AddCostCenterModal
                isOpen={isAddModalOpen}
                onClose={() => setIsAddModalOpen(false)}
                onSave={handleCostCenterAdded}
            />

            {selectedCostCenter && (
                <>
                    <EditCostCenterModal
                        isOpen={isEditModalOpen}
                        costCenter={selectedCostCenter}
                        onClose={() => {
                            setIsEditModalOpen(false);
                            setSelectedCostCenter(null);
                        }}
                        onSave={handleCostCenterUpdated}
                    />

                    <DeleteCostCenterModal
                        isOpen={isDeleteModalOpen}
                        costCenter={selectedCostCenter}
                        onClose={() => {
                            setIsDeleteModalOpen(false);
                            setSelectedCostCenter(null);
                        }}
                        onDelete={handleCostCenterDeleted}
                    />
                </>
            )}
        </Content>
    );
}
