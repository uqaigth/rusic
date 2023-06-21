import { useState } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import Scale from './Scale.jsx'


function KeyBoard() {
    const degrees = [0, 1, 2, 3, 4, 5, 6, 7, 8]


    return (<div className="h-full w-24 bg-gray-500 overflow-y-scroll no-scrollbar">
        { degrees.reverse().map(degree => <Scale degree={ degree }/>) }
    </div>)
}

export default KeyBoard
