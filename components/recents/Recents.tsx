import RecentsCard from "./RecentsCard"
import useWindowDimensions from "./WindowHeight";
import React, { useState, useEffect, useMemo} from "react";
import {getNotesData,createNote} from "../../lib/DatabaseFunctions";
import Database from '../../lib/Database';
import { appLocalDataDir } from '@tauri-apps/api/path';

type NoteData = {
    id: number
    title: string,
    content: string;
    atime: number
}

async function getRecents(queryAmount: number): Promise<NoteData[] | null> {
    try {
        const appDataDir = await appLocalDataDir();
        const dbPath = `${appDataDir}database.sqlite`;
        console.log(dbPath)
        const db = new Database(dbPath);
        await db.connect();
        console.log('Database connected');
        const retrievedNotes = await getNotesData(db,queryAmount);
        console.log('getnotes',retrievedNotes)
        return retrievedNotes as NoteData[];
    } catch (error) {
        console.error('Could not connect to database',error);
        return null
    }
}

export default function Recents() {
    const [cardCount, setCardCount] = useState(() => {
         return 0;
    });
    const height = useWindowDimensions()
    const divHeight = 77;

    const [recentsData, setRecentsData] = useState<NoteData[]>([]);
    // 60 is the height the search bar takes up
    const avaialableHeight = height - 60

    useMemo(() => {
        if (avaialableHeight > 0){
            setCardCount(Math.round((avaialableHeight)/(divHeight+8)))
            console.log('card changed');
            const now = new Date();
            console.log(now.getTime(),' balls ', Date.now())
            console.log("current time stamp in seconds is",Math.floor(now.getTime() - Date.now()))
        }
    }, [avaialableHeight]);

    useEffect(() => {
    async function fetchData(){
        try {
            const data = await getRecents(cardCount);
            setRecentsData(data || []);
        } catch (error) {
            console.error('Error fetching recents:', error);
            setRecentsData([]);
        }
    }
    fetchData();
}, [cardCount]);

    if (recentsData.length > 0){
        console.log('entered')
    const recentsCardsList = recentsData.slice(0, cardCount).map((note, i) => (
        <div key={i} className="opacity-0 animate-fade-in" style={{ animationDelay: `${i * 0.06}s` }}>
            <RecentsCard 
                title={note.title} 
                desc={note.content}
                atime={note.atime} 
            />
        </div>
    ));
    return (
        <>
            {recentsCardsList}
        </>
    );
    }
    else {
        return (
            <>
                <p className="mt-2 text-xl font-bold text-gray-700">
                     Create a note to get started
                </p>
            </>
        );
    }

}

