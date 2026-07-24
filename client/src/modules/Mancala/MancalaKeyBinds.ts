import { Pit } from "../../types/types";

type MancalaActions = {
  undo: () => void;
  redo: () => void;
  reset: () => void;
  movePit: (pit: Pit) => Promise<void>;
};

export default class MancalaKeyBinds {
  private actions: MancalaActions = {
    undo: () => {},
    redo: () => {},
    reset: () => {},
    movePit: async () => {},
  };

  private isListening = false;

  private readonly keyUpHandler = (event: KeyboardEvent): void => {
    this.onKeyPress(event);
  };

  public setAction<K extends keyof MancalaActions>(
    actionKey: K,
    action: MancalaActions[K],
  ): void {
    this.actions[actionKey] = action;
  }

  public start(): void {
    if (this.isListening) {
      return;
    }

    window.addEventListener("keyup", this.keyUpHandler);
    this.isListening = true;
  }

  public stop(): void {
    if (!this.isListening) {
      return;
    }

    window.removeEventListener("keyup", this.keyUpHandler);
    this.isListening = false;
  }

  private onKeyPress(event: KeyboardEvent): void {
    const target = event.target;

    if (
      target instanceof HTMLInputElement ||
      target instanceof HTMLTextAreaElement ||
      target instanceof HTMLSelectElement
    ) {
      return;
    }

    switch (event.key.toLowerCase()) {
      case "z":
      case "arrowleft":
      case "backspace":
        this.actions.undo();
        break;

      case "y":
      case "arrowright":
        this.actions.redo();
        break;

      case "r":
        this.actions.reset();
        break;

      default:
        if (/^[1-6]$/.test(event.key)) {
          void this.actions.movePit(Number(event.key) as Pit);
        }
    }
  }
}
