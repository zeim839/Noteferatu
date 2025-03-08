import React, { useState, useEffect, useRef } from "react";

type RecentsCardsProps = {
    title : string
    desc  : string 
    atime : string | Date
    updateDivHeight: (height: number) => void;
}

function timeAgo(timestamp: string | Date): string {
    const now = new Date();
    const past = new Date(timestamp);
    const diffInSeconds = Math.floor((now.getTime() - past.getTime()) / 1000);

    const diffInMinutes = Math.floor(diffInSeconds / 60);
    if (diffInMinutes < 60) return `${diffInMinutes}m ago`;
    const diffInHours = Math.floor(diffInMinutes / 60);
    if (diffInHours < 24) return `${diffInHours}h ago`;
    const diffInDays = Math.floor(diffInHours / 24);
    if (diffInDays < 30) return `${diffInDays}d ago`;
    const diffInMonths = Math.floor(diffInDays / 30);
    if (diffInMonths < 12) return `${diffInMonths}mo ago`;
    const diffInYears = Math.floor(diffInMonths / 12);
    return `${diffInYears}y ago`;
} 
export default function RecentsCard({title, desc, atime, updateDivHeight} : RecentsCardsProps) {
    console.log(atime)
    const refContainer = useRef<HTMLDivElement>(null);
    useEffect(() => {
        if (refContainer.current) {
            updateDivHeight(refContainer.current.offsetHeight);
            console.log('current div height is',refContainer.current.offsetHeight)
        }
    }, []);
    
    return (
        <div ref = {refContainer} className='w-[344px] min-h-[76px] bg-white rounded-md border border-[#979797] grid grid-cols-[3fr_1fr] my-1'>
            <div className="p-2">
                <p className='font-extrabold text-sm line-clamp-1'>{title}</p>
                <p className='font-light text-sm line-clamp-2'>{desc}</p>
            </div>
            <div className='flex items-center justify-center'>
                <p className='text-sm font-light'>{timeAgo(atime)}</p>
            </div>
        </div>
    );
}