'use client'
import { Category, Catalog as CatalogIcon } from '@carbon/icons-react'
import { Content, SideNav, SideNavItems, SideNavLink } from '@carbon/react'

const Catalog = () => {
  return (
    <>
      <SideNav isFixedNav expanded={true} isChildOfHeader={false} aria-label="Side navigation">
        <SideNavItems>
          <SideNavLink renderIcon={CatalogIcon} large href='#'>Catalog Items</SideNavLink>
          <SideNavLink renderIcon={Category} large href='#'>Item Categories</SideNavLink>
        </SideNavItems>
      </SideNav>
      <Content className='min-h-[calc(100dvh-3rem)] p-0'>
        <div className="p-4">Content</div>
      </Content>
    </>
  )
}

export default Catalog
