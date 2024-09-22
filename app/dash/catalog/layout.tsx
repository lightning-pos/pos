'use client'
import React from 'react'
import { Category, Catalog as CatalogIcon, PricingConsumption, ShoppingCatalog } from '@carbon/icons-react'
import { SideNav, SideNavItems, SideNavLink } from '@carbon/react'
import { useRouter } from 'next/navigation'

export default function CatalogLayout({
  children,
}: {
  children: React.ReactNode
}) {
  const router = useRouter()
  return (
    <>
      <SideNav isFixedNav expanded={true} isChildOfHeader={false} aria-label="Side navigation">
        <SideNavItems>
          <SideNavLink renderIcon={Category} large onClick={() => { router.push('/dash/catalog/categories') }}>
            Categories
          </SideNavLink>
          <SideNavLink renderIcon={CatalogIcon} large onClick={() => { router.push('/dash/catalog/items') }}>
            Base Menu
          </SideNavLink>
          <SideNavLink renderIcon={ShoppingCatalog} large onClick={() => { router.push('/dash/catalog/display-menu') }}>
            Display Menu
          </SideNavLink>
          <SideNavLink renderIcon={PricingConsumption} large onClick={() => { router.push('/dash/catalog/charges') }}>
            Charges
          </SideNavLink>
        </SideNavItems>
      </SideNav>
      {children}
    </>
  )
}
