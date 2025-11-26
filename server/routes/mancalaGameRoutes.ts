import Routes from '../../common/routes.ts';
import Express from 'express';
import MancalaGameController from '../controllers/mancalaGame.controller.ts';

const MancalaRoutes = Express.Router();
const MancalaControler = new MancalaGameController();

MancalaRoutes.get(`${Routes.ENDPOINTS.MANCALA.MAKE_MOVE}`, MancalaControler.handleMoveRequest);
MancalaRoutes.get(`${Routes.ENDPOINTS.MANCALA.SIMULATE_GAME}`, MancalaControler.simulateGame);

export default MancalaRoutes;