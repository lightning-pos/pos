'use client'
import { useState, useCallback, useEffect } from 'react'
import { Content } from '@carbon/react'
import DataTable from '@/components/ui/DataTable'
import {
  GetSalesChargeTypesDocument,
  SalesChargeType,
  SalesChargeTypeNewInput,
  SalesChargeTypeUpdateInput,
  CreateSalesChargeTypeDocument,
  UpdateSalesChargeTypeDocument,
  DeleteSalesChargeTypeDocument
} from '@/lib/graphql/graphql'
import { gql } from '@/lib/graphql/execute'
import AddChargeTypeModal from '@/app/dash/sales/charge-types/add_charge_type_modal'
import EditChargeTypeModal from '@/app/dash/sales/charge-types/edit_charge_type_modal'
import DeleteChargeTypeModal from '@/app/dash/sales/charge-types/delete_charge_type_modal'

export default function ChargeTypesPage() {
  const [chargeTypes, setChargeTypes] = useState<SalesChargeType[]>([])
  const [totalItems, setTotalItems] = useState(0)
  const [currentPage, setCurrentPage] = useState(1)
  const [pageSize, setPageSize] = useState(10)
  const [loading, setLoading] = useState(false)

  // Modal states
  const [isAddModalOpen, setIsAddModalOpen] = useState(false)
  const [isEditModalOpen, setIsEditModalOpen] = useState(false)
  const [isDeleteModalOpen, setIsDeleteModalOpen] = useState(false)
  const [selectedChargeType, setSelectedChargeType] = useState<SalesChargeType | null>(null)

  const fetchChargeTypes = useCallback(async (page: number, size: number) => {
    setLoading(true)
    try {
      const offset = (page - 1) * size
      const result = await gql(GetSalesChargeTypesDocument, { first: size, offset })
      setChargeTypes(result.salesChargeTypes)
      setTotalItems(result.salesChargeTypesCount)
    } catch (error) {
      console.error('Error fetching charge types:', error)
    } finally {
      setLoading(false)
    }
  }, [])

  useEffect(() => {
    fetchChargeTypes(currentPage, pageSize)
  }, [fetchChargeTypes, currentPage, pageSize])

  const headers = [
    { key: 'name', header: 'Name' },
    { key: 'description', header: 'Description' }
  ]

  const handleAddChargeType = async (chargeType: SalesChargeTypeNewInput) => {
    try {
      await gql(CreateSalesChargeTypeDocument, { input: chargeType })
      fetchChargeTypes(currentPage, pageSize)
      setIsAddModalOpen(false)
    } catch (error) {
      console.error('Error adding charge type:', error)
    }
  }

  const handleEditChargeType = async (chargeType: SalesChargeTypeUpdateInput) => {
    try {
      await gql(UpdateSalesChargeTypeDocument, { input: chargeType })
      fetchChargeTypes(currentPage, pageSize)
      setIsEditModalOpen(false)
      setSelectedChargeType(null)
    } catch (error) {
      console.error('Error updating charge type:', error)
    }
  }

  const handleDeleteChargeType = async () => {
    if (!selectedChargeType) return

    try {
      await gql(DeleteSalesChargeTypeDocument, { id: selectedChargeType.id })
      fetchChargeTypes(currentPage, pageSize)
      setIsDeleteModalOpen(false)
      setSelectedChargeType(null)
    } catch (error) {
      console.error('Error deleting charge type:', error)
    }
  }

  return (
    <Content className='min-h-[calc(100dvh-3rem)] p-0 flex flex-col'>
      <div className="p-4 flex-grow flex flex-col" style={{ minHeight: 'calc(100vh - 12rem)' }}>
        <DataTable<SalesChargeType>
          title="Charge Types"
          description="Manage your sales charge types here. These are additional charges that can be applied to sales orders."
          headers={headers}
          tableRows={chargeTypes}
          loading={loading}
          totalItems={totalItems}
          currentPage={currentPage}
          pageSize={pageSize}
          pageSizes={[10, 20, 30, 40, 50]}
          onPageChange={(page, size) => {
            setCurrentPage(page)
            setPageSize(size)
            fetchChargeTypes(page, size)
          }}
          onAddClick={() => setIsAddModalOpen(true)}
          onEditClick={(chargeType) => {
            setSelectedChargeType(chargeType)
            setIsEditModalOpen(true)
          }}
          onDeleteClick={(chargeType) => {
            setSelectedChargeType(chargeType)
            setIsDeleteModalOpen(true)
          }}
        />
      </div>

      <AddChargeTypeModal
        isOpen={isAddModalOpen}
        onClose={() => setIsAddModalOpen(false)}
        onSave={handleAddChargeType}
      />

      {selectedChargeType && (
        <>
          <EditChargeTypeModal
            isOpen={isEditModalOpen}
            chargeType={selectedChargeType}
            onClose={() => {
              setIsEditModalOpen(false)
              setSelectedChargeType(null)
            }}
            onSave={handleEditChargeType}
          />

          <DeleteChargeTypeModal
            isOpen={isDeleteModalOpen}
            chargeTypeId={selectedChargeType.id}
            chargeTypeName={selectedChargeType.name}
            onClose={() => {
              setIsDeleteModalOpen(false)
              setSelectedChargeType(null)
            }}
            onDelete={handleDeleteChargeType}
          />
        </>
      )}
    </Content>
  )
}
