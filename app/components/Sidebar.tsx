import React from 'react'

import {
  BellIcon,
  HashtagIcon,
  BookmarkIcon,
  CollectionIcon,
  DotsCircleHorizontalIcon,
  MailIcon,
  UserIcon,
  HomeIcon
} from '@heroicons/react/outline' 
import SidebarRow from './SidebarRow'


function Sidebar() {
  return (
    <>
      <section className='flex flex-col '>
        <img src="https://links.papareact.com/drq" className="h-10 w-10" alt="" />        
        <SidebarRow Icon={HomeIcon} title="Hello"/>
        <SidebarRow Icon={HashtagIcon} title="Explore"/>
        <SidebarRow Icon={BellIcon} title="Notifactions"/>
        <SidebarRow Icon={MailIcon} title="Messages"/>
        <SidebarRow Icon={BookmarkIcon} title="Bookmarks"/>
        <SidebarRow Icon={CollectionIcon} title="Lists"/>
        <SidebarRow Icon={UserIcon} title="Sign In"/>

        <SidebarRow Icon={DotsCircleHorizontalIcon} title="More"/>
      </section>
    </>
  )
}

export default Sidebar