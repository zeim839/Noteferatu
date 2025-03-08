import RecentsCard from "./RecentsCard"
import useWindowDimensions from "./WindowHeight";
import React, { useState, useEffect, useRef, useMemo } from "react";


export default function Recents() {
    const [cardCount, setCardCount] = useState(0);
    const height = useWindowDimensions()
    
    const [divHeight, setDivHeight] = useState(76);

    const updateDivHeight = (height: number) => {
        setDivHeight(height);
      };
    
    // 60 is the height the search bar takes up
    const avaialableHeight = height - 60
    useMemo(() => {
        if (divHeight > 0){
        setCardCount(Math.round((avaialableHeight)/(divHeight+8)))
    }
    }, [divHeight,avaialableHeight]);
    console.log(cardCount,"this is card count")
    console.log(avaialableHeight);
    console.log(divHeight);
    const recentsCardsList = [];
    for (let i = 0; i < cardCount; i++){
        recentsCardsList.push(<div key={i}  className="opacity-0 animate-fade-in" style={{ animationDelay: `${i * 0.1}s` }}>
            <RecentsCard title="Open Source Club Fortnite balls lowasn " desc="In" atime={"2025-03-06 12:00:00"} updateDivHeight={updateDivHeight}/>
        </div>
        );
    }
    return (
        <>
            {recentsCardsList}
        </>
    );
}

