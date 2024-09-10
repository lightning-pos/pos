'use client'
import React from 'react'
import { SideNav, SideNavItems, SideNavLink } from '@carbon/react'
import { useRouter } from 'next/navigation'
import { Dashboard, UserProfile } from '@carbon/icons-react'

export default function CustomersLayout({
  children,
}: {
  children: React.ReactNode
}) {
  const router = useRouter()

  return (
    <>
      <SideNav isFixedNav expanded={true} isChildOfHeader={false} aria-label="Customer navigation">
        <SideNavItems>
          <SideNavLink renderIcon={Dashboard} large onClick={() => { router.push('/dash/customers') }}>
            Overview
          </SideNavLink>
          <SideNavLink renderIcon={UserProfile} large onClick={() => { router.push('/dash/customers/segmentation') }}>
            Segmentation
          </SideNavLink>
        </SideNavItems>
      </SideNav>
      {children}
    </>
  )
}
