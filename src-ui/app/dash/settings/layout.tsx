'use client'
import { Content, SideNav, SideNavItems, SideNavLink } from '@carbon/react'
import { ReactNode } from 'react'
import { useRouter } from 'next/navigation'
import { Settings, Printer, Network_2, Currency } from '@carbon/icons-react'

const SettingsLayout = ({ children }: { children: ReactNode }) => {
  const router = useRouter()

  return (
    <>
      <SideNav isFixedNav expanded={true} isChildOfHeader={false} aria-label="Side navigation">
        <SideNavItems>
          <SideNavLink renderIcon={Settings} large onClick={() => { router.push('/dash/settings') }}>
            POS Settings
          </SideNavLink>
          <SideNavLink renderIcon={Printer} large onClick={() => { router.push('/dash/settings/printer') }}>
            Printer Settings
          </SideNavLink>
          <SideNavLink renderIcon={Network_2} large onClick={() => { router.push('/dash/settings/channels') }}>
            Channels
          </SideNavLink>
          <SideNavLink renderIcon={Currency} large onClick={() => { router.push('/dash/settings/taxes') }}>
            Taxes
          </SideNavLink>
        </SideNavItems>
      </SideNav>
      <Content className='min-h-[calc(100dvh-3rem)] p-4'>{children}</Content>
    </>
  )
}

export default SettingsLayout