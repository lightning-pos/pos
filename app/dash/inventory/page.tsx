'use client'
import { Bottles_01, ChartAreaStepper, TaskLocation, UserMultiple } from '@carbon/icons-react'
import { Content, SideNav, SideNavItems, SideNavLink } from '@carbon/react'
import React from 'react'

const Inventory = () => {
  return (
    <>
      <SideNav isFixedNav expanded={true} isChildOfHeader={false} aria-label="Side navigation">
        <SideNavItems>
          <SideNavLink renderIcon={Bottles_01} large href='#'>Stock</SideNavLink>
          <SideNavLink renderIcon={ChartAreaStepper} large href='#'>Activity</SideNavLink>
          <SideNavLink renderIcon={TaskLocation} large href='#'>Locations</SideNavLink>
          <SideNavLink renderIcon={UserMultiple} large href='#'>Suppliers</SideNavLink>
        </SideNavItems>
      </SideNav>
      <Content className='min-h-[calc(100dvh-3rem)] p-4'>Inventory</Content>
    </>
  )
}

export default Inventory
