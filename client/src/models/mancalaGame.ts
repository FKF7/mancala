import Constants from '../../../common/constants.js';

class MancalaGame {
    private board: number[][];
    constructor(game) {
        if (game) {
            this.board = game.board;
            this.currentTurn = game.currentTurn;
        } else {
            this.resetGame();
        }
    }

    resetGame() {
        this.board = [
            [0, 4, 4, 4, 4, 4, 4], // Player 1's pits
            [0, 4, 4, 4, 4, 4, 4], // Player 2's pits
        ];
        this.currentTurn = Constants.PLAYERS.PLAYER1; // Player 1 starts
    }
  
    getBoard() {
        return this.board;
    }

    getCurrentTurn() {
        return this.currentTurn;
    }

    makeMove(player, pit) {
        const freeTurn = this.board[player][pit] % 13 === pit;
        let pebbles = this.board[player][pit];
        this.board[player][pit] = 0;

        while (pebbles > 0) {
            if (pit > 0) {
                if (pit === 1 && player !== this.currentTurn) {
                    pit = 6;
                    player = Math.abs(player - 1);
                } else {
                    pit--;
                }
            } else {
                pit = 6;
                player = Math.abs(player - 1);
            }
            this.board[player][pit]++;
            pebbles--;
        }

        if (this.board[player][pit] === 1 && player === this.currentTurn && pit !== 0 && this.board[Math.abs(player - 1)][7 - pit] !== 0) {
            this.board[this.currentTurn][0] += this.board[Math.abs(player - 1)][7 - pit] + 1;
            this.board[Math.abs(player - 1)][7 - pit] = 0;
            this.board[player][pit] = 0;
        }

        if(this.isGameEnded()) {
            this.endGame();
        } else {
            if (!freeTurn) {
                this.switchPlayerTurn();
            }
        }
    }

    switchPlayerTurn() {
        this.currentTurn = Math.abs(this.currentTurn - 1);
    }

    isGameEnded() {
        const player1Empty = this.board[Constants.PLAYERS.PLAYER1].slice(1, 7).every(pit => pit === 0);
        const player2Empty = this.board[Constants.PLAYERS.PLAYER2].slice(1, 7).every(pit => pit === 0);
        return player1Empty || player2Empty;
    }

    endGame() {
        this.board[Constants.PLAYERS.PLAYER1][0] += this.board[Constants.PLAYERS.PLAYER1].slice(1, 7).reduce((a, b) => a + b, 0);
        this.board[Constants.PLAYERS.PLAYER2][0] += this.board[Constants.PLAYERS.PLAYER2].slice(1, 7).reduce((a, b) => a + b, 0);

        for (let i = 1; i < 7; i++) {
            this.board[Constants.PLAYERS.PLAYER1][i] = 0;
            this.board[Constants.PLAYERS.PLAYER2][i] = 0;
        }
        this.currentTurn = null;
    }

    simulate() {
        // let player = this.currentTurn;
        // let pit = Math.floor(Math.random() * 6) + 1;
        // this.makeMove(player, pit);
    }
}
  
export default MancalaGame;