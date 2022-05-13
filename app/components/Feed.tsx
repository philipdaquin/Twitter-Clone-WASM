import React, { useState } from 'react'
import { RefreshIcon } from '@heroicons/react/outline'
import TweetBox from './TweetBox'
import { Tweet } from '../typings'
import TweetComponent from '../components/Tweet' 
import { fetchTweets } from '../utils/fetchTweets'
import toast from 'react-hot-toast'

interface Props { 
    tweets: Tweet[]
}

function Feed({tweets: tweetsProp}: Props) {
    const [tweets, setTweets] = useState<Tweet[]>(tweetsProp);
    console.log(tweets);

    const handleRequest = async () => { 
        const refreshToast = toast.loading('Refreshing..');
        //  Fetch New Recent Tweets 
        const tweets = await fetchTweets();
        //  Set the Set to this Newtweets 
        setTweets(tweets)

        toast.success('Feed Updated', { 
            id: refreshToast
        })
    }

    return (
    <div className="col-span-7 lg:col-span-5 max-h-screen overflow-scroll border-x scrollbar-hide">
        <div className="flex items-center justify-between">
            <h1 className='p-5 pb-0 text-xl font-bold'>Home</h1>
            <RefreshIcon 
                onClick={handleRequest} 
                className='h-8 w-8 cursor-pointer text-twitter 
                    mr-5 transition-all duration-500 ease-out hover:rotate-180 active:scale-125'
            />
        </div>
        {/* Tweet bo</div>x */}
        <div>
            <TweetBox setTweets={setTweets}/>
        </div>

        {/* Feed */}
        <div>
            {/* {tweets.map(i => {
                <TweetComponent key={i.id} tweet={i} />
            })} */}
        </div>
    </div>
  )
}

export default Feed