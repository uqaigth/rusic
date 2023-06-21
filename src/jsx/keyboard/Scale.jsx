import { useState } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import White from './White.jsx'
import Black from './Black.jsx'


function Scale({ degree = 0 }) {
    return (<div className="w-full h-28 relative bg-gray-500">
        <White pitch="B" degree={degree}/>
        <Black pitch="A#" degree={degree}/>
        <White pitch="A" degree={degree}/>
        <Black pitch="G#" degree={degree}/>
        <White pitch="G" degree={degree}/>
        <Black pitch="F#" degree={degree}/>
        <White pitch="F" degree={degree}/>
        <White pitch="E" degree={degree}/>
        <Black pitch="D#" degree={degree}/>
        <White pitch="D" degree={degree}/>
        <Black pitch="C#" degree={degree}/>
        <White pitch="C" degree={degree}>
            <span className="text-gray-600 absolute right-0.5 leading-4 cursor-default">C{ degree }</span>
        </White>
    </div>)
}

export default Scale
