import React, { SVGProps } from 'react'

interface Props { 
    Icon: (props: SVGProps<SVGSVGElement>) => JSX.Element
    title: String
}

function SidebarRow({Icon, title}: Props) {
  return (
    <div className='flex max-w-fit items-center space-x-2 px-4 py-3 rounded-full 
        hover:bg-gray-100 cursor-pointer transition-all duration200 group'>

        <Icon className="w-6 h-6"/>
        <p className='group-hover:text-twitter'>{title}</p>
    </div>
  )
}

export default SidebarRow