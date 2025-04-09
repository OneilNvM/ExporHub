'use client'

import FavouritesTab from '@/app/components/profile-tabs/FavouritesTab'
import FollowingTab from '@/app/components/profile-tabs/FollowingTab'
import ProfileTab from '@/app/components/profile-tabs/ProfileTab'
import ProjectsTab from '@/app/components/profile-tabs/ProjectsTab'
import { useSearchParams } from 'next/navigation'
import React from 'react'
import { Project, User } from '~/types/types'

export default function AccountTab({ user, projects, projectNotifications, favouriteProjects }: {user: User | null, projects: Project[] | null, projectNotifications: Project[] | null, favouriteProjects: Project[] | null}) {
    const searchParams = useSearchParams()

    switch (searchParams.get('tab')) {
        case 'projects':
            return (
                <ProjectsTab user={user} projects={projects} />
            )
        case 'favourites':
            return (
                <FavouritesTab user={user} favouriteProjects={favouriteProjects} />
            )
        case 'following':
            return (
                <FollowingTab />
            )
        default:
            return (
                <ProfileTab projectNotifications={projectNotifications} user={user}/>
            )
    }
}