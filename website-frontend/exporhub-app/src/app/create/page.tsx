import React from 'react'
import FormContainer from './components/FormContainer'

export default function Create() {
  return (
    <div className='flex flex-col flex-1'>
      <main className='flex flex-1 flex-row'>
        <FormContainer />
        <section className='hidden flex-1 bg-gradient-to-br from-red-400 via-indigo-400 to-purple-500 md:flex'>
          <div className='flex flex-col m-4 flex-1 items-center justify-center gap-4'>
            <span aria-label='Side Gradient' className='text-4xl text-center'>Create, Share, Explore!</span>
            <span className='text-xl text-center'>Get started with a new account to share your inspirations with the world.</span>
          </div>
        </section>
      </main>
    </div>
  )
}
