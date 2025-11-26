import React, { useEffect, ReactElement, useState } from 'react';
import MancalaGame from '../../../../common/models/mancalaGame.model.ts';
import Constants from '../../../../common/constants.ts';
import '../../App.css';
import './MancalaBoard.css';
import { MancalaTurn, Pit, Hint } from '../../../../common/types.ts';
import { executeGETRequest } from '../../utils/requestExecutor.ts';
import Routes from '../../../../common/routes';
import History from '../../../../common/models/history.ts';

export default function MancalaBoard(game2: MancalaGame) {
  const [game, setGame] = useState(new MancalaGame());
  const [history] = useState(() => new History(game));
  const hint = new Array(6).fill(Constants.MANCALA.EMPTY_HINT);

  async function onPitClick(pit: Pit) {
    let newGame = await executeGETRequest(`${Routes.BASE_URL}${Routes.API_URL.MANCALA}${Routes.ENDPOINTS.MANCALA.MAKE_MOVE}`,
      { pit, game: JSON.stringify(game) });

    newGame = new MancalaGame(newGame.board, newGame.currentTurn as MancalaTurn)
    setGame(newGame);
    history.add(newGame);
  }

  function drawPit(player: MancalaTurn, pit: Pit) {
    let clickable = game.getPit(player, pit) > 0 && game.getCurrentTurn() === player;
    let className = `pit player${player as number + 1}Board ${clickable ? ' clickable' : ''}`;
    let onClick = clickable ? () => {onPitClick(pit);} : () => {};
    return (
      <div key={`p${pit + 1}-${pit}`} className={className} onClick={onClick}>
        {game.getPit(player, pit)}
      </div>
    )
  }

  

  function drawPlayer1Pits() {
    const pits: ReactElement[] = []
    for (let i = 6; i > 0; i--) {
      pits.push(drawPit(Constants.PLAYERS.PLAYER1 as MancalaTurn, i as Pit));
    }
    return pits;
  }

  function drawPlayer2Pits() {
    const pits: ReactElement[] = [];
    for (let i = 1; i < 7; i++) {
      pits.push(drawPit(Constants.PLAYERS.PLAYER2 as MancalaTurn, i as Pit));
    }
    return pits;
  }

  function onResetButtonClick() {
    let newGame = new MancalaGame();
    setGame(newGame);
    history.reset(newGame);
  }

  async function onSimulateButtonClick() {
    // Vai por mim, melhor não kkkk
    // let newGame = await executeGETRequest(`${Routes.BASE_URL}${Routes.API_URL.MANCALA}${Routes.ENDPOINTS.MANCALA.SIMULATE_GAME}`, {});
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

  return (
    <div>
      <div className="board">
        <div className="store player2Board">{game.getPit(Constants.PLAYERS.PLAYER2 as MancalaTurn, 0)}</div>
        <div className="pit-grid">
          {drawPlayer2Pits()}
          {drawPlayer1Pits()}
        </div>
        <div className="store player1Board">{game.getPit(Constants.PLAYERS.PLAYER1 as MancalaTurn, 0)}</div>
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