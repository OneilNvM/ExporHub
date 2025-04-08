'use client'

import { signIn } from '@/app/actions/auth'
import Link from 'next/link'
import { useRouter } from 'next/navigation'
import React, { FormEvent, useRef, useState } from 'react'
import { LoginStatus, User } from '~/types/types'

export default function LoginFormComponent() {
    const [identity, setIdentity] = useState("")
    const [password, setPassword] = useState("")
    const [error, setError] = useState("")
    const router = useRouter()
    const identityInput = useRef<HTMLInputElement | null>(null)
    const passwordInput = useRef<HTMLInputElement | null>(null)

    const resetInputs = () => {
        setPassword("")
        setIdentity("")
    }

    const handleLogin = async (e: FormEvent) => {
        e.preventDefault()

        try {
            const response = await fetch("https://api.exporhub.com:9000/account/login", {
                method: "post",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify({ username_or_email: identity, password: password }),
            })

            if (!response.ok) {
                throw new Error(`Error: ${response.statusText}`)
            }
    
            let json = await response.json();
    
            if (json.code) {
                const status = json as LoginStatus;
    
                console.log(status.code, status.message)
    
                if (status.code === 1) {
                    setError(status.message)
                } else if (status.code === 2) {
                    setError("Invalid login credentials")
                }
            } else {
                const user = json as User;
                console.dir("User Object: " + user)
                await signIn(user)
                console.log("Signin finished")
                router.push('/account')
            }
        } catch (error) {
            console.error(error)

            resetInputs()
        }

        resetInputs()
    }
    return (
        <div className='flex-[0_0_40%]'>
            <div className='min-w-[25rem] bg-gradient-to-b from-[var(--border-color)] to-pink-900 rounded-2xl p-[1px]'>
                <div className='flex flex-col items-center py-10 gap-4 rounded-2xl bg-[var(--background)]'>
                    <p className='font-bold text-xl'>Login To Your Account</p>
                    <p id='error' className='text-red-400'>{error}</p>
                    <form onSubmit={handleLogin} className='flex flex-col self-stretch items-center gap-8'>
                        <div className='flex w-3/5 flex-col gap-3'>
                            <label htmlFor="identity">Email Address/ Username</label>
                            <input value={identity} ref={identityInput} onChange={e => setIdentity(e.target.value)} className='border border-[var(--border-color)] bg-transparent  px-5 py-2 rounded-3xl max-h-10' id='identity' type="text" required />
                        </div>
                        <div className='flex flex-col w-3/5 gap-3'>
                            <div className='flex justify-between'>
                                <label htmlFor="password">Password</label>
                                <label htmlFor="password"><Link href={"/forgot-password"}>Forgot Password?</Link></label>
                            </div>
                            <input value={password} ref={passwordInput} onChange={e => setPassword(e.target.value)} className='border border-[var(--border-color)] px-5 py-2 rounded-3xl max-h-10 bg-transparent' id='password' type="password" required />
                        </div>
                        <div className='flex flex-col items-center gap-2'>
                            <input className='cursor-pointer rounded-3xl px-10 py-1 text-md transition-all hover:shadow-md hover:shadow-pink-900/40 bg-pink-300 hover:bg-pink-600' type="submit" value="Login" />
                            <p>Or</p>
                            <Link href={"/create"}>Create a new account</Link>
                        </div>
                    </form>
                </div>
            </div>
        </div>
    )
}
