'use client'
import React, { createContext, useState, useEffect, useCallback, useContext } from 'react'
import { drizzleDb } from '@/components/providers/system_provider'
import { Item, ItemCategory, Tax, ItemTax, itemsTable, itemCategoriesTable, taxesTable, itemTaxesTable } from '@/lib/pglite/schema'
import { eq } from 'drizzle-orm'
import { uid } from 'uid'

interface ItemWithTaxIds extends Item {
  taxIds: string;
}

interface ItemsContextType {
  itemsList: ItemWithTaxIds[];
  categories: ItemCategory[];
  taxesList: Tax[];
  loading: boolean;
  editingItem: Partial<ItemWithTaxIds> | null;
  isModalOpen: boolean;
  isDeleteModalOpen: boolean;
  currentPage: number;
  pageSize: number;
  fetchData: () => Promise<void>;
  handleSaveItem: (e: React.FormEvent) => Promise<void>;
  handleDeleteItem: () => Promise<void>;
  setEditingItem: React.Dispatch<React.SetStateAction<Partial<ItemWithTaxIds> | null>>;
  setIsModalOpen: React.Dispatch<React.SetStateAction<boolean>>;
  setIsDeleteModalOpen: React.Dispatch<React.SetStateAction<boolean>>;
  setCurrentPage: React.Dispatch<React.SetStateAction<number>>;
  setPageSize: React.Dispatch<React.SetStateAction<number>>;
}

const ItemsContext = createContext<ItemsContextType | undefined>(undefined)

export const ItemsProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [itemsList, setItemsList] = useState<ItemWithTaxIds[]>([])
  const [categories, setCategories] = useState<ItemCategory[]>([])
  const [taxesList, setTaxesList] = useState<Tax[]>([])
  const [loading, setLoading] = useState(true)
  const [editingItem, setEditingItem] = useState<Partial<ItemWithTaxIds> | null>(null)
  const [isModalOpen, setIsModalOpen] = useState(false)
  const [isDeleteModalOpen, setIsDeleteModalOpen] = useState(false)
  const [currentPage, setCurrentPage] = useState(1)
  const [pageSize, setPageSize] = useState(10)

  const fetchData = useCallback(async () => {
    setLoading(true)
    try {
      const itemsResult = await drizzleDb.select().from(itemsTable)
      const itemTaxesResult = await drizzleDb.select().from(itemTaxesTable)

      const itemsWithTaxIds: ItemWithTaxIds[] = itemsResult.map(item => ({
        ...item,
        taxIds: itemTaxesResult
          .filter(it => it.itemId === item.id)
          .map(it => it.taxId)
          .join(',')
      }))

      setItemsList(itemsWithTaxIds)
      const categoriesResult = await drizzleDb.select().from(itemCategoriesTable)
      setCategories(categoriesResult)
      const taxesResult = await drizzleDb.select().from(taxesTable)
      setTaxesList(taxesResult)
    } catch (error) {
      console.error('Error fetching data:', error)
    } finally {
      setLoading(false)
    }
  }, [])

  useEffect(() => {
    fetchData()
  }, [fetchData])

  const handleSaveItem = useCallback(async (e: React.FormEvent) => {
    e.preventDefault()
    if (!editingItem) return
    try {
      const itemData: Partial<Item> = {
        name: editingItem.name,
        description: editingItem.description,
        price: Number(editingItem.price),
        categoryId: editingItem.categoryId,
      }

      if (editingItem.id) {
        await drizzleDb.update(itemsTable)
          .set(itemData)
          .where(eq(itemsTable.id, editingItem.id))
          .execute()

        await drizzleDb.delete(itemTaxesTable)
          .where(eq(itemTaxesTable.itemId, editingItem.id))
          .execute()
      } else {
        const newItemId = uid()
        await drizzleDb.insert(itemsTable).values({ ...itemData, id: newItemId } as Item).execute()
        itemData.id = newItemId
      }

      if (editingItem.taxIds) {
        const taxIdsArray = editingItem.taxIds.split(',')
        for (const taxId of taxIdsArray) {
          await drizzleDb.insert(itemTaxesTable).values({
            id: uid(),
            itemId: itemData.id!,
            taxId: taxId,
          } as ItemTax).execute()
        }
      }

      setIsModalOpen(false)
      setEditingItem(null)
      fetchData()
    } catch (error) {
      console.error('Error saving item:', error)
    }
  }, [editingItem, fetchData])

  const handleDeleteItem = useCallback(async () => {
    if (!editingItem?.id) return
    try {
      await drizzleDb.delete(itemTaxesTable)
        .where(eq(itemTaxesTable.itemId, editingItem.id))
        .execute()
      await drizzleDb.delete(itemsTable)
        .where(eq(itemsTable.id, editingItem.id))
        .execute()
      setIsDeleteModalOpen(false)
      setEditingItem(null)
      fetchData()
    } catch (error) {
      console.error('Error deleting item:', error)
    }
  }, [editingItem, fetchData])

  return (
    <ItemsContext.Provider value={{
      itemsList,
      categories,
      taxesList,
      loading,
      editingItem,
      isModalOpen,
      isDeleteModalOpen,
      currentPage,
      pageSize,
      fetchData,
      handleSaveItem,
      handleDeleteItem,
      setEditingItem,
      setIsModalOpen,
      setIsDeleteModalOpen,
      setCurrentPage,
      setPageSize
    }}>
      {children}
    </ItemsContext.Provider>
  )
}

export const useItems = () => {
  const context = useContext(ItemsContext)
  if (context === undefined) {
    throw new Error('useItems must be used within an ItemsProvider')
  }
  return context
}
