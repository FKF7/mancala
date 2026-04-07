import MancalaGame from "../models/mancalaGame.model";
import { Hint, MancalaTurn, Pit } from "../types/types.ts";
import { executeGETRequest } from "../utils/requestExecutor";
import Routes from "../utils/routes.ts";


export async function getHintData(game: MancalaGame): Promise<Hint[]> {
  return await executeGETRequest(`${Routes.BASE_URL}${Routes.API_URL.MANCALA}${Routes.ENDPOINTS.MANCALA.GET_HINT_DATA}`,
    { game: JSON.stringify(game) });
}

export async function simulate(game: MancalaGame): Promise<Hint[]> {
  return await executeGETRequest(`${Routes.BASE_URL}${Routes.API_URL.MANCALA}${Routes.ENDPOINTS.MANCALA.SIMULATE_GAME}`,
    { game: JSON.stringify(game) });
}

export async function makeMove(game: MancalaGame, pit: Pit): Promise<MancalaGame> {
  let newGame =  await executeGETRequest(`${Routes.BASE_URL}${Routes.API_URL.MANCALA}${Routes.ENDPOINTS.MANCALA.MAKE_MOVE}`,
      { pit, game: JSON.stringify(game) });
  return new MancalaGame(newGame.board, newGame.currentTurn as MancalaTurn);
}