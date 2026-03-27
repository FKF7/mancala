import { Pit } from "../../../../common/types";

type MancalaActions = {
    undo: () => void;
    redo: () => void;
    reset: () => void;
    movePit: (pit: Pit) => Promise<void>;
};

export default class MancalaKeyBinds {
    private actions: MancalaActions;

    constructor() {
        this.actions = {
            undo: () => {},
            redo: () => {},
            reset: () => {},
            movePit: async (pit: number) => {}
        };
        window.addEventListener("keyup", this.onKeyPress.bind(this));
    }

    public setAction<K extends keyof MancalaActions>(
        actionKey: K,
        action: MancalaActions[K]
    ) {
        this.actions[actionKey] = action;
    }

    public onKeyPress(
        e: KeyboardEvent | React.KeyboardEvent,
    ) {
        // debugger;
        console.log(e.key);
        console.log(this.actions)
        return;

        switch (e.key) {
            case "z": case "Z":
                this.actions.undo();
                break;

            case "y": case "Y":
                this.actions.redo();
                break;

            case "r": case "R":
                this.actions.reset();
                break;

            default:
                if (e.key >= "1" && e.key <= "6") {
                    console.log(`Moving pit ${e.key}`)
                    this.actions.movePit(Number(e.key) as Pit);
                }
        }
    }
}