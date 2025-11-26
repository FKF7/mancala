import MancalaGame from '../../common/models/mancalaGame.model.js';
import Constants from '../../common/constants.js';
import { MancalaTurn, Pit } from '../../common/types.js';
import { Request, Response } from 'express';

export default class MancalaGameController {
    private gamesSimulated: number;
    private nodeCount: number;

    constructor() {
        this.gamesSimulated = 0;
        this.nodeCount = 0;
        this.simulateGame = this.simulateGame.bind(this);
        this.handleMoveRequest = this.handleMoveRequest.bind(this);
    }

    public handleMoveRequest(req: any, res: any) {
        const gameData = JSON.parse(req.query.game as string);
        const game = new MancalaGame(gameData.board, gameData.currentTurn as MancalaTurn);
        let pit = Number(req.query.pit) as Pit | 0;

        this.makeMove(game, pit);
        
        res.json(game);
    }

    public simulateGame(req: any, res: any) {
        const game = new MancalaGame();
        this.nodeCount = 1;
        this.gamesSimulated = 0;

        for (let i = 1; i <= Constants.MANCALA.NUM_PITS; i++) {
            if (this.isPitClickable(game, i as Pit)) {
                this.gameSimulation(game, i as Pit);
                console.log("Passou da primeira parte");
            }
        }

        res.json({
            gamesSimulated: this.gamesSimulated,
            nodeCount: this.nodeCount
        })

        console.log(`Games simulated: ${this.gamesSimulated}`);
        console.log(`Node count: ${this.nodeCount}`);
    }

    private gameSimulation(game: MancalaGame, pit: Pit) {
        this.nodeCount++;
        this.makeMove(game, pit);
        if (game.getCurrentTurn() === null) {
            this.gamesSimulated++;
            console.log(`Game ended. Total games simulated: ${this.gamesSimulated}`);
            return;
        } else {
            for (let i = 1; i <= Constants.MANCALA.NUM_PITS; i++) {
                if (this.isPitClickable(game, i as Pit)) {
                    this.gameSimulation(new MancalaGame(game.getBoard(), game.getCurrentTurn()), i as Pit);
                }
            }
        }
    }

    private makeMove(game: MancalaGame, pit: Pit | 0) {
        let boardSide = game.getCurrentTurn() as 0 | 1;
        if (boardSide !== null) {
            let pebbles = game.getCurrentBoardPit(pit);
            const freeTurn = (pebbles % Constants.MANCALA.FULL_CYCLE) === pit;
            game.resetCurrentBoardPit(pit);

            while (pebbles > 0) {
                if (pit > Constants.MANCALA.MANCALA_PIT) {
                    if (pit === 1 && boardSide !== game.getCurrentTurn()) {
                        pit = Constants.MANCALA.NUM_PITS as Pit;
                        boardSide = this.switchTurnNumber(boardSide);
                    } else {
                        pit--;
                    }
                } else {
                    pit = Constants.MANCALA.NUM_PITS as Pit;
                    boardSide = this.switchTurnNumber(boardSide);
                }
                game.incrementPit(boardSide as MancalaTurn, pit as Pit);
                pebbles--;
            }

            if (game.getPit(boardSide, pit) === 1 && boardSide === game.getCurrentTurn() && pit !== 0 && game.getOppositePit(boardSide, pit) !== 0) {
                game.incrementCurrentBoardPit(0, game.getOppositePit(game.getCurrentTurn(), pit) + 1);
                game.resetCapturePits(boardSide, pit);
            }
            
            if(this.isGameEnded(game)) {
                this.endGame(game);
            } else if (!freeTurn) {
                game.switchPlayerTurn();
            }
        }

        return game;
    }

    private isGameEnded(game: MancalaGame) {
        const player1Empty = game.getBoard()[Constants.PLAYERS.PLAYER1].slice(1, 7).every(pit => pit === 0);
        const player2Empty = game.getBoard()[Constants.PLAYERS.PLAYER2].slice(1, 7).every(pit => pit === 0);
        return player1Empty || player2Empty;
    }

    private endGame(game: MancalaGame) {
        game.incrementPit(Constants.PLAYERS.PLAYER1 as MancalaTurn, Constants.MANCALA.MANCALA_PIT as Pit, game.getBoard()[Constants.PLAYERS.PLAYER1].slice(1, 7).reduce((a, b) => a + b, 0));
        game.getBoard()[Constants.PLAYERS.PLAYER2][0] += game.getBoard()[Constants.PLAYERS.PLAYER2].slice(1, 7).reduce((a, b) => a + b, 0);

        for (let i = 1; i <= Constants.MANCALA.NUM_PITS; i++) {
            game.getBoard()[Constants.PLAYERS.PLAYER1][i] = 0;
            game.getBoard()[Constants.PLAYERS.PLAYER2][i] = 0;
        }
        game.resetCurrentTurn();

        return game;
    }

    private switchTurnNumber(turn: 0 | 1) {
        return Math.abs(turn - 1) as 0 | 1;
    }

    private isPitClickable(game: MancalaGame, pit: Pit) {
        return game.getCurrentBoardPit(pit) > 0;
    }
}