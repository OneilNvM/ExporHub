'use client'

import Link from 'next/link'
import React from 'react'

export default function CreateAccountForm() {
  return (
    <div className='flex flex-1 mt-24'>
        <form className='flex flex-col gap-5' action="">
          <div className='flex flex-col gap-3'>
            <label className='text-2xl' htmlFor="username">Username</label>
            <input className='px-5 py-4 text-xl rounded-3xl shadow-lg shadow-pink-300/40 focus:outline-pink-400 bg-gradient-to-b from-transparent to-pink-300/30' required type="text" id='username' spellCheck='false' />
          </div>
          <div className='flex flex-col gap-3'>
            <label className='text-2xl' htmlFor="email">Email Address</label>
            <input className='px-5 py-4 text-xl rounded-3xl shadow-lg shadow-pink-300/40 focus:outline-pink-400 bg-gradient-to-b from-transparent to-pink-300/30' required type="text" id='email' spellCheck='false' />
          </div>
          <div className='flex flex-col gap-3'>
            <label className='text-2xl' htmlFor="password">Password</label>
            <input className='px-5 py-4 text-pink-600 text-xl rounded-3xl shadow-lg shadow-pink-300/40 focus:outline-pink-400 bg-gradient-to-b from-transparent to-pink-300/30' required type="password" name="" id="password" spellCheck='false' />
          </div>
          <div className='flex flex-row-reverse gap-4'>
            <label htmlFor="terms-privacy">By ticking this checkbox, you confirm that you agree to the <Link className='text-blue-400 underline' href={'/terms-and-conditions'}>Terms and Conditions</Link> and the <Link className='text-blue-400 underline' href={'/privacy-policy'}>Privacy Policy</Link></label>
            <input className='' required type="checkbox" name="" id="terms-privacy" />
          </div>

          <input className='mt-12 px-6 py-2 text-lg rounded-3xl self-center cursor-pointer shadow-lg transition-all hover:translate-y-[-5px] hover:bg-pink-400 hover:shadow-pink-300/90 hover:shadow-xl bg-pink-200 ' type="submit" value="Create Account" />
        </form>
    </div>
  )
}
