'use client'

import Link from 'next/link'
import React from 'react'

export default function LoginFormComponent() {
    return (
        <div className='flex-[0_0_40%]'>
            <div className='min-w-[25rem] bg-gradient-to-b from-[var(--border-color)] to-pink-900 rounded-2xl p-[1px]'>
                <div className='flex flex-col items-center py-10 gap-4 rounded-2xl bg-[var(--background)]'>
                    <p className='font-bold text-xl'>Login To Your Account</p>
                    <p className='text-red-400'></p>
                    <form className='flex flex-col self-stretch items-center gap-8' action="">
                        <div className='flex w-3/5 flex-col gap-3'>
                            <label htmlFor="identity">Email Address/ Username</label>
                            <input className='border border-[var(--border-color)] bg-transparent px-5 py-2 rounded-3xl max-h-10' id='identity' type="text" />
                        </div>
                        <div className='flex flex-col w-3/5 gap-3'>
                            <div className='flex justify-between'>
                                <label htmlFor="password">Password</label>
                                <label htmlFor="password"><Link href={"/forgot-password"}>Forgot Password?</Link></label>
                            </div>
                            <input className='border border-[var(--border-color)] px-5 py-2 rounded-3xl max-h-10 bg-transparent' id='password' type="password" />
                        </div>
                        <div className='flex flex-col items-center gap-2'>
                            <input className='cursor-pointer rounded-3xl px-10 py-1 text-md transition-all hover:shadow-md hover:shadow-pink-900/40 bg-pink-300 hover:bg-pink-600 dark:border-white' type="submit" value="Login" />
                            <p>Or</p>
                            <Link href={"/create"}>Create a new account</Link>
                        </div>
                    </form>
                </div>
            </div>
        </div>
    )
}
