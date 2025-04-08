import ProjectItem from '@/app/components/project/ProjectItem'
import ProfileItem from '@/app/components/user/ProfileItem'
import React from 'react'

export default function SearchResults() {
    return (
        <>
            <section className='flex w-full justify-evenly'>
                <p className='text-2xl'>Search results</p>
                <div className='bg-black h-fit rounded-full'>
                    <button className='text-lg px-6 py-1'>Filter</button>
                </div>
            </section>
            <section className='flex flex-col gap-8'>
                <ProjectItem />
                <ProfileItem />
                <ProjectItem />
                <ProfileItem />
                <ProjectItem />
                <ProfileItem />
                <ProjectItem />
                <ProfileItem />
                <ProjectItem />
                <ProfileItem />
            </section>
        </>
    )
}
