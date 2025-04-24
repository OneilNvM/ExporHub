'use client'

import FavouritesTab from '@/app/components/profile-tabs/FavouritesTab'
import FollowingTab from '@/app/components/profile-tabs/FollowingTab'
import ProfileTab from '@/app/components/profile-tabs/ProfileTab'
import ProjectsTab from '@/app/components/profile-tabs/ProjectsTab'
import { useSearchParams } from 'next/navigation'
import React from 'react'
<<<<<<< HEAD

export default function ProfileTabs() {
=======
import { Project, User } from '~/types/types'

export default function ProfileTabs({ user, favouritesArr, favouritedProjectsArr, projects, favouriteProjects, followings, profileUserId, sessionUserId, projectNotifications, isFollowed }: { user: User | null, profileUserId: number, sessionUserId: number, followings: User[] | null, favouritesArr: boolean[], favouritedProjectsArr: boolean[], projects: Project[] | null, projectNotifications: Project[] | null, favouriteProjects: Project[] | null, isFollowed: boolean }) {
>>>>>>> origin/main
    const searchParams = useSearchParams()

    switch (searchParams.get('tab')) {
        case 'projects':
            return (
<<<<<<< HEAD
                <ProjectsTab />
            )
        case 'favourites':
            return (
                <FavouritesTab />
            )
        case 'following':
            return (
                <FollowingTab />
            )
        default:
            return (
                <ProfileTab />
=======
                <ProjectsTab sessionUserId={sessionUserId} favouritesArr={favouritesArr} projects={projects} />
            )
        case 'favourites':
            return (
                <FavouritesTab favouritedProjectsArr={favouritedProjectsArr} favouriteProjects={favouriteProjects} sessionUserId={sessionUserId} />
            )
        case 'following':
            return (
                <FollowingTab followings={followings} profileUserId={profileUserId} />
            )
        default:
            return (
                <ProfileTab sessionUserId={sessionUserId} profileUserId={profileUserId} isFollowed={isFollowed} user={user} projectNotifications={projectNotifications} />
>>>>>>> origin/main
            )
    }
}
