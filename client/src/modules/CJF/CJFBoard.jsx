import React, { useState } from 'react';
import CJFGame from '../../models/CJFGame';
import Constants from '../../../../common/constants';
import '../../App.css';

const svgSize = 500;
const radius = svgSize / 2.5;
const center = svgSize / 2;
const numTiles = 16;
const tileRadius = 30;
const imgSize = 40;
const playerSize = 50;
const a = 10;

const tiles = Array.from({ length: numTiles }, (_, i) => {
  const angle = (2 * Math.PI * i) / numTiles - Math.PI / 2;
  const x = center + radius * Math.cos(angle);
  const y = center + radius * Math.sin(angle);
  return { x, y, index: i };
});

const moves = Array.from({ length: 6 }, (_, i) => ({
  x: center + (i - 2.5) * 40,
  y: center + 15,
  number: i + 1
}));

export default function FireBoard() {
  const [game, setGame] = useState(new CJFGame());

  const onTileClick = (dir) => {
    game.makeMove(dir);
    setGame((game) => {
      return new CJFGame(game);
    });
  }

  const onNumMoveClick = (numMove) => {
    game.setNumMove(numMove);
    setGame((game) => {
      return new CJFGame(game);
    });
  }

  const onResetButtonClick = () => {
    game.resetGame();
    setGame((game) => {
      return new CJFGame(game);
    });
  }

  return (
    <div className="w-full h-full flex items-center justify-center">
      <svg width={svgSize} height={svgSize}>
        <defs>
          {
            Constants.CJF.TILE_TYPE_KEYS.map((type) => (
              <pattern key={type} id={Constants.CJF.TILE_PATTERNS[type]} patternUnits="objectBoundingBox" width={1} height={1}>
                <rect width="100%" height="100%" fill={Constants.CJF.TILE_FILL_COLORS[type]} />
                <image xlinkHref={Constants.CJF.TILE_IMAGE_URLS[type]} x={a} y={a} width={imgSize} height={imgSize} />
              </pattern>
            ))
          }
        </defs>

        {tiles.map(({ x, y, index }) => {
          let fill = Constants.CJF.TILE_PATTERNS[Constants.CJF.TILES_KEYS[index]];
          let borderColor = Constants.CJF.TILE_BORDER_COLORS[Constants.CJF.TILES_KEYS[index]];

          let clickable = game.getNumMove() && index === (game.getTurnPos() + game.getNumMove()) % Constants.CJF.NUM_TILES || index === (game.getTurnPos() - game.getNumMove() + Constants.CJF.NUM_TILES) % Constants.CJF.NUM_TILES;
          let dir = index === (game.getTurnPos() + game.getNumMove()) % Constants.CJF.NUM_TILES
          let onClick = clickable ? () => {onTileClick(dir);} : null;

          return (
            <g key={index} onClick={onClick} className={`tile${ clickable ? ' clickable' : ''}`}>
              {clickable && (<circle cx={x} cy={y} fill="#F9A825">
                <animate attributeName="r" values={`${36};${38};${36}`} dur="1.25s" repeatCount="indefinite" />
              </circle>)}
              <circle cx={x} cy={y} r={tileRadius} fill={`url(#${fill})`} stroke={borderColor} strokeWidth={3} />
              {index === game.getPlayerPos() && (
                <image xlinkHref="/penguin.png" x={x - playerSize/2} y={y - playerSize/2} textAnchor="middle" width={playerSize} height={playerSize} />
              )}
              {index === game.getSenseiPos() && (
                <image xlinkHref="/sensei.webp" x={x - playerSize/2} y={y - playerSize/2} textAnchor="middle" width={playerSize} height={playerSize} />
              )}
            </g>
          )
        })}
        <text x={center} y={center - 50} fontSize="25" textAnchor="middle" fontWeight="bold" >
          {game.isGameEnded() ? 'Game Over' : `${game.getTurn() === Constants.CJF.PLAYERS.PLAYER ? 'Player' : 'Sensei'}'s Turn`}
        </text>
        <text x={center} y={center - 8} fontSize="18" textAnchor="middle" fontWeight="bold" >
          Select a Movement Number
        </text>
        {moves.map(({ x, y, number }) => {
          let selected = game.getNumMove() === number;
          let className = `numMove${selected ? ' selected' : ''}`;
          let cursor = game.isGameEnded() ? 'default' : 'pointer';
          return (
            <g key={number} onClick={() => {onNumMoveClick(number)}} cursor={cursor}>
              <circle cx={x} cy={y} r={15} fill="#e5e7eb" stroke="#9ca3af" strokeWidth={2} className={className} />
              <text x={x} y={y + 5} textAnchor="middle" fontSize="12" fill="#1f2937" fontWeight="bold">{number}</text>
            </g>
          )
        })}
        <text x={center} y={center + 60} fontSize="18" textAnchor="middle" fontWeight="bold" >
          {`${game.getPlayerLives()} x ${game.getSenseiLives()}`}
        </text>
        <g onClick={onResetButtonClick} cursor="pointer">
          <rect rx={10} ry={10} x={center - 40} y={center + 95} width={80} height={30} fill="#e5e7eb" />
          <text x={center} y={center + 116} fontSize="18" textAnchor="middle" fontWeight="bold" >
            Reset
          </text>
        </g>
      </svg>
    </div>
  );
}
