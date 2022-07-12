import { Tweet } from "../typings";

export const fetchTweets = async () => {
    const res = await fetch(`${process.env.NEXT_PUBLIC_BASE_URL}/api/getTweet`)
    const data = await res.json();
    const tweets: Tweet[] = data.tweets;

    return tweets
}