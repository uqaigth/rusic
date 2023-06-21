import { KEY_BOARD_SCROLL, PANEL_SCROLL } from '../action.js'

const scrollInitial = {
    keyboardScroll: 0, paneScroll: 0,
}

export function scroll(state = scrollInitial, action) {
    switch (action.type) {
        case KEY_BOARD_SCROLL:
            return {
                ...state, keyboardScroll: action.payload,
            }
        case PANEL_SCROLL:
            return {
                ...state, paneScroll: action.payload,
            }
        default:
            return state
    }
}

