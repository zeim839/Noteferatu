import RecentsCard from "./RecentsCard"
import useWindowDimensions from "./WindowHeight";
import React, { useState, useEffect, useMemo} from "react";
import {getNotesData,createNote, deleteNote} from "../../lib/DatabaseFunctions";
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
        const retrievedNotes = await getNotesData(db,queryAmount);
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
    const [recentsData, setRecentsData] = useState<NoteData[] | null>([]);
    // 60 is the height the search bar takes up
    const avaialableHeight = height - 60

    const [isLoading, setIsLoading] = useState(true);
    
    useMemo(() => {
        if (height > 0){
            setCardCount(Math.round((avaialableHeight)/(divHeight+8)))
        }
    }, [height]);

    useEffect(() => {
        async function fetchData(){
            try {
                const data = await getRecents(cardCount);
                setRecentsData(data);
                console.log("data fetched");
            } catch (error) {
                console.error('Error fetching recents:', error);
                setRecentsData(null);
            } finally {
                setIsLoading(false);
            }
        }
        fetchData();
    }, [cardCount]);
    
    if (isLoading){
        return(
            <>
            </>
        );
    }
    if (recentsData && recentsData.length > 0){
        console.log('first condition has been entered')
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
            <div className="h-full">
                {recentsCardsList}
            </div>
        );
    }
    if (recentsData === null){
        return(
        <div className="flex h-full items-center justify-center">
            <p className="text-xl font-bold text-red-700">            
                Unable to connect to Database
            </p>
        </div>
        );
    }
    else {
        return (
            <div className="flex h-full items-center justify-center">
                <p className="text-xl font-bold text-gray-700">
                    Create a note to get started
                </p>
            </div>

        );
    }

}

