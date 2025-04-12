'use client'

import React from 'react'

export default function CreateProjectForm() {
  return (
    <div className='w-full'>
        <form className='flex flex-col gap-8 items-center' action="" encType='multipart'>
            <div className='flex flex-col w-1/2 gap-3'>
                <label htmlFor='project-name'>Project Name</label>
                <input className='input' id='project-name' type="text" placeholder='A new name :o'/>
            </div>
            <div className='flex flex-col w-1/2 gap-3'>
                <label htmlFor='description'>Description</label>
                <textarea className='input' id='description' placeholder="That's pretty interesting..." />
            </div>
            <div className='flex flex-col w-1/2 gap-3'>
              <label htmlFor='file-input'>Images</label >
              <input className='block w-full text-base text-pink-950 border border-pink-200 rounded-lg cursor-pointer bg-pink-50 file:bg-pink-600 file:border-none file:text-white hover:file:bg-pink-700 dark:text-pink-700 focus:outline-none dark:bg-pink-950/25 dark:border-pink-950 dark:file:bg-pink-800 dark:hover:file:bg-pink-900 dark:placeholder-gray-400' type="file" name='files' id='file-input' multiple/>
            </div>
            <div className='flex flex-col w-1/2 gap-8'>
              <p>Socials</p>
              <input className='input' type="text" placeholder='X link' />
              <input className='input' type="text" placeholder='Github link' />
              <input className='input' type="text" placeholder='Instagram link' />
              <input className='input' type="text" placeholder='Linkedin link' />
            </div>
            <div>
              <input className='border-none rounded-full cursor-pointer px-8 py-2 transition-colors duration-300 ease-in-out bg-pink-600 text-white hover:bg-pink-700 dark:bg-pink-800' type="submit" value="Create Project" />
            </div>
        </form>
    </div>
  )
}
