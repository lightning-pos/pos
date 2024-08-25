'use client'
import { Content, Header, HeaderName, Theme } from '@carbon/react'
import React from 'react'

const Dashboard = () => {
    return (
        <Theme theme='g90'>
            <Header>
                <HeaderName prefix='MINNAL⚡'>Point of Sale</HeaderName>
            </Header>
            <Content className='h-[calc(100dvh-3rem)] p-0'>
                <div className="grid grid-cols-12 gap-4">
                    <div className='col-span-2 p-4 border-2' style={{ borderColor: 'var(--cds-border-subtle-00)' }}>Category</div>
                    <div className='col-span-7 p-4 border'>Item</div>
                    <div className='col-span-3 p-4 border'>Cart</div>
                </div>
            </Content>
        </Theme>
    )
}

export default Dashboard
