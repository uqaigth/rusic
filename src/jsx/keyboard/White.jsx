import classnames from 'tailwindcss-classnames'

function White({ pitch = 'C', degree = 0, children }) {

    let className = classnames('w-full', 'h-4', 'relative', 'bg-gray-50', 'hover:bg-gray-200', {
        'border-b': pitch !== 'C' || degree !== 0,
        'border-solid': pitch !== 'C' || degree !== 0,
        'border-gray-600': pitch !== 'C' || degree !== 0,
    })

    return (<div className={ className }>
        { children }
    </div>)
}

export default White
