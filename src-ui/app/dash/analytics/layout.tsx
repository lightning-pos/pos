'use client'
import React from 'react'
import { Content, SideNav, SideNavItems, SideNavLink } from '@carbon/react'
import { useRouter } from 'next/navigation'
import { Analytics as AnalyticsIcon, ChartWinLoss, ChartStepper } from '@carbon/icons-react'

export default function AnalyticsLayout({
    children,
}: {
    children: React.ReactNode
}) {
    const router = useRouter()

    return (
        <>
            <SideNav isFixedNav expanded={true} isChildOfHeader={false} aria-label="Analytics navigation">
                <SideNavItems>
                    <SideNavLink renderIcon={AnalyticsIcon} large onClick={() => { router.push('/dash/analytics/overview') }}>
                        Overview
                    </SideNavLink>
                    <SideNavLink renderIcon={ChartWinLoss} large onClick={() => { router.push('/dash/analytics/sales-report') }}>
                        Sales Report
                    </SideNavLink>
                    <SideNavLink renderIcon={ChartStepper} large onClick={() => { router.push('/dash/analytics/stock-report') }}>
                        Stock Report
                    </SideNavLink>
                </SideNavItems>
            </SideNav>
            <Content className='min-h-[calc(100dvh-3rem)] p-4'>{children}</Content>
        </>
    )
}
