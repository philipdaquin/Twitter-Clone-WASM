import React from 'react'
import router, { useRouter } from 'next/router'
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
import { signIn, signOut, useSession } from 'next-auth/react'

function Sidebar() {

  const router = useRouter();

  const { data: session } = useSession()

  return (
    <>
      <section className='col-span-2 flex flex-col items-center px-4 md:items-start'>
        <img src="https://ra.ac.ae/wp-content/uploads/2020/01/logo-twitter-icon-symbol-0.png" className="m-3 h-10 w-10" alt="" />        
        <SidebarRow Icon={HomeIcon} title="Home" onClick={() => router.push("/")}/>
        <SidebarRow Icon={HashtagIcon} title="Explore" onClick={() => router.push("/")}/>
        <SidebarRow Icon={BellIcon} title="Notifactions" onClick={() => router.push("/")}/>
        <SidebarRow Icon={MailIcon} title="Messages" onClick={() => router.push("/")}/>
        <SidebarRow Icon={BookmarkIcon} title="Bookmarks" onClick={() => router.push("/")}/>
        <SidebarRow Icon={CollectionIcon} title="Lists" onClick={() => router.push("/")}/>
        <SidebarRow onClick={session ? signOut: signIn} Icon={UserIcon} title={session? 'Sign Out': 'Sign In'}/>
        <SidebarRow Icon={DotsCircleHorizontalIcon} title="More" onClick={() => router.push("/")}/>
      </section>
    </>
  )
}

export default Sidebar

