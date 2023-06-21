import { configureStore } from '@reduxjs/toolkit'
import { scroll } from './reducers/scroll.js'


export default configureStore({
    reducer: {
        scroll,
    },
})
