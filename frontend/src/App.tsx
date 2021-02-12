import React, { useState, useEffect } from 'react';
import logo from './logo.svg';
import './App.css';
import axios from 'axios';
import { fetchUsers } from './api/user';
import { User } from './models/user';


const App: React.FC = () => {
  const [msg, setMsg] = useState("");
  const [userList, setUserList] = useState<User[] | undefined>(undefined);

  const fetchUserReq = async () => {
    try {
      const { data } = await fetchUsers();
      return data;
    } catch (e) {
      console.error(e);
    }
  }

  useEffect(() => {
    const fetch = async () => {
      const res = await axios.get('http://localhost/api').then((res) => res);
      setMsg(res['data']);
    }
    fetch();

    const data = fetchUserReq();
    data.then(users => {
      setUserList(users);
    });
  }, []);

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          {msg}
        </p>
        <table>
          <thead>
            <tr>
              <td>ID</td>
              <td>NAME</td>
              <td>EMAIL</td>
            </tr>
          </thead>
          <tbody>
            {
              userList && userList.map((user) => {
                return (
                  <tr key="{user.id}">
                    <td>{user.id}</td>
                    <td>{user.name}</td>
                    <td>{user.email}</td>
                  </tr>
                );
              })
            }
          </tbody>
          
        </table>
      </header>
    </div>
  );
}

export default App;
