import MancalaGame from './mancalaGame.model.ts';
import { MancalaTurn, Pit } from '../types.ts'

export default class History {
    private history: MancalaGame[];
    private index = 0;

    constructor(game: MancalaGame) {
        this.reset(game);
    }

    public add(game: MancalaGame) {
        this.history = this.history.slice(0, this.index + 1);
        this.history.push(game);
        this.index ++;
    }

    public fix() {
        this.history = this.history.slice(this.index);
        this.index = 0;
    }

    public back(): MancalaGame {
        if (this.index > 0) {
            this.index --;
        }
        return this.history[this.index]
    }

    public forward(): MancalaGame {
        if (this.index < this.history.length - 1) {
            this.index ++;
        }
        return this.history[this.index]
    }

    public reset(game: MancalaGame) {
        this.history = [game];
        this.index = 0;
    }
}