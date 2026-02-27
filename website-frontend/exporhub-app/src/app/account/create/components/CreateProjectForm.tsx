'use client'

import { useRouter } from 'next/navigation'
import React, { FormEvent, Suspense, useState } from 'react'
import { uploadImages } from '~/supabase/functions/uploadImage'
import { Project } from '~/types/types'

export default function CreateProjectForm({ sessionUserId }: { sessionUserId: number }) {
  const [name, setName] = useState("")
  const [description, setDescription] = useState("")
  const [images, setImages] = useState<FileList | null>(null)
  const router = useRouter()

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault()

    try {
      await uploadImages(images)

      const imageNames: Array<string | undefined> = []

      if (images) {
        for (let i = 0; i < images.length; i++) {
          const item = images.item(i)
          if (item) {
            imageNames.push(item.name)
          }
        }
      }

      const response = await fetch("https://api.exporhub.com:9000/api/project/create-project", {
        method: "post",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ project_name: name, description: description, images: imageNames, user_id: sessionUserId })
      })

      if (!response.ok) {
        throw new Error(`Failed to create project`)
      }

      const project = await response.json() as Project;

      console.log(project)

      router.push(`/project/${project.name}`)
    } catch (error) {
      console.error(error)
    }
  }

  return (
    <Suspense>
      <div className='w-full'>
        <form onSubmit={handleSubmit} className='flex flex-col gap-8 items-center'>
          <div className='flex flex-col w-1/2 gap-3'>
            <label htmlFor='project-name'>Project Name</label>
            <input onChange={e => setName(e.target.value)} value={name} className='input' id='project-name' type="text" placeholder='A new name :o' required />
          </div>
          <div className='flex flex-col w-1/2 gap-3'>
            <label htmlFor='description'>Description</label>
            <textarea onChange={e => setDescription(e.target.value)} value={description} className='input' id='description' placeholder="That's pretty interesting..." required />
          </div>
          <div className='flex flex-col w-1/2 gap-3'>
            <label htmlFor='file-input'>Images</label >
            <input onChange={e => setImages(e.target.files)} className='block w-full text-base text-pink-950 border border-pink-200 rounded-lg cursor-pointer bg-pink-50 file:bg-pink-600 file:border-none file:text-white hover:file:bg-pink-700 dark:text-pink-700 focus:outline-none dark:bg-pink-950/25 dark:border-pink-950 dark:file:bg-pink-800 dark:hover:file:bg-pink-900 dark:placeholder-gray-400' type="file" id='file-input' multiple />
          </div>
          <div className='flex flex-col w-1/2 gap-8'>
            <p>Socials</p>
            <input className='input' type="text" placeholder='X link' />
            <input className='input' type="text" placeholder='Github link' />
            <input className='input' type="text" placeholder='Instagram link' />
            <input className='input' type="text" placeholder='Linkedin link' />
          </div>

          <input className='border-none rounded-full cursor-pointer px-8 py-2 transition-colors duration-300 ease-in-out bg-pink-600 text-white hover:bg-pink-700 dark:bg-pink-800' type="submit" value="Create Project" />
        </form>
      </div>
    </Suspense>
  )
}
