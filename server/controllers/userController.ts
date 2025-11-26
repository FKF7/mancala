import users from '../data/users.json' assert { type: 'json' };

const UserController = {
  getAllUsers: (req: any, res: any) => {
    res.json(users);
  },
  getUserById: (req: any, res: any) => {
    const user = users.find(u => u.id === parseInt(req.query.id));
    user ? res.json(user) : res.status(404).send("User not found");
  }
}

export default UserController;