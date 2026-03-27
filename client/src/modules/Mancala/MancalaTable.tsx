import React, { useEffect, ReactElement, useState } from 'react';
import MancalaGame from '../../../../common/models/mancalaGame.model.ts';
import Constants from '../../../../common/constants.ts';
import '../../App.css';
import './MancalaBoard.css';
import { MancalaTurn, Pit, Hint } from '../../../../common/types.ts';
import { executeGETRequest } from '../../utils/requestExecutor.ts';
import Routes from '../../../../common/routes';
import History from '../../../../common/models/history.ts';
import MancalaBoard from './MancalaBoard';
import { getHintData, simulate } from '../../lib/requests.ts';
import MancalaKeyBinds from "./MancalaKeyBinds";

export default function MancalaTable() {
  const [game, setGame] = useState(() => new MancalaGame());
  const [hints, setHints] = useState<Hint[]>([]);

  useEffect(() => {
    getHintData(game).then((hints) => {
      if (hints) {
        setHints(hints)
      } else {
        setHints([]);
      }
    });
  }, [game]);

  const [history] = useState(() => new History(game));

  
  
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

  async function onSimulateClick() {
    await simulate(game).then((hints) => {
      if (hints) {
        setHints(hints)
      }
    });
  }

  return (
    <div>
      <MancalaBoard game={game} setGame={setGame} history={history} hints={hints} mancalaKeysHandler={mancalaKeysHandler}/>
      <div className="buttons">
        <button onClick={onResetButtonClick}>Reset</button>
        <button onClick={onZClick}>Z</button>
        <button onClick={onFixClick}>Fix</button>
        <button onClick={onYClick}>Y</button>
        <button onClick={onSimulateClick} disabled={game.getFrozenPebbles() < 33}>Matrix</button>
      </div>
    </div>
  );
}