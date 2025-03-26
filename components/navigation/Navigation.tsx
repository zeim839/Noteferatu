"use client"

import { ReactNode, useState } from "react"
import { usePathname } from "next/navigation"
import { NavigationState } from "./NavigationState"
import { LeftNavigation, LeftSidebar } from "./LeftNavigation"
import { RightNavigation, RightSidebar } from "./RightNavigation"
import { cn } from "@/lib/utils"

const Navigation = ({ children } : { children?: ReactNode }) => {
  const [isLeftOpen, setLeftOpen] = useState<boolean>(false)
  const [isRightOpen, setRightOpen] = useState<boolean>(false)

  // Use a solid background color for notes page otherwise text
  // visibility is poor.
  const background = usePathname() === '/note' ?
    'bg-[#FBF9F3]' : 'bg-transparent'

  // Wrap Navigation state to conveniently pass it to LeftNavigation
  // and RightNavigation.
  const navState = () => ({
    isLeftOpen, setLeftOpen, isRightOpen, setRightOpen
  } as NavigationState)

  return (
    <div>
      <LeftNavigation state={navState()} />
      <RightNavigation state={navState()} />
      <div className={cn(background, 'flex justify-between')}>
        <LeftSidebar isOpen={isLeftOpen}/>
        <div className='w-full h-full overflow-hidden'>
          {children}
        </div>
        <RightSidebar isOpen={isRightOpen}/>
      </div>
    </div>
  )
}

export default Navigation
