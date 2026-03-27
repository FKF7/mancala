import Constants from '../constants.ts';
import { MancalaTurn, Pit } from '../types.ts'

export default class MancalaGame {
    private board: number[][];
    private currentTurn: MancalaTurn;

    constructor(board?: number[][], currentTurn?: MancalaTurn) {
        if (board !== undefined && currentTurn !== undefined) {
            this.board = [
                [...board[0]], // Player 1's pits
                [...board[1]], // Player 2's pits
            ];
            this.currentTurn = currentTurn;
        } else {
            // this.resetBoard();
            this.board = [
                [0, 4, 4, 4, 4, 4, 4], // Player 1's pits
                [0, 4, 4, 4, 4, 4, 4], // Player 2's pits
            ];
            this.currentTurn = Constants.PLAYERS.PLAYER1 as MancalaTurn; // Player 1 starts
        }
    }

    resetBoard() {
        this.board = [
            [0, 4, 4, 4, 4, 4, 4], // Player 1's pits
            [0, 4, 4, 4, 4, 4, 4], // Player 2's pits
        ];
        this.currentTurn = Constants.PLAYERS.PLAYER1 as MancalaTurn; // Player 1 starts
    }

    getBoard() {
        return this.board;
    }

    public getCurrentTurn() {
        return this.currentTurn;
    }

    public resetCurrentTurn() {
        this.currentTurn = null;
    }

    public switchPlayerTurn() {
        if (this.currentTurn !== null) {
            this.currentTurn = Math.abs(this.currentTurn - 1) as MancalaTurn;
        }
    }

    public getCurrentBoardPit(pit: number) {
        if (this.currentTurn !== null) {
            return this.board[this.currentTurn][pit];
        } else {
            return -1;
        }
    }

    public incrementCurrentBoardPit(pit: number, amount: number = 1) {
        if (this.currentTurn !== null) {
            this.board[this.currentTurn][pit] += amount;
        }
    }

    public resetCurrentBoardPit(pit: number) {
        if (this.currentTurn !== null) {    
            this.board[this.currentTurn][pit] = 0;
        }
    }

    public getPit(boardSide: MancalaTurn, pit: number) {
        if (boardSide !== null) {
            return this.board[boardSide][pit];
        } else {
            return -1;
        }
    }

    public incrementPit(boardSide: MancalaTurn, pit: Pit, amount: number = 1) {
        if (boardSide !== null) {
            this.board[boardSide][pit] += amount;
        }
    }

    public getOppositePit(boardSide: MancalaTurn, pit: Pit) {
        if (boardSide !== null) {
            return this.board[Math.abs(boardSide - 1)][7 - pit];
        } else {
            return -1;
        }
    }

    public setPit(boardSide : MancalaTurn, pit: Pit, value: number) {
        if (boardSide !== null) {
            this.board[boardSide][pit] = value;
        }
    }

    public resetCapturePits(boardSide: MancalaTurn, pit: number) {
        if (boardSide !== null) {
            this.board[Math.abs(boardSide - 1)][7 - pit] = 0;
            this.board[boardSide][pit] = 0;
        }
    }

    public getFrozenPebbles(): number {
        return this.board[0][0] + this.board[1][0];
    }


}