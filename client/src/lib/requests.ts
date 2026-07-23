import MancalaGame from "../models/mancalaGame.model";
import { Hint, MancalaTurn, Pit, SimulationResult } from "../types/types";
import { executeGETRequest } from "../utils/requestExecutor";
import Routes from "../utils/routes";

export async function getHintData(game: MancalaGame): Promise<Hint[]> {
  return await executeGETRequest(
    `${Routes.BASE_URL}${Routes.API_URL}${Routes.ENDPOINTS.GET_HINT_DATA}`,
    { game: JSON.stringify(game) },
  );
}

export async function simulate(
  game: MancalaGame,
  ends: number,
): Promise<SimulationResult> {
  return await executeGETRequest(
    `${Routes.BASE_URL}${Routes.API_URL}${Routes.ENDPOINTS.SIMULATE_GAME}`,
    { game: JSON.stringify(game), ends },
  );
}

export async function makeMove(
  game: MancalaGame,
  pit: Pit,
): Promise<MancalaGame> {
  let newGame = await executeGETRequest(
    `${Routes.BASE_URL}${Routes.API_URL}${Routes.ENDPOINTS.MAKE_MOVE}`,
    { pit, game: JSON.stringify(game) },
  );
  return new MancalaGame(newGame.board, newGame.currentTurn as MancalaTurn);
}

export function printPath(game: MancalaGame): void {
  executeGETRequest(
    `${Routes.BASE_URL}${Routes.API_URL}${Routes.ENDPOINTS.PRINT_PATH}`,
    { game: JSON.stringify(game) },
  );
}
