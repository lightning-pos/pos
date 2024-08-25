'use client'
import { Button, Theme } from '@carbon/react'
import React from 'react'

const Login = () => {
    return (
        <div className="grid grid-cols-12 gap-4">
            <div className="col-span-8 bg-[url('/login.jpg')] bg-cover bg-center bg-no-repeat"></div>
            <div className="col-span-4">
                <div className="flex h-screen">
                    <div className="my-auto p-4">
                        <Button> Log In</Button>
                    </div>
                </div>
            </div>
        </div >
    )
}

export default Login
