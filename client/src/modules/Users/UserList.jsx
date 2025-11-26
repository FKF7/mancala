import { useEffect, useState } from 'react';
import Routes from '../../../../common/routes';
import axios from 'axios';

function UserList() {
  const [users, setUsers] = useState([]);

  useEffect(() => {
    axios.get(`${Routes.BASE_URL}${Routes.API_URL.USERS}${Routes.ENDPOINTS.USERS.GET_BY_ID}`, {
      params: { id: 1 }
    })
      .then(res => setUsers(res.data))
      .catch(err => console.error(err));
  }, []);

  return (
    <div>
      <h2>Users</h2>
      <ul>
        {users.name}
      </ul>
    </div>
  );
}

export default UserList;
