'use client'
import { Content, SideNav, SideNavItems, SideNavLink } from '@carbon/react'

const Settings = () => {
  return (
    <>
      <SideNav isFixedNav expanded={true} isChildOfHeader={false} aria-label="Side navigation">
        <SideNavItems>
          <SideNavLink large href='#'>POS Settings</SideNavLink>
          <SideNavLink large href='#'>Printer Settings</SideNavLink>
          <SideNavLink large href='#'>Channels</SideNavLink>
          <SideNavLink large href='#'>Taxes</SideNavLink>
        </SideNavItems>
      </SideNav>
      <Content className='min-h-[calc(100dvh-3rem)] p-4'>Settings</Content>
    </>
  )
}

export default Settings
