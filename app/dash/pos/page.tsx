'use client'
import { db } from '@/components/providers/system_provider'
import { Content } from '@carbon/react'
import { useQuery } from '@powersync/react'
import { use } from 'react'
import { uid } from 'uid'

const POS = () => {
  // use(db.insertInto('item_categories').values({ id: uid(), name: 'Steamed - ' + uid() }).execute())
  // use(db.deleteFrom('item_categories').execute())
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
