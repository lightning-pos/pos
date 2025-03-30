'use client'
import {
    Content, Header, HeaderGlobalAction, HeaderGlobalBar, HeaderName, HeaderPanel,
    Switcher, SwitcherDivider, SwitcherItem, Theme
} from '@carbon/react'
import { Switcher as SwitcherIcon } from '@carbon/icons-react'
import { useState } from 'react';
import { useRouter } from 'next/navigation';
import { useHotkeys } from 'react-hotkeys-hook';

export default function DashboardLayout({
    children,
}: Readonly<{
    children: React.ReactNode;
}>) {
    const router = useRouter()
    const [isSideNavExpanded, setIsSideNavExpanded] = useState(false)

    useHotkeys('f1', () => {
        setIsSideNavExpanded(false)
        router.push('/dash/pos')
    })

    useHotkeys('esc', () => {
        setIsSideNavExpanded(false)
    })

    return (
        <Theme theme='g90'>
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
                <HeaderPanel expanded={isSideNavExpanded}>
                    <Switcher aria-label="Switcher Container" expanded={isSideNavExpanded} >
                        <SwitcherItem aria-label="POS" onClick={() => { router.push('/dash/pos') }}>Point of Sale</SwitcherItem>
                        <SwitcherItem aria-label="Sales" onClick={() => { router.push('/dash/sales') }}>Sales</SwitcherItem>
                        <SwitcherItem aria-label="Purchases" onClick={() => { router.push('/dash/purchases') }}>Purchases</SwitcherItem>
                        <SwitcherItem aria-label="Catalog" onClick={() => { router.push('/dash/catalog/categories') }}>Catalog</SwitcherItem>
                        <SwitcherItem aria-label="Inventory" onClick={() => { router.push('/dash/inventory') }}>Inventory</SwitcherItem>
                        <SwitcherItem aria-label="Analytics" onClick={() => { router.push('/dash/analytics/overview') }}>Analytics</SwitcherItem>
                        <SwitcherDivider />
                        <SwitcherItem aria-label="Customers" onClick={() => { router.push('/dash/customers/overview') }}>Customers</SwitcherItem>
                        <SwitcherItem aria-label="Suppliers" onClick={() => { router.push('/dash/suppliers/overview') }}>Suppliers</SwitcherItem>
                        <SwitcherItem aria-label="Users" onClick={() => { router.push('/dash/users') }}>Users</SwitcherItem>
                        <SwitcherDivider />
                        <SwitcherItem aria-label="Settings" onClick={() => router.push('/dash/settings')}>Settings</SwitcherItem>
                        <SwitcherItem aria-label="Log Out" onClick={() => { router.push('/login') }}>Log Out</SwitcherItem>
                    </Switcher>
                </HeaderPanel>
            </Header>
            {children}
        </Theme >
    );
}
