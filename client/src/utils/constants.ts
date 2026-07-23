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
  NUM_PITS: 6,
  NUM_PLAYERS: 2,
  NUM_PEBBLES: 4,
  MANCALA_PIT: 0,
  PLAYERS: {
    PLAYER1: 0,
    PLAYER2: 1,
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
    HIDDEN: 255,
  },
  SOUNDS: {
    SIMULATION_COMPLETED: new Audio(simulationCompletedSound),
    SIMULATION_ERROR: new Audio(simulationErrorSound),
  },
  IMG: {
    MUTE_ICON: muteIcon,
    SPEAKER_ICON: speakerIcon,
    HIDDEN_ICON: hiddenIcon,
    VISIBLE_ICON: visibleIcon,
    FORBIDDEN_ICON: forbiddenIcon,
    BACK_ICON: backIcon,
    FORWARD_ICON: forwardIcon,
  },
});

export default Constants;
