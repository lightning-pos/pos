'use client'
import React from 'react'
import { SideNav, SideNavItems, SideNavLink } from '@carbon/react'
import { useRouter } from 'next/navigation'
import { Dashboard, DocumentAdd } from '@carbon/icons-react'

export default function SuppliersLayout({
    children,
}: {
    children: React.ReactNode
}) {
    const router = useRouter()

    return (
        <>
            <SideNav isFixedNav expanded={true} isChildOfHeader={false} aria-label="Supplier navigation">
                <SideNavItems>
                    <SideNavLink renderIcon={Dashboard} large onClick={() => { router.push('/dash/suppliers/overview') }}>
                        Overview
                    </SideNavLink>
                    <SideNavLink renderIcon={DocumentAdd} large onClick={() => { router.push('/dash/suppliers/orders') }}>
                        Purchase Orders
                    </SideNavLink>
                </SideNavItems>
            </SideNav>
            {children}
        </>
    )
}
