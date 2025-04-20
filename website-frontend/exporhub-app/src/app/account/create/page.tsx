import React from 'react'
import CreateProjectForm from './components/CreateProjectForm'

export default function CreateProject() {
    return (
        <div className='grid auto-rows-auto size-full overflow-auto'>
            <main className='flex flex-col w-full my-8 px-24'>
                <div className='flex flex-col items-center gap-16 w-full h-full p-8 border rounded-xl border-pink-200 dark:border-pink-950'>
                    <h2 className='text-3xl'>Create a Project</h2>
                    <CreateProjectForm />
                </div>
            </main>
        </div>
    )
}
