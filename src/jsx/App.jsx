import { useState } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import '../assets/css/App.css'
import ToolBar from './toolbar/ToolBar.jsx'
import Panel from './panel/Panel.jsx'
import KeyBoard from './keyboard/KeyBoard.jsx'

function App() {
    const [greetMsg, setGreetMsg] = useState('')
    const [name, setName] = useState('')

    async function greet() {
        // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
        setGreetMsg(await invoke('greet', { name }))
    }

    return (<div className="h-screen w-screen flex flex-col bg-gray-800">
        <ToolBar/>
        <div className="w-screen flex-grow flex justify-center overflow-hidden">
            <KeyBoard/>
            <Panel/>
        </div>
    </div>)
}

export default App
