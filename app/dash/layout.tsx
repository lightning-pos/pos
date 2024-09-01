'use client'
import {
  Content, Header, HeaderGlobalAction, HeaderGlobalBar, HeaderName, HeaderPanel,
  Switcher, SwitcherDivider, SwitcherItem, Theme
} from '@carbon/react'
import { Switcher as SwitcherIcon } from '@carbon/icons-react'
import { useState } from 'react';
import { useRouter } from 'next/navigation';

export default function DashboardLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  const [isSideNavExpanded, setIsSideNavExpanded] = useState(false)
  const router = useRouter()

  return (
    <Theme theme='g100'>
      <Header>
        <HeaderName prefix='MINNAL⚡'></HeaderName>
        <HeaderGlobalBar>
          <HeaderGlobalAction
            aria-label={isSideNavExpanded ? 'Close Main Menu' : 'Open Main Menu'}
            aria-expanded={isSideNavExpanded}
            isActive={isSideNavExpanded}
            onClick={() => { setIsSideNavExpanded(!isSideNavExpanded) }}
            tooltipAlignment="end">
            <SwitcherIcon />
          </HeaderGlobalAction>
        </HeaderGlobalBar>
        <HeaderPanel expanded={isSideNavExpanded} onHeaderPanelFocus={() => { setIsSideNavExpanded(!isSideNavExpanded) }}>
          <Switcher aria-label="Switcher Container" expanded={isSideNavExpanded}>
            <SwitcherItem aria-label="Point of Sale" onClick={() => { router.push('/dash/pos') }}>
              Point of Sale
            </SwitcherItem>
            <SwitcherItem aria-label="Catalog" onClick={() => { router.push('/dash/catalog') }}>
              Catalog Management
            </SwitcherItem>
            <SwitcherItem aria-label="Inventory">
              Inventory
            </SwitcherItem>
            <SwitcherItem aria-label="Customers">
              Customers
            </SwitcherItem>
            <SwitcherItem aria-label="Catalog">
              Settings
            </SwitcherItem>
            <SwitcherDivider />
            <SwitcherItem aria-label="Log Out" onClick={() => { router.push('/login') }}>
              Log Out
            </SwitcherItem>
          </Switcher>
        </HeaderPanel>
      </Header>
      {children}
    </Theme >
  );
}
