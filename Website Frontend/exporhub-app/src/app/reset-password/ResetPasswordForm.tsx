import React from 'react'

export default function ResetPasswordForm() {
    return (
        <div className='min-w-[30rem] bg-gradient-to-b from-[var(--border-color)] to-pink-900 rounded-2xl p-[1px]'>
            <div className='rounded-2xl bg-[var(--background)]'>
                <form className='flex flex-col gap-6 p-12' action="">
                    <div className='flex flex-col gap-2'>
                        <label className='text-xl' htmlFor="new-passwd">New Password</label>
                        <input className='border border-[var(--border-color)] bg-transparent px-5 py-2 rounded-3xl max-h-10' type="password" name="" id="new-passwd" />
                    </div>
                    <div className='flex flex-col gap-2'>
                        <label className='text-xl' htmlFor="confirm-passwd">Confirm Password</label>
                        <input className='border border-[var(--border-color)] bg-transparent px-5 py-2 rounded-3xl max-h-10' type="password" name="" id="confirm-passwd" />
                    </div>

                    <input type="submit" value="Reset Password" />
                </form>
            </div>
        </div>
    )
}
