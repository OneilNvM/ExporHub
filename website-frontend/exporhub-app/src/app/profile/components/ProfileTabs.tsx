import FavouritesTab from '@/app/components/profile-tabs/FavouritesTab'
import FollowingTab from '@/app/components/profile-tabs/FollowingTab'
import ProfileTab from '@/app/components/profile-tabs/ProfileTab'
import ProjectsTab from '@/app/components/profile-tabs/ProjectsTab'
import { headers } from 'next/headers'
import React from 'react'
import { Project, User } from '~/types/types'

export default async function ProfileTabs({ user, projects, favouriteProjects, followings, profileUserId, sessionUserId, projectNotifications, isFollowed }: { user: User | null, profileUserId: number, sessionUserId: number, followings: User[] | null, projects: Project[] | null, projectNotifications: Project[] | null, favouriteProjects: Project[] | null, isFollowed: boolean }) {
    const headerList = await headers()

    const searchParams = headerList.get("x-search-params")

    console.log(searchParams)

    switch (searchParams) {
        case 'tab=projects':
            return (
                <ProjectsTab user={user} sessionUserId={sessionUserId} projects={projects} />
            )
        case 'tab=favourites':
            return (
                <FavouritesTab user={user} favouriteProjects={favouriteProjects} sessionUserId={sessionUserId} />
            )
        case 'tab=following':
            return (
                <FollowingTab followings={followings} sessionUserId={sessionUserId} />
            )
        default:
            return (
                <ProfileTab sessionUserId={sessionUserId} profileUserId={profileUserId} isFollowed={isFollowed} user={user} projectNotifications={projectNotifications} />
            )
    }
}
