import React from 'react'
import EmailVerificationForm from './EmailVerificationForm'

export default function ForgotPassword() {
  return (
    <div className='flex flex-col flex-1'>
      <main className='flex flex-1 justify-center items-center'>
        <div className='flex flex-col flex-[0_0_40%] justify-center gap-8'>
          <div className='text-center text-3xl'>Forgot your password?</div>
          <EmailVerificationForm />
        </div>
      </main>
    </div>
  )
}
