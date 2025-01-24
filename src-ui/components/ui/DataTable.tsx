import React from 'react'
import { Add } from '@carbon/icons-react'
import { DataTable as CarbonDataTable, Table, TableHead, TableRow, TableHeader, TableBody, TableCell, Pagination, DataTableSkeleton, Button, TableToolbar, TableToolbarContent, TableContainer, OverflowMenu, OverflowMenuItem } from '@carbon/react'

interface DataTableProps<T> {
  title: string
  description: string
  headers: { key: string; header: string }[]
  tableRows: T[]
  loading: boolean
  totalItems: number
  currentPage: number
  pageSize: number
  pageSizes: number[]
  onPageChange: (page: number, pageSize: number) => void
  onAddClick: () => void
  onEditClick: (item: T) => void
  onDeleteClick: (item: T) => void
}

function DataTable<T extends { id: string }>({
  title,
  description,
  headers,
  tableRows,
  loading,
  totalItems,
  currentPage,
  pageSize,
  pageSizes,
  onPageChange,
  onAddClick,
  onEditClick,
  onDeleteClick
}: DataTableProps<T>) {
  if (loading) {
    return <DataTableSkeleton headers={headers} rowCount={pageSize} />
  }

  return (
    <TableContainer title={title} description={description}>
      <TableToolbar>
        <TableToolbarContent>
          <Button renderIcon={Add} onClick={onAddClick}>
            Add {title}
          </Button>
        </TableToolbarContent>
      </TableToolbar>
      <CarbonDataTable rows={tableRows} headers={headers}>
        {({ rows, headers, getTableProps }) => (
          <Table {...getTableProps()}>
            <TableHead>
              <TableRow>
                {headers.map((header) => (
                  <TableHeader key={header.key}>{header.header}</TableHeader>
                ))}
                <TableHeader key="actions" style={{ width: '8rem' }}>Actions</TableHeader>
              </TableRow>
            </TableHead>
            <TableBody>
              {rows.map((row) => (
                <TableRow key={row.id}>
                  {row.cells.map((cell) => (
                    <TableCell key={cell.id}>{cell.value}</TableCell>
                  ))}
                  <TableCell>
                    <OverflowMenu label="Actions">
                      <OverflowMenuItem
                        itemText="Edit"
                        onClick={() => {
                          onEditClick(tableRows.find((r) => r.id === row.id) as T);
                        }}
                      />
                      <OverflowMenuItem
                        itemText="Delete"
                        hasDivider
                        isDelete
                        onClick={() => {
                          onDeleteClick(tableRows.find((r) => r.id === row.id) as T);
                        }}
                      />
                    </OverflowMenu>
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        )}
      </CarbonDataTable>
      <Pagination
        totalItems={totalItems}
        backwardText="Previous page"
        forwardText="Next page"
        pageSize={pageSize}
        pageSizes={pageSizes}
        itemsPerPageText="Items per page:"
        page={currentPage}
        onChange={({ page, pageSize }) => onPageChange(page, pageSize)}
      />
    </TableContainer>
  )
}

export default DataTable
