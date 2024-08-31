'use client'
import { useQuery } from '@powersync/react'
import React from 'react'

const POS = () => {
  const { data: categories } = useQuery('select * from categories')
  return (
    <div className="grid grid-cols-12 gap-4 h-[calc(100dvh-3rem)]">
      <div className='col-span-2 p-4 border'>
        Category
        <br />
        {
          categories.map((category) => (
            <div key={category.id}>{category.name}<br /></div>
          ))
        }
      </div>
      <div className='col-span-7 p-4 border'>Item</div>
      <div className='col-span-3 p-4 border'>Cart</div>
    </div>
  )
}

export default POS
