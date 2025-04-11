'use client'

import FavouritesTab from '@/app/components/profile-tabs/FavouritesTab'
import FollowingTab from '@/app/components/profile-tabs/FollowingTab'
import ProfileTab from '@/app/components/profile-tabs/ProfileTab'
import ProjectsTab from '@/app/components/profile-tabs/ProjectsTab'
import { useSearchParams } from 'next/navigation'
import React from 'react'
import { Follow, Project, User } from '~/types/types'

export default function AccountTab({ user, sessionUserId, followings, projects, favouritesArr, projectNotifications, favouriteProjects }: { user: User | null, sessionUserId: number, followings: User[] | null, favouritesArr: boolean[], projects: Project[] | null, projectNotifications: Project[] | null, favouriteProjects: Project[] | null }) {
    const searchParams = useSearchParams()

    switch (searchParams.get('tab')) {
        case 'projects':
            return (
                <ProjectsTab favouritesArr={favouritesArr} sessionUserId={sessionUserId} projects={projects} />
            )
        case 'favourites':
            return (
                <FavouritesTab sessionUserId={sessionUserId} favouriteProjects={favouriteProjects} />
            )
        case 'following':
            return (
                <FollowingTab sessionUserId={sessionUserId} followings={followings} />
            )
        default:
            return (
                <ProfileTab projectNotifications={projectNotifications} user={user} />
            )
    }
}