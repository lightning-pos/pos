'use client'
import { Receipt } from '@carbon/icons-react'
import { Content, SideNav, SideNavItems, SideNavLink } from '@carbon/react'
import React from 'react'

const Orders = () => {
  return (
    <>
      <SideNav isFixedNav expanded={true} isChildOfHeader={false} aria-label="Side navigation">
        <SideNavItems>
          <SideNavLink renderIcon={Receipt} large href='#'>Overview</SideNavLink>
        </SideNavItems>
      </SideNav>
      <Content className='min-h-[calc(100dvh-3rem)] p-0'>
        <div className="p-4">Orders</div>
      </Content>
    </>
  )
}

export default Orders
