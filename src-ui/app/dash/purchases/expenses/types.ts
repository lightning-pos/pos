export interface TableRow {
    id: string;
    title: string;
    amount: string;
    expenseDate: string;
    categoryId: string;
    costCenterId: string;
    description?: string | null;
    createdAt: string;
    updatedAt: string;
    formattedDate: string;
    category: string;
    costCenter: string;
}
