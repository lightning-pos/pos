'use client'
import React from 'react'
import { Content, Tabs, Tab, TabList, TabPanels, TabPanel } from '@carbon/react'

const TestPage = () => {
    return (
        <Content className="min-h-[calc(100dvh-3rem)] p-4 flex flex-col">
            <h1>Test Page</h1>
            
            <Tabs>
                <TabList aria-label="Test Tabs">
                    <Tab>Tab 1</Tab>
                    <Tab>Tab 2</Tab>
                    <Tab>Tab 3</Tab>
                </TabList>
                <TabPanels>
                    <TabPanel>
                        <div className="mt-4">
                            <h3>Content for Tab 1</h3>
                            <p>This is the content for tab 1.</p>
                        </div>
                    </TabPanel>
                    <TabPanel>
                        <div className="mt-4">
                            <h3>Content for Tab 2</h3>
                            <p>This is the content for tab 2.</p>
                        </div>
                    </TabPanel>
                    <TabPanel>
                        <div className="mt-4">
                            <h3>Content for Tab 3</h3>
                            <p>This is the content for tab 3.</p>
                        </div>
                    </TabPanel>
                </TabPanels>
            </Tabs>
        </Content>
    )
}

export default TestPage
