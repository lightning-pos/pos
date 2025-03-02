'use client'
import { Content, SideNav, SideNavItems, SideNavLink, SideNavMenu, SideNavMenuItem } from '@carbon/react'
import { ReactNode } from 'react'
import { useRouter } from 'next/navigation'
import { Bottles_01, ChartAreaStepper, TaskLocation } from '@carbon/icons-react'

const InventoryLayout = ({ children }: { children: ReactNode }) => {
    const router = useRouter()

    return (
        <>
            <SideNav isFixedNav expanded={true} isChildOfHeader={false} aria-label="Side navigation">
                <SideNavItems>
                    <SideNavLink renderIcon={Bottles_01} large href='#'>Stock</SideNavLink>
                    <SideNavLink renderIcon={ChartAreaStepper} large href='#'>Activity</SideNavLink>
                    <SideNavLink renderIcon={TaskLocation} large href='#'>Locations</SideNavLink>
                </SideNavItems>
            </SideNav>
            <Content className='min-h-[calc(100dvh-3rem)] p-4'>{children}</Content>
        </>
    )
}

export default InventoryLayout
