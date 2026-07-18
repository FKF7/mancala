import React, { useEffect, ReactElement, useState } from 'react';
import MancalaGame from '../../models/mancalaGame.model';
import History from '../../models/history';
import Constants from '../../utils/constants';
import '../../App.css';
import './MancalaBoard.css';
import { MancalaTurn, Pit, Hint } from '../../types/types';
import { makeMove } from '../../lib/requests';
import MancalaKeyBinds from './MancalaKeyBinds';

type MancalaBoardProps = {
  game: MancalaGame;
  setGame: React.Dispatch<React.SetStateAction<MancalaGame>>;
  history: History;
  hints: Hint[];
  isSimulating: boolean;
  mancalaKeysHandler: MancalaKeyBinds;
}

export default function MancalaBoard({ game, setGame, history, hints, isSimulating, mancalaKeysHandler }: MancalaBoardProps) {

  useEffect(() => {
    mancalaKeysHandler.setAction("movePit", onPitClick);
  }, [game]);

  async function onPitClick(pit: Pit) {
    if (game.getCurrentBoardPit(pit) === 0) {
      return;
    }
    let newGame = await makeMove(game, pit);
    if (newGame) {
      setGame(newGame);
      history.add(newGame);
    }
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

  function drawHint(hint: Hint, player: MancalaTurn, hintIndex: number) {
    if (game.getCurrentTurn() !== player) {
      return (
        <div key={`hint-${player}-${hintIndex}`} className='hint disabled'>
          <img className="pit-icon" src={Constants.MANCALA.IMG.FORBIDDEN_ICON} alt="Forbidden" />
        </div>
      )
    }

    switch (hint?.hintType) {
      case Constants.MANCALA.HINT_TYPE.HIDDEN:
        return (
        <div className='hint disabled'>
          <img className="pit-icon" src={Constants.MANCALA.IMG.HIDDEN_ICON} alt="Hidden" />
        </div>
      );
      case Constants.MANCALA.HINT_TYPE.NORMAL:
        return (
          <div key={`hint-${player}-${hintIndex}`} className='hint'>
            {hint.value}
          </div>
        );
      case Constants.MANCALA.HINT_TYPE.UNKNOWN:
        return (
          <div key={`hint-${player}-${hintIndex}`} className='hint unknown'>
            ?
          </div>
        );
      case Constants.MANCALA.HINT_TYPE.COMPLETED:
        return (
          <div key={`hint-${player}-${hintIndex}`} className='hint completed'>
            {hint.value}
          </div>
        );
      case Constants.MANCALA.HINT_TYPE.INVALID:
        return (
          <div key={`hint-${player}-${hintIndex}`} className='hint disabled'>
            <img className="pit-icon" src={Constants.MANCALA.IMG.FORBIDDEN_ICON} alt="Forbidden" />
          </div>
        );
      default:
        return <div key={`hint-${player}-${hintIndex}`} className='hint weird'>
          ??
        </div>
    }
  }

  function drawPlayer1Hints() {
    const hintElements: ReactElement[] = []
    for (let i = 5; i >= 0; i--) {
      hintElements.push(drawHint(hints[i], Constants.PLAYERS.PLAYER1 as MancalaTurn, i));
    }
    return hintElements;
  }

  function drawPlayer2Hints() {
    const hintElements: ReactElement[] = []
    for (let i = 0; i < 6; i++) {
      hintElements.push(drawHint(hints[i], Constants.PLAYERS.PLAYER2 as MancalaTurn, i));
    }
    return hintElements;
  }

  return (
    <div className={`board-container ${isSimulating ? 'board-disabled' : ''}`}>
      <div className='hint-grid'>
        {drawPlayer2Hints()}
      </div>
      <div className="board">
        <div className="store player2Board">{game.getPit(Constants.PLAYERS.PLAYER2 as MancalaTurn, 0)}</div>
        <div className="pit-grid">
          {drawPlayer2Pits()}
          {drawPlayer1Pits()}
        </div>
        <div className="store player1Board">{game.getPit(Constants.PLAYERS.PLAYER1 as MancalaTurn, 0)}</div>
      </div>
      <div className='hint-grid'>
        {drawPlayer1Hints()}
      </div>
    </div>
  );
  
}