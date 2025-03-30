export type LoginStatus = {
    code: number,
    message: string
}

export type User = {
    userId: number,
    username: string,
    email: string,
    password: string,
    profileImg: string,
    followers: number,
    dateCreated: Date
}