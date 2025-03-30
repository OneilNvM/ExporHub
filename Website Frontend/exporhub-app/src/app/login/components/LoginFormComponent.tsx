'use client'

import Link from 'next/link'
import React, { FormEvent, useRef, useState } from 'react'
import { LoginStatus } from '~/types/types'

export default function LoginFormComponent() {
    const [identity, setIdentity] = useState("")
    const [password, setPassword] = useState("")
    const identityInput = useRef<HTMLInputElement | null>(null)
    const passwordInput = useRef<HTMLInputElement | null>(null)

    const resetInputs = () => {
        if (identityInput.current && passwordInput.current) {
            identityInput.current.value = ""
            passwordInput.current.value = ""
        }

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
                throw new Error(`Invalid credentials provided`)
            }

            let json = await response.json() as LoginStatus;

            console.log(json.code, json.message)

            resetInputs()
        } catch (error) {
            const errorField: HTMLElement | null = document.getElementById('error');
            const message = `${error}`

            if (errorField) {
                errorField.innerHTML = message.replace("Error: ", "")
            }

            if (passwordInput.current) {
                passwordInput.current.value = ""
            }
    
            setPassword("")
        }

    }
    return (
        <div className='flex-[0_0_40%]'>
            <div className='min-w-[25rem] bg-gradient-to-b from-[var(--border-color)] to-pink-900 rounded-2xl p-[1px]'>
                <div className='flex flex-col items-center py-10 gap-4 rounded-2xl bg-[var(--background)]'>
                    <p className='font-bold text-xl'>Login To Your Account</p>
                    <p id='error' className='text-red-400'></p>
                    <form onSubmit={handleLogin} className='flex flex-col self-stretch items-center gap-8'>
                        <div className='flex w-3/5 flex-col gap-3'>
                            <label htmlFor="identity">Email Address/ Username</label>
                            <input ref={identityInput} onChange={e => setIdentity(e.target.value)} className='border border-[var(--border-color)] bg-transparent  px-5 py-2 rounded-3xl max-h-10' id='identity' type="text" required />
                        </div>
                        <div className='flex flex-col w-3/5 gap-3'>
                            <div className='flex justify-between'>
                                <label htmlFor="password">Password</label>
                                <label htmlFor="password"><Link href={"/forgot-password"}>Forgot Password?</Link></label>
                            </div>
                            <input ref={passwordInput} onChange={e => setPassword(e.target.value)} className='border border-[var(--border-color)] px-5 py-2 rounded-3xl max-h-10 bg-transparent' id='password' type="password" required />
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
