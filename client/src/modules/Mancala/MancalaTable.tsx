import React, { useEffect, ReactElement, useState } from 'react';
import MancalaGame from '../../../../common/models/mancalaGame.model.ts';
import Constants from '../../../../common/constants.ts';
import '../../App.css';
import './MancalaBoard.css';
import { MancalaTurn, Pit, Hint } from '../../../../common/types.ts';
import { executeGETRequest } from '../../utils/requestExecutor.ts';
import Routes from '../../../../common/routes';
import History from '../../../../common/models/history.ts';
import MancalaBoard from './MancalaBoard.tsx';

export default function MancalaTable() {
  const game = new MancalaGame()
  const [history] = useState(() => new History(game));
  const hint = new Array(6).fill(Constants.MANCALA.EMPTY_HINT);
  const a = MancalaBoard(game);

  function drawHint(hint: number) {
    return (
      <div className='hint'>
        {hint}
      </div>
    )
  }

  function drawPlayer1Hints() {
    const hints: ReactElement[] = []
    for (let i = 5; i >= 0; i--) {
      hints.push(drawHint(hint[i]));
    }
    return hints;
  }

  function drawPlayer2Hints() {
    const hints: ReactElement[] = []
    for (let i = 0; i < 6; i++) {
      hints.push(drawHint(hint[i]));
    }
    return hints;
  }

  function onResetButtonClick() {
    let newGame = new MancalaGame();
    // setGame(newGame);
    history.reset(newGame);
  }

  async function onSimulateButtonClick() {
    // Vai por mim, melhor não kkkk
    // let newGame = await executeGETRequest(`${Routes.BASE_URL}${Routes.API_URL.MANCALA}${Routes.ENDPOINTS.MANCALA.SIMULATE_GAME}`, {});
  }

  function onZClick() {
    console.log(a)
    // setGame(history.back());
  }

  function onFixClick() {
    history.fix();
  }

  function onYClick() {
    a.props.onYClick()
    // setGame(
    // history.forward());
  }

  return (
    <div>
      <div className='hint-grid'>
          {drawPlayer2Hints()}
        </div>
      {a}
      <div className='hint-grid'>
          {drawPlayer1Hints()}
        </div>
      <div className="buttons">
        <button onClick={onResetButtonClick}>Reset</button>
        <button onClick={onSimulateButtonClick}>Simulate</button>

        <button onClick={onZClick}>Z</button>
        <button onClick={onFixClick}>Fix</button>
        <button onClick={onYClick}>Y</button>
      </div>
    </div>
  );
}