'use client'
import React from 'react'
import { Category, Catalog as CatalogIcon, PricingConsumption, ShoppingCatalog, Branch, PiggyBank, ShoppingCartPlus, ShoppingCart } from '@carbon/icons-react'
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
                    <SideNavLink renderIcon={ShoppingCart} large onClick={() => { router.push('/dash/catalog/display-menu') }}>
                        Display Menu
                    </SideNavLink>
                    <SideNavLink renderIcon={ShoppingCartPlus} large onClick={() => { router.push('/dash/catalog/addons') }}>
                        Addons
                    </SideNavLink>
                    <SideNavLink renderIcon={Branch} large onClick={() => { router.push('/dash/catalog/brands') }}>
                        Brands
                    </SideNavLink>
                    <SideNavLink renderIcon={PiggyBank} large onClick={() => { router.push('/dash/catalog/discounts') }}>
                        Discounts
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
