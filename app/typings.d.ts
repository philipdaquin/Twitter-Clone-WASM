export interface Tweet extends TweetBody { 
    id: string
    created_at: string
    updated_at: string
    rev: string
    type: 'tweet'
    blockTweet: boolean
}

export type TweetBody = {
    text: string 
    username: string
    profile_img: string
    image: string
}