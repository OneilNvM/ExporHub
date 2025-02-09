import React from 'react'
import CreateAccountForm from './components/CreateAccountForm'

export default function Create() {
  return (
    <div className='flex flex-col flex-1 bg-gradient-to-b from-transparent via-transparent to-pink-900/20'>
      <main className='flex flex-1 flex-row'>
        <section className='flex flex-[2_1_0%] flex-col items-start gap-24 m-5 lg:items-center'>
          <div className='text-5xl'>Create your new account</div>
          <CreateAccountForm />
        </section>
        <section className='hidden flex-1 bg-gradient-to-br from-red-400 via-pink-400 via-indigo-400 to-purple-500 md:flex'>
          <div className='flex flex-1 items-center justify-center'>
            <span aria-hidden="true" aria-label='Gradient'>Create, Share, Explore</span>
          </div>
        </section>
      </main>
    </div>
  )
}
