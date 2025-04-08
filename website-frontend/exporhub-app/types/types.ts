export type LoginStatus = {
    code: number,
    message: string
}

export type AccountCreateStatus = {
    code: number,
    message: string
}

export type User = {
    user_id: number,
    username: string,
    email: string,
    password: string,
    bio: string,
    profileImg: string,
    followers: number,
    dateCreated: Date
}

export type UserSessionPayload = {
    userId: number,
    expiresAt: Date
}

export type Project = {
    project_id: number,
    name: string,
    description: string,
    favourites: number,
    user_id: number,
    date_created: Date,
    date_updated: Date,
}