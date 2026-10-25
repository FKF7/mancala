import MancalaGame, { MancalaGameDto } from "../models/mancalaGame.model";
import { HintData, Pit, SimulationResult } from "../types/types";
import { executeRequest } from "../utils/requestExecutor";
import Routes from "../utils/routes";

export async function getHintData(game: MancalaGame): Promise<HintData> {
  return await executeRequest<HintData>(
    `${Routes.BASE_URL}${Routes.API_URL}${Routes.ENDPOINTS.GET_HINT_DATA}`,
    {
      method: "POST",
      body: JSON.stringify({
        game,
      }),
    },
  );
}

export async function simulate(
  game: MancalaGame,
  ends: number,
  prune: boolean = false,
): Promise<SimulationResult> {
  return await executeRequest<SimulationResult>(
    `${Routes.BASE_URL}${Routes.API_URL}${Routes.ENDPOINTS.SIMULATE_GAME}`,
    {
      method: "POST",
      body: JSON.stringify({
        game,
        ends,
        prune,
      }),
    },
  );
}

export async function makeMove(
  game: MancalaGame,
  pit: Pit,
): Promise<MancalaGame> {
  let newGame = await executeRequest<MancalaGameDto>(
    `${Routes.BASE_URL}${Routes.API_URL}${Routes.ENDPOINTS.MAKE_MOVE}`,
    {
      method: "POST",
      body: JSON.stringify({
        pit,
        game,
      }),
    },
  );

  return new MancalaGame(newGame.board, newGame.currentTurn);
}

export function printPath(game: MancalaGame): void {
  executeRequest(
    `${Routes.BASE_URL}${Routes.API_URL}${Routes.ENDPOINTS.PRINT_PATH}`,
    {
      method: "POST",
      body: JSON.stringify({
        game,
      }),
    },
  );
}
