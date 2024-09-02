'use client'
import { Category, Catalog as CatalogIcon, PricingConsumption, ShoppingCatalog } from '@carbon/icons-react'
import { Content, SideNav, SideNavItems, SideNavLink } from '@carbon/react'

const Catalog = () => {
  return (
    <>
      <SideNav isFixedNav expanded={true} isChildOfHeader={false} aria-label="Side navigation">
        <SideNavItems>
          <SideNavLink renderIcon={Category} large href='#'>Categories</SideNavLink>
          <SideNavLink renderIcon={CatalogIcon} large href='#'>Base Menu</SideNavLink>
          <SideNavLink renderIcon={ShoppingCatalog} large href='#'>Display Menu</SideNavLink>
          <SideNavLink renderIcon={PricingConsumption} large href='#'>Charges</SideNavLink>
        </SideNavItems>
      </SideNav>
      <Content className='min-h-[calc(100dvh-3rem)] p-0'>
        <div className="p-4">Content</div>
      </Content>
    </>
  )
}

export default Catalog
