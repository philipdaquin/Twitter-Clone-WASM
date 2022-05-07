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

export type CommentBody = { 
    comment: string
    tweetId: string
    username: string
    profile_img: string
}

export interface Comment extends CommentBody { 
    created_at: string
    id: string 
    rev: string
    type: 'comment'
    updated_at: string
    tweet: { 
        ref: string
        type: 'reference'
    }
}