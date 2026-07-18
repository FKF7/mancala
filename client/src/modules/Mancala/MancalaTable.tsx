import React, { useEffect, ReactElement, useState, useRef } from 'react';
import MancalaGame from '../../models/mancalaGame.model';
import History from '../../models/history';
import Constants from '../../utils/constants';
import Routes from '../../utils/routes';
import '../../App.css';
import './MancalaTable.css';
import { MancalaTurn, Pit, Hint } from '../../types/types';
import { executeGETRequest } from '../../utils/requestExecutor';
import MancalaBoard from './MancalaBoard';
import { getHintData, printPath, simulate } from '../../lib/requests';
import MancalaKeyBinds from "./MancalaKeyBinds";
import LoadingDots from '../../components/loading/LoadingDots';
import ProcessButton from '../../components/ProcessButton/ProcessButton';
import { SimulationResultStatus } from '../../enums/enums';
import SimulationResultDialog from '../../components/simulationResultDialog/SimulationResultDialog';
import { SimulationResultDialogProps } from '../../components/simulationResultDialog/SimulationResultDialog';
import FlagButton from '../../components/FlagButton/FlagButton';

export default function MancalaTable() {
  const [game, setGame] = useState(() => new MancalaGame());
  const [hints, setHints] = useState<Hint[]>([]);
  let [hideHints, setHideHints] = useState(true);
  let [isMute, setIsMute] = useState(false);
  let [isSimulating, setIsSimulating] = useState(false);
  let [interruptSim, setInterruptSim] = useState(false);
  const interruptSimRef = useRef(false);

  const requestInterrupt = () => {
    interruptSimRef.current = true;
    setInterruptSim(true);
  };

  useEffect(() => {
    if (hideHints) {
      setHints(Array(6).fill({ hintType: Constants.MANCALA.HINT_TYPE.HIDDEN, value: 0 } as Hint)); 
      return;
    }

    getHintData(game).then((hints) => {
      if (hints) {
        setHints(hints)
      } else {
        setHints([]);
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



  const mancalaKeysHandler = new MancalaKeyBinds();

  useEffect(() => {
    mancalaKeysHandler.setAction("undo", onZClick);
    mancalaKeysHandler.setAction("redo", onYClick);
    mancalaKeysHandler.setAction("reset", onResetButtonClick);
  }, []);
  

  // window.addEventListener("keyup", mancalaKeysHandler.onKeyPress);

  // const onKeyPress = (e: React.KeyboardEvent<HTMLDivElement>) => {
  //   mancalaKeysHandler.onKeyPress(e);
  // };

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
      await simulate(game, 5000).then((result) => {
        if (result.hints) {
          setHints(result.hints)
          totalEnds += result.ends;
        }
      }).catch(() => {
        setIsSimulating(false);
        playSound(Constants.MANCALA.SOUNDS.SIMULATION_ERROR);
        error = true;
        setSimulationDialog({
          open: true,
          status: SimulationResultStatus.ERROR,
          message: `An error occurred while simulating at ${totalEnds} ends reached.`
        });
      });
    } while (totalEnds % 5000 === 0 && !error && !interruptSimRef.current);
    
    if (error) {
      return;
    }

    playSound(Constants.MANCALA.SOUNDS.SIMULATION_COMPLETED);
    setIsSimulating(false);
    if (totalEnds >= 5000) {
      setSimulationDialog({
        open: true,
        status: SimulationResultStatus.SUCCESS,
        message: `Simulated ${totalEnds} game ends!`
      });
    }
  }

  function onHelpButtonClick() {
    printPath(game);
  }

  return (
    <div>
      <div className="board-area">
        <div className={isSimulating ? "board-disabled" : ""}></div>
      <MancalaBoard game={game} setGame={setGame} history={history} hints={hints} isSimulating={isSimulating} mancalaKeysHandler={mancalaKeysHandler}/>
      {isSimulating && (
        <div className="board-loading-overlay">
          <LoadingDots visible={true} />
        </div>
      )}
    </div>
      <div className="buttons">
        <button onClick={onHelpButtonClick} disabled={isSimulating}>
          Help!
        </button>
        <button onClick={onResetButtonClick} disabled={isSimulating}>
          Reset
        </button>
        <button onClick={onZClick} disabled={isSimulating}>
          {<img className="icon" src={Constants.MANCALA.IMG.BACK_ICON} alt="Undo" />}
        </button>
        <button onClick={onFixClick} disabled={isSimulating}>
          Fix
        </button>
        <button onClick={onYClick} disabled={isSimulating}>
          {<img className="icon" src={Constants.MANCALA.IMG.FORWARD_ICON} alt="Redo" />}
        </button>
        <FlagButton
          contentTrue={<img className="icon" src={Constants.MANCALA.IMG.HIDDEN_ICON} alt="Hide Hints" />}
          contentFalse={<img className="icon" src={Constants.MANCALA.IMG.VISIBLE_ICON} alt="Show Hints" />}
          flag={hideHints}
          setFlag={setHideHints}
          disabled={isSimulating}/>
        <FlagButton
          contentTrue={<img className="icon" src={Constants.MANCALA.IMG.MUTE_ICON} alt="Sound On" />}
          contentFalse={<img className="icon" src={Constants.MANCALA.IMG.SPEAKER_ICON} alt="Sound Off" />}
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