'use client'
import { db } from '@/components/providers/system_provider'
import { Content } from '@carbon/react'
import { use } from 'react'

const POS = () => {
  const categories = use(db.selectFrom('item_categories').selectAll().execute())

  return (
    <Content className='min-h-[calc(100dvh-3rem)] p-0'>
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
    </Content>
  )
}

export default POS
