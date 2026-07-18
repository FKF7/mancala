import simulationCompletedSound from "../assets/sounds/simulationCompleted.mp3";
import simulationErrorSound from "../assets/sounds/simulationError.mp3";

import muteIcon from "../assets/img/muteIcon.svg";
import speakerIcon from "../assets/img/speakerIcon.svg";
import hiddenIcon from "../assets/img/hiddenIcon.png";
import visibleIcon from "../assets/img/visibleIcon.png";
import forbiddenIcon from "../assets/img/forbiddenIcon.png";
import backIcon from "../assets/img/backIcon.png";
import forwardIcon from "../assets/img/forwardIcon.png";

const Constants = Object.freeze({
    MANCALA: {
        NUM_PITS: 6,
        NUM_PLAYERS: 2,
        NUM_PEBBLES: 4,
        MANCALA_PIT: 0,
        PLAYERS: {
            PLAYER1: 0,
            PLAYER2: 1
        },
        INITIAL_BOARD: [
            [0, 4, 4, 4, 4, 4, 4], // Player 1's pits
            [0, 4, 4, 4, 4, 4, 4], // Player 2's pits
        ],
        INITIAL_TURN: 0,
        FULL_CYCLE: 13,
        HINT_TYPE: {
            NORMAL: 0,
            UNKNOWN: 64,
            COMPLETED: 128,
            INVALID: 192,
            HIDDEN: 255
        },
        SOUNDS: {
            SIMULATION_COMPLETED: new Audio(simulationCompletedSound),
            SIMULATION_ERROR: new Audio(simulationErrorSound)
        },
        IMG: {
            MUTE_ICON: muteIcon,
            SPEAKER_ICON: speakerIcon,
            HIDDEN_ICON: hiddenIcon,
            VISIBLE_ICON: visibleIcon,
            FORBIDDEN_ICON: forbiddenIcon,
            BACK_ICON: backIcon,
            FORWARD_ICON: forwardIcon
        }
    },
    PLAYERS: {
        PLAYER1: 0,
        PLAYER2: 1
    },
    CJF: {
        NUM_TILES: 16,
        DUEL_TILES: [0, 8],
        MIN_MOVE: 1,
        MAX_MOVE: 6,
        PLAYERS: {
            PLAYER: 0,
            SENSEI: 1
        },
        INITIAL_POSITION: {
            SENSEI: 0,
            PLAYER: 8
        },
        INITIAL_LIVES: 6,
        TILE_TYPE_KEYS: ['F', 'W', 'S', 'D', 'J'],
        TILES_KEYS: ['D', 'W', 'F', 'S', 'J', 'F', 'W', 'S', 'D', 'F', 'S', 'W', 'J', 'F', 'W', 'S'],
        TILE_PATTERNS: {
            F: 'firePattern',
            W: 'waterPattern',
            S: 'snowPattern',
            D: 'duelPattern',
            J: 'jokerPattern'
        },
        TILE_BORDER_COLORS: {
            F: 'red',
            W: 'blue',
            S: '#74f6ff',
            D: '#059669',
            J: '#7C3AED'
        },
        TILE_FILL_COLORS: {
            F: '#ff5151',
            W: '#3b82f6',
            S: '#b7faff',
            D: '#6EE7B7',
            J: '#D8B4FE'
        },
        TILE_IMAGE_URLS: {
            F: '/fire.webp',
            W: '/water.webp',
            S: '/snow.webp',
            D: '/duel.png',
            J: '/joker.webp'
        },
    }
});

export default Constants;