'use client'
import { Analytics as AnalyticsIcon, ChartStepper, ChartWinLoss } from '@carbon/icons-react'
import { Content, SideNav, SideNavItems, SideNavLink } from '@carbon/react'

const Analytics = () => {
    return (
        <>
            <SideNav isFixedNav expanded={true} isChildOfHeader={false} aria-label="Side navigation">
                <SideNavItems>
                    <SideNavLink renderIcon={AnalyticsIcon} large href='#'>Overview</SideNavLink>
                    <SideNavLink renderIcon={ChartWinLoss} large href='#'>Sales Report</SideNavLink>
                    <SideNavLink renderIcon={ChartStepper} large href='#'>Stock Report</SideNavLink>
                </SideNavItems>
            </SideNav>
            <Content className='min-h-[calc(100dvh-3rem)] p-4'>Analytics</Content>
        </>
    )
}

export default Analytics
