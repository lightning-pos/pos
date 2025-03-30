'use client'
import React from 'react'
import { SideNav, SideNavItems, SideNavLink } from '@carbon/react'
import { useRouter } from 'next/navigation'
import { Dashboard, Money, FolderDetails, Receipt } from '@carbon/icons-react'

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
                    <SideNavLink renderIcon={Receipt} large onClick={() => { router.push('/dash/purchases/orders') }}>
                        Orders
                    </SideNavLink>
                    <SideNavLink renderIcon={Money} large onClick={() => { router.push('/dash/purchases/expenses') }}>
                        Expenses
                    </SideNavLink>
                    <SideNavLink renderIcon={FolderDetails} large onClick={() => { router.push('/dash/purchases/categories') }}>
                        Categories
                    </SideNavLink>
                </SideNavItems>
            </SideNav>
            {children}
        </>
    )
}
