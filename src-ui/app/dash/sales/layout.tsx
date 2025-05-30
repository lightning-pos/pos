'use client'
import React from 'react'
import { SideNav, SideNavItems, SideNavLink } from '@carbon/react'
import { useRouter } from 'next/navigation'
import { Dashboard, Receipt, PricingConsumption } from '@carbon/icons-react'

export default function OrdersLayout({
    children,
}: {
    children: React.ReactNode
}) {
    const router = useRouter()

    return (
        <>
            <SideNav isFixedNav expanded={true} isChildOfHeader={false} aria-label="Orders navigation">
                <SideNavItems>
                    <SideNavLink renderIcon={Dashboard} large onClick={() => { router.push('/dash/sales') }}>
                        Overview
                    </SideNavLink>
                    <SideNavLink renderIcon={Receipt} large onClick={() => { router.push('/dash/sales/orders') }}>
                        Sales Orders
                    </SideNavLink>
                    <SideNavLink renderIcon={PricingConsumption} large onClick={() => { router.push('/dash/sales/charge-types') }}>
                        Charge Types
                    </SideNavLink>
                </SideNavItems>
            </SideNav>
            {children}
        </>
    )
}
