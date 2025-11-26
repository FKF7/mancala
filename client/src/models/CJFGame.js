import Constants from '../../../common/constants';

class CJFGame {
    constructor(game) {
        if (game) {
            this.players = [
                {
                    pos: game.players[Constants.CJF.PLAYERS.PLAYER].pos,
                    lives: game.players[Constants.CJF.PLAYERS.PLAYER].lives,
                },
                {
                    pos: game.players[Constants.CJF.PLAYERS.SENSEI].pos,
                    lives: game.players[Constants.CJF.PLAYERS.SENSEI].lives,
                }
            ];
            this.turn = game.turn;
            if (game.numMove) {
                this.numMove = game.numMove;
            }
        } else {
            this.resetGame();
        }
    }

    resetGame() {
        this.players = [
            {
                pos: Constants.CJF.INITIAL_POSITION.PLAYER,
                lives: Constants.CJF.INITIAL_LIVES
            },
            {
                pos: Constants.CJF.INITIAL_POSITION.SENSEI,
                lives: Constants.CJF.INITIAL_LIVES
            }
        ];
        this.turn = Constants.CJF.PLAYERS.PLAYER;
        this.numMove = undefined;
    }

    getPlayerPos() {
        return this.players[Constants.CJF.PLAYERS.PLAYER].pos;
    }

    getPlayerLives() {
        return this.players[Constants.CJF.PLAYERS.PLAYER].lives;
    }
  
    getSenseiPos() {
        return this.players[Constants.CJF.PLAYERS.SENSEI].pos;
    }

    getSenseiLives() {
        return this.players[Constants.CJF.PLAYERS.SENSEI].lives;
    }

    getTurn() {
        return this.turn;
    }

    getTurnPos() {
        return this.players[this.turn].pos;
    }

    playersSharingPos() {
        return this.players[Constants.CJF.PLAYERS.PLAYER].pos === this.players[Constants.CJF.PLAYERS.SENSEI].pos;
    }

    getNumMove() {
        return this.numMove;
    }

    setNumMove(numMove) {
        if (!this.isGameEnded() && numMove >= Constants.CJF.MIN_MOVE && numMove <= Constants.CJF.MAX_MOVE) {
            this.numMove = numMove;
        }
    }

    makeMove(dir) {
        this.players[this.turn].pos = (dir ? this.players[this.turn].pos + this.numMove : this.players[this.turn].pos + Constants.CJF.NUM_TILES - this.numMove) % Constants.CJF.NUM_TILES;
        
        this.players[Constants.CJF.PLAYERS.SENSEI].lives--;
        if (Constants.CJF.DUEL_TILES.includes(this.getTurnPos()) || this.playersSharingPos()) {
            this.players[Constants.CJF.PLAYERS.PLAYER].lives++;
        }

        if(this.isGameEnded()) {
            this.endGame();
        } else {
            this.switchPlayerTurn();
            this.numMove = null;
        }
    }

    switchPlayerTurn() {
        this.turn = Math.abs(this.turn - 1);
    }

    isGameEnded() {
        return this.players[Constants.CJF.PLAYERS.PLAYER].lives <= 0 || this.players[Constants.CJF.PLAYERS.SENSEI].lives <= 0;
    }

    endGame() {
        this.numMove = undefined;
    }

    simulate() {
        // let player = this.currentTurn;
        // let pit = Math.floor(Math.random() * 6) + 1;
        // this.makeMove(player, pit);
    }
}
  
export default CJFGame;