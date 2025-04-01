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
    profileImg: string,
    followers: number,
    dateCreated: Date
}

export type UserSessionPayload = {
    userId: number,
    expiresAt: Date
}