export type ResponseStatus = {
    code: number,
    message: string
}

export type User = {
    user_id: number,
    username: string,
    email: string,
    password: string,
    bio?: string,
    profileImg?: string,
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
    date_updated?: Date,
}

export type Favourite = {
    favourite_id: number,
    user_id: number,
    project_id: number,
    date_favourited: Date,
}

export type Follow = {
    follow_id: number,
    follower: number,
    following: number,
    date_followed: Date,
}

export type Image = {
    image_id: number,
    file_path: string,
    user_id?: number,
    project_id?: number,
    date_uploaded: Date
}