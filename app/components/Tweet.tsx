import React, { useState, useEffect } from 'react'
import {Tweet} from '../typings'
import {
    ChatAlt2Icon,
    HeartIcon,
    SwitchHorizontalIcon,
    UploadIcon
} from '@heroicons/react/outline'
import TimeAgo from 'react-timeago'
import { fetchComments } from '../utils/fetchComments'
import { Comment } from '../typings'

interface Props { 
    tweet: Tweet
}

function Tweet({tweet} : Props) {
    const [comments, setComments] = useState<Comment[]>([])

    const refreshComments = async () => { 
        const comments: Comment[] = await fetchComments(tweet.id)
        setComments(comments);
    }
    useEffect(() => {
      refreshComments();
    }, [])

    return (
        <>
            <div className='flex flex-col space-x-3 border-y p-5 border-gray-100 '>
                <p>hellssso</p>
                <div className='flex space-x-3'>
                    <img className='h-10 w-10 rounded-full object-cover' src={tweet.profile_img} alt="" />
                    <div>
                        <div className='flex items-center space-x-1'>
                            <p className='mr-1 font-bold'>{tweet.username}</p>
                            <p className='hidden text-sm text-gray-500 sm:inline'>@{tweet.username.replace(/\s+/g,'').toLowerCase()}  .  </p>
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
                
                {/* Comment Box Login */}
                {/* {comments?.map(comment => { 
                    <div key={comment.id}>
                        <img 
                            src={comment.profile_img} 
                            className='h-7 w-7 object-cover ' 
                            alt="" />
                        <div>
                            <div>
                                <p>{comment.username}</p>
                                <p>@{comment.username
                                    .replace(/\s+/g, '').toLowerCase()}
                                </p>
                            </div>                            
                        </div>
                    </div>
                })} */}

            </div>
        </>
    )
}

export default Tweet