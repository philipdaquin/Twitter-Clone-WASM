import React, { SVGProps } from 'react'

interface Props { 
    Icon: (props: SVGProps<SVGSVGElement>) => JSX.Element
    title: String,
    onClick: () => {}
}

function SidebarRow({Icon, title, onClick}: Props) {
  return (
    <div onClick={() => onClick?.()} className='group flex max-w-fit items-center space-x-2 px-4 py-3 
    rounded-full hover:bg-gray-100 cursor-pointer transition-all duration-200'>
        <Icon className="w-6 h-6"/>
        <p className='hidden group-hover:text-twitter md:inline-flex first-letter 
            text-base font-regular lg:text-xl'>{title}</p>
    </div>
  )
}

export default SidebarRow