import React from 'react'
import { RefreshIcon } from '@heroicons/react/outline'
import TweetBox from './TweetBox'


function Feed() {
  return (
    <div className="col-span-7 lg:log-span-5 border-x">
        <div className="flex items-center justify-between">
            <h1 className='p-5 pb-0 text-xl font-bold'>Home</h1>
            <RefreshIcon className='h-8 w-8 cursor-pointer text-twitter 
                mr-5 transition-all duration-500 ease-out hover:rotate-180 active:scale-125'/>
        </div>
        {/* Tweet box */}
        <div>
            <TweetBox />
        </div>
    </div>
  )
}

export default Feed