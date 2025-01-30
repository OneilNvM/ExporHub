'use client'

import Link from 'next/link'
import React from 'react'

export default function LoginFormComponent() {
    return (
        <div className='min-w-1/2'>
            <div className='flex flex-col items-center py-10 gap-4 border rounded-2xl '>
                <p className='font-bold text-xl'>Login To Your Account</p>
                <p className='text-red-400'></p>
                <form className='flex flex-col gap-10' action="">
                    <div className='flex flex-col gap-4'>
                        <label htmlFor="identity">Email Address/ Username</label>
                        <input id='identity' type="text" />
                    </div>
                    <div className='flex flex-col gap-4'>
                        <div>
                            <label htmlFor="password">Password</label>
                            <label htmlFor="password">Forgot Password</label>
                        </div>
                        <input className='border rounded-3xl max-h-10 ' id='password' type="password" />
                    </div>
                    <div>
                        <input type="submit" value="Login" />
                        <p>Or</p>
                        <Link href={"/create"}>Create a new account</Link>
                    </div>
                </form>
            </div>
        </div>
    )
}
