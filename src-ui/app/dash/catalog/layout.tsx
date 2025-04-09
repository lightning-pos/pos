'use client'
import React from 'react'
import { Category, Catalog as CatalogIcon, PricingConsumption, ShoppingCatalog, Branch, PiggyBank, ShoppingCartPlus, ShoppingCart, TagGroup } from '@carbon/icons-react'
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
                        Items
                    </SideNavLink>
                    <SideNavLink renderIcon={TagGroup} large onClick={() => { router.push('/dash/catalog/variants') }}>
                        Variants
                    </SideNavLink>
                    <SideNavLink renderIcon={ShoppingCartPlus} large onClick={() => { router.push('/dash/catalog/addons') }}>
                        Addons
                    </SideNavLink>
                    <SideNavLink renderIcon={PiggyBank} large onClick={() => { router.push('/dash/catalog/discounts') }}>
                        Discounts
                    </SideNavLink>
                </SideNavItems>
            </SideNav>
            {children}
        </>
    )
}
