'use client'
import { Button, TextInput, Form, InlineNotification } from '@carbon/react'
import { useRouter } from 'next/navigation'
import React, { useState } from 'react'
import { gql } from '../../lib/graphql/execute'
import { LoginDocument } from '../../lib/graphql/graphql'

const Login = () => {
    const router = useRouter()
    const [username, setUsername] = useState('test')
    const [password, setPassword] = useState('test')
    const [loading, setLoading] = useState(false)
    const [error, setError] = useState('')

    const handleLogin = async (e: React.FormEvent) => {
        e.preventDefault()
        setLoading(true)
        setError('')

        try {
            // Call the login mutation
            const result = await gql(LoginDocument, { username, password })
            console.log('Login successful:', result)
            // Navigate to POS page on successful login
            router.push('/dash/pos')
        } catch (err) {
            console.error('Login failed:', err)
            setError('Login failed. Please check your credentials.')
        } finally {
            setLoading(false)
        }
    }

    return (
        <div className="grid grid-cols-12 gap-4">
            <div className="col-span-8 bg-[url('/login.jpg')] bg-cover bg-center bg-no-repeat"></div>
            <div className="col-span-4">
                <div className="flex h-screen">
                    <div className="my-auto p-4 w-full">
                        <h1 className="text-2xl font-bold mb-6">Login</h1>
                        {error && (
                            <InlineNotification
                                kind="error"
                                title="Error"
                                subtitle={error}
                                className="mb-4"
                            />
                        )}
                        <Form onSubmit={handleLogin}>
                            <div className="mb-4">
                                <TextInput
                                    id="username"
                                    labelText="Username"
                                    value={username}
                                    onChange={(e) => setUsername(e.target.value)}
                                    required
                                />
                            </div>
                            <div className="mb-6">
                                <TextInput
                                    id="password"
                                    labelText="Password"
                                    type="password"
                                    value={password}
                                    onChange={(e) => setPassword(e.target.value)}
                                    required
                                />
                            </div>
                            <Button type="submit" disabled={loading}>
                                {loading ? 'Logging in...' : 'Log In'}
                            </Button>
                        </Form>
                    </div>
                </div>
            </div>
        </div>
    )
}

export default Login
