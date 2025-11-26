import Routes from '../../common/routes.ts';
import Express from 'express';
import UserController from '../controllers/userController.ts';

const UserRoutes = Express.Router();

UserRoutes.get(`${Routes.ENDPOINTS.USERS.GET_ALL}`, UserController.getAllUsers);
UserRoutes.get(`${Routes.ENDPOINTS.USERS.GET_BY_ID}`, UserController.getUserById);

export default UserRoutes;