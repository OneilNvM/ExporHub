'use client'

import React from 'react'

export default function EmailVerificationForm() {
    return (
        <div className='min-w-[25rem] bg-gradient-to-b from-[var(--border-color)] to-pink-900 rounded-2xl p-[1px]'>
            <div className='flex flex-col items-center py-10 px-4 gap-8 rounded-2xl bg-[var(--background)]'>
                <div className='text-2xl text-center'>Send a password reset link to your email address</div>
                <form className='flex flex-col self-stretch items-center gap-8' action="/forgot-password" method='get'>
                    <div className='inline-flex min-w-[60%] flex-col gap-3'>
                        <label className='text-lg' htmlFor="email">Email Address</label>
                        <input className='border border-[var(--border-color)] bg-transparent px-5 py-2 rounded-3xl max-h-10' id='email' type="text" name='email' />
                    </div>

                    <input className='cursor-pointer rounded-3xl px-10 py-1 text-md transition-all hover:shadow-md hover:shadow-pink-900/40 bg-pink-300 hover:bg-pink-600 dark:border-white' type="submit" value="Send Email" />
                </form>
            </div>
        </div>
    )
}
