import FavouritesTab from '@/app/components/profile-tabs/FavouritesTab'
import FollowingTab from '@/app/components/profile-tabs/FollowingTab'
import ProfileTab from '@/app/components/profile-tabs/ProfileTab'
import ProjectsTab from '@/app/components/profile-tabs/ProjectsTab'
import { headers } from 'next/headers'
import React from 'react'
import { Project, User } from '~/types/types'

export default async function AccountTab({ user, sessionUserId, followings, projects, projectNotifications, favouriteProjects }: { user: User | null, sessionUserId: number, followings: User[] | null, projects: Project[] | null, projectNotifications: Project[] | null, favouriteProjects: Project[] | null }) {
    const headerList = await headers()

    const searchParams = headerList.get("x-search-params")

    switch (searchParams) {
        case 'tab=projects':
            return (
                <ProjectsTab user={user} sessionUserId={sessionUserId} projects={projects} />
            )
        case 'tab=favourites':
            return (
                <FavouritesTab sessionUserId={sessionUserId} favouriteProjects={favouriteProjects} />
            )
        case 'tab=following':
            return (
                <FollowingTab sessionUserId={sessionUserId} followings={followings} />
            )
        default:
            return (
                <ProfileTab projectNotifications={projectNotifications} user={user} />
            )
    }
}