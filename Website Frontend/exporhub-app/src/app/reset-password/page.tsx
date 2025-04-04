import React from 'react'
import ResetPasswordForm from './ResetPasswordForm'

export default function ResetPassword() {
  return (
    <div className='flex flex-col flex-1'>
      <main className='flex flex-1 items-center justify-center'>
        <div className='flex flex-col flex-[0_0_40%] gap-12 justify-center'>
          <div className='text-4xl text-center'>Reset Password</div>
          <ResetPasswordForm />
        </div>
      </main>
    </div>
  )
}
