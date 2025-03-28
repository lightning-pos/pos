'use client'
import React from 'react'
import { SideNav, SideNavItems, SideNavLink } from '@carbon/react'
import { useRouter } from 'next/navigation'
import { Dashboard, Money } from '@carbon/icons-react'

export default function PurchasesLayout({
    children,
}: {
    children: React.ReactNode
}) {
    const router = useRouter()

    return (
        <>
            <SideNav isFixedNav expanded={true} isChildOfHeader={false} aria-label="Purchases navigation">
                <SideNavItems>
                    <SideNavLink renderIcon={Dashboard} large onClick={() => { router.push('/dash/purchases') }}>
                        Overview
                    </SideNavLink>
                    <SideNavLink renderIcon={Money} large onClick={() => { router.push('/dash/purchases/expenses') }}>
                        Expenses
                    </SideNavLink>
                </SideNavItems>
            </SideNav>
            {children}
        </>
    )
}
