import React from 'react'
import CreateProjectForm from './components/CreateProjectForm'

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
}
