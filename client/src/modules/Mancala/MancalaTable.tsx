import { useEffect, useState, useRef } from "react";
import { toBlob } from "html-to-image";
import MancalaGame from "../../models/mancalaGame.model";
import History from "../../models/history";
import Constants from "../../utils/constants";
import { HintData } from "../../types/types";
import MancalaBoard from "./MancalaBoard";
import { getHintData, printPath, simulate } from "../../lib/requests";
import MancalaKeyBinds from "./MancalaKeyBinds";
import LoadingDots from "../../components/loading/LoadingDots";
import ProcessButton from "../../components/ProcessButton/ProcessButton";
import { SimulationResultStatus } from "../../enums/enums";
import SimulationResultDialog from "../../components/simulationResultDialog/SimulationResultDialog";
import FlagButton from "../../components/FlagButton/FlagButton";

import "../../App.css";
import "./MancalaTable.css";

export default function MancalaTable() {
  const [game, setGame] = useState(() => new MancalaGame());
  const [hintData, setHintData] = useState<HintData>({
    hints: [],
    min: 0,
    max: 0,
  });
  let [hideHints, setHideHints] = useState(true);
  let [isMute, setIsMute] = useState(false);
  let [isSimulating, setIsSimulating] = useState(false);
  let [interruptSim, setInterruptSim] = useState(false);
  const mancalaKeysHandlerRef = useRef<MancalaKeyBinds | null>(null);
  const interruptSimRef = useRef(false);

  if (mancalaKeysHandlerRef.current === null) {
    mancalaKeysHandlerRef.current = new MancalaKeyBinds();
  }
  const mancalaKeysHandler = mancalaKeysHandlerRef.current;

  useEffect(() => {
    mancalaKeysHandler.setAction("undo", onZClick);
    mancalaKeysHandler.setAction("redo", onYClick);
    mancalaKeysHandler.setAction("reset", onResetButtonClick);

    mancalaKeysHandler.start();

    return () => {
      mancalaKeysHandler.stop();
    };
  }, [mancalaKeysHandler]);

  const requestInterrupt = () => {
    interruptSimRef.current = true;
    setInterruptSim(true);
  };

  const boardRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (hideHints) {
      setHintData({
        hints: Array(6).fill({
          hintType: Constants.HINT_TYPE.HIDDEN,
          value: 0,
          min: 0,
          max: 0,
        }),
        min: 0,
        max: 0,
      });
      return;
    }

    getHintData(game).then((hint_data) => {
      if (hint_data?.hints) {
        setHintData(hint_data);
      } else {
        setHintData({
          hints: [],
          min: 0,
          max: 0,
        });
      }
    });
  }, [game, hideHints]);

  const [history] = useState(() => new History(game));

  const [simulationDialog, setSimulationDialog] = useState<{
    open: boolean;
    status: SimulationResultStatus;
    message: string;
  }>({
    open: false,
    status: SimulationResultStatus.SUCCESS,
    message: "",
  });

  async function copyBoard() {
    if (!boardRef.current) return;

    const blob = await toBlob(boardRef.current);

    if (!blob) return;

    await navigator.clipboard.write([
      new ClipboardItem({
        "image/png": blob,
      }),
    ]);
  }

  async function onResetButtonClick() {
    let newGame = new MancalaGame();
    setGame(newGame);
    history.reset(newGame);
  }

  function onZClick() {
    setGame(history.back());
  }

  function onFixClick() {
    history.fix();
  }

  function onYClick() {
    setGame(history.forward());
  }

  function playSound(sound: HTMLAudioElement) {
    if (!isMute) {
      sound.play();
    }
  }

  async function onSimulateClick() {
    setIsSimulating(true);
    setInterruptSim(false);
    interruptSimRef.current = false;
    let totalEnds = 0;
    let error = false;

    do {
      await simulate(game, 5000, true)
        .then((result) => {
          if (result.hints) {
            setHintData({
              hints: result.hints,
              min: result.min,
              max: result.max,
            });
            totalEnds += result.ends;
          }
        })
        .catch(() => {
          setIsSimulating(false);
          playSound(Constants.SOUNDS.SIMULATION_ERROR);
          error = true;
          setSimulationDialog({
            open: true,
            status: SimulationResultStatus.ERROR,
            message: `An error occurred while simulating at ${totalEnds} ends reached.`,
          });
        });
    } while (totalEnds % 5000 === 0 && !error && !interruptSimRef.current);

    if (error) {
      return;
    }

    playSound(Constants.SOUNDS.SIMULATION_COMPLETED);
    setIsSimulating(false);
    if (totalEnds >= 5000) {
      setSimulationDialog({
        open: true,
        status: SimulationResultStatus.SUCCESS,
        message: `Simulated ${totalEnds} game ends!`,
      });
    }
  }

  function onHelpButtonClick() {
    printPath(game);
  }

  return (
    <div>
      <div className="board-area" ref={boardRef}>
        <div className={isSimulating ? "board-disabled" : ""}></div>
        <MancalaBoard
          game={game}
          setGame={setGame}
          history={history}
          hintData={hintData}
          isSimulating={isSimulating}
          mancalaKeysHandler={mancalaKeysHandler}
        />
        {isSimulating && (
          <div className="board-loading-overlay">
            <LoadingDots visible={true} />
          </div>
        )}
      </div>
      <div className="buttons">
        <button onClick={copyBoard} disabled={isSimulating}>
          {<img className="icon" src={Constants.IMG.COPY_ICON} alt="Copy" />}
        </button>
        <button onClick={onHelpButtonClick} disabled={isSimulating}>
          Help!
        </button>
        <button onClick={onResetButtonClick} disabled={isSimulating}>
          Reset
        </button>
        <button onClick={onZClick} disabled={isSimulating}>
          {<img className="icon" src={Constants.IMG.BACK_ICON} alt="Undo" />}
        </button>
        <button onClick={onFixClick} disabled={isSimulating}>
          Fix
        </button>
        <button onClick={onYClick} disabled={isSimulating}>
          {<img className="icon" src={Constants.IMG.FORWARD_ICON} alt="Redo" />}
        </button>
        <FlagButton
          contentTrue={
            <img
              className="icon"
              src={Constants.IMG.HIDDEN_ICON}
              alt="Hide Hints"
            />
          }
          contentFalse={
            <img
              className="icon"
              src={Constants.IMG.VISIBLE_ICON}
              alt="Show Hints"
            />
          }
          flag={hideHints}
          setFlag={setHideHints}
          disabled={isSimulating}
        />
        <FlagButton
          contentTrue={
            <img
              className="icon"
              src={Constants.IMG.MUTE_ICON}
              alt="Sound On"
            />
          }
          contentFalse={
            <img
              className="icon"
              src={Constants.IMG.SPEAKER_ICON}
              alt="Sound Off"
            />
          }
          flag={isMute}
          setFlag={setIsMute}
        />
        <ProcessButton
          executing={isSimulating}
          startFunction={onSimulateClick}
          interrupting={interruptSim}
          requestInterrupt={requestInterrupt}
          startText="Matrix"
        />
      </div>
      <SimulationResultDialog
        open={simulationDialog.open}
        status={simulationDialog.status}
        message={simulationDialog.message}
        onClose={() =>
          setSimulationDialog((prev) => ({
            ...prev,
            open: false,
          }))
        }
      />
    </div>
  );
}
