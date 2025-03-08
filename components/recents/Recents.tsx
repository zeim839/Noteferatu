import RecentsCard from "./RecentsCard"
import useWindowDimensions from "./WindowHeight";
import React, { useState, useEffect, useMemo} from "react";
import {getNotesData, createNote} from "../../lib/DatabaseFunctions";
import Database from '../../lib/Database';
import { appLocalDataDir } from '@tauri-apps/api/path';

type NoteData = {
    id: number
    title: string,
    content: string;
    atime: Date
    mtime: Date
}

async function getRecents(queryAmount: number): Promise<NoteData[] | null> {
    try {
        const appDataDir = await appLocalDataDir();
        const dbPath = `${appDataDir}database.sqlite`;
        console.log(dbPath)
        const db = new Database(dbPath);
        await db.connect();
        console.log('Database connected');
        createNote(db,'JOJO IS THE GREATEST SHOW EVER','WAMU NOOOOO I LOVED YOU FOREVER AND EVER AND EVER AND EVER')
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
    const [divHeight, setDivHeight] = useState(() =>{
        // 76 is the minimum height of the recents card
        return 76;
    });

    const updateDivHeight = (height: number) => {
        setDivHeight(height);
      };

    const [recentsData, setRecentsData] = useState<NoteData[]>([]);
    // 60 is the height the search bar takes up
    const avaialableHeight = height - 60
    useMemo(() => {
        if (divHeight > 0){
            // 8 is height of vertical margin above and below card
            setCardCount(Math.round((avaialableHeight)/(divHeight+8)))
        }
    }, [divHeight,avaialableHeight]);

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

    const recentsCardsList = recentsData.slice(0, cardCount).map((note, i) => (
        <div key={i} className="opacity-0 animate-fade-in" style={{ animationDelay: `${i * 0.06}s` }}>
            <RecentsCard 
                title={note.title} 
                desc={note.content}
                atime={note.atime} 
                updateDivHeight={updateDivHeight}
            />
        </div>
    ));
    return (
        <>
            {recentsCardsList}
        </>
    );
}

