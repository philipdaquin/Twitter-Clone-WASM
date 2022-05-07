import React from 'react'
import {SearchIcon} from '@heroicons/react/outline'


function Widgets() {
  return (
    <div>
        {/* Search */}
        <div className='flex items-center space-x-2 bg-gray-100'>
            <SearchIcon className='h-5 w-5 '/>
            <input type="text" placeholder='Search Twitter' />
        </div>
    </div>
  )
}

export default Widgets