import React from 'react'
import {Tweet} from '../typings'
import {
    ChatAlt2Icon,
    HeartIcon,
    SwitchHorizontalIcon,
    UploadIcon
} from '@heroicons/react/outline'
import TimeAgo from 'react-timeago'

interface TweetProps { 
    tweet: Tweet
}

function Tweet({tweet} : TweetProps) {
    
    return (
        <>
            <section className='flex flex-col space-x-3 border-y p-5 border-gray-100 '>
                <div className='flex space-x-3' >
                    <img className='h-10 w-10 rounded-full object-cover' src={tweet.profile_img} alt="" />
                    <div>
                        <div className='flex items-center space-x-1'>
                            <p className='mr-1 font-bold'>{tweet.username}</p>
                            <p className='hidden text-sm text-gray-500 sm:inline'>@{tweet.username.replace(/\s+/g, '').toLowerCase()}  .  </p>
                            <TimeAgo 
                                className="text-sm text-gray-500"
                                date={tweet.created_at}
                            />
                        </div>
                        <p className='pt-1'>{tweet.text}</p>
                        {tweet.image && 
                            <img 
                                src={tweet.image} 
                                alt="" 
                                className='m-5 ml-0 mb-1 max-h-60 rounded-lg object-cover shadow-sm'
                            />}
                    </div>
                </div>
                <div className='mt-5 flex justify-between'>
                    
                    <div className='flex cursor-pointer items-center space-x-3 text-gray-400'>
                        <ChatAlt2Icon className='h-5 w-5'/>
                        <p></p>
                    </div>
                    
                    <div className='flex cursor-pointer items-center space-x-3 text-gray-400'>
                        <SwitchHorizontalIcon className='h-5 w-5'/>
                    </div>
                    
                    <div className='flex cursor-pointer items-center space-x-3 text-gray-400'>
                        <HeartIcon className='h-5 w-5'/>
                    </div>
                    
                    <div className='flex cursor-pointer items-center space-x-3 text-gray-400'>
                        <UploadIcon className='h-5 w-5'/>
                    </div>
                </div>
            </section>
        </>
    )
}

export default Tweet