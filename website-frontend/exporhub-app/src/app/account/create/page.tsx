import React from 'react'
import CreateProjectForm from './components/CreateProjectForm'
<<<<<<< HEAD

export default function CreateProject() {
    return (
        <div className='grid auto-rows-auto size-full overflow-auto'>
            <main className='flex flex-col mx-96 my-20 p-8 border rounded-xl'>
                <div className='flex flex-col items-center w-full'>
                    <h2>Create a Project</h2>
                    <CreateProjectForm />
                </div>
            </main>
        </div>
    )
=======
import { cookies } from 'next/headers'
import { decrypt } from '@/app/lib/session'
import { redirect } from 'next/navigation'

export default async function CreateProject() {
    const cookieStore = await cookies()
    const session = cookieStore.get('session')
    const result = await decrypt(session?.value)

    if (result) {

        return (
            <div className='grid auto-rows-auto size-full overflow-auto'>
                <main className='flex flex-col w-full my-8 px-24'>
                    <div className='flex flex-col items-center gap-16 w-full h-full p-8 border rounded-xl border-pink-200 dark:border-pink-950'>
                        <h2 className='text-3xl'>Create a Project</h2>
                        <CreateProjectForm sessionUserId={result.userId}/>
                    </div>
                </main>
            </div>
        )
    } else {
        redirect("/login")
    }
>>>>>>> origin/main
}
