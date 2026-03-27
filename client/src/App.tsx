import UserList from './modules/Users/UserList';
import React, { useState } from 'react';
import MancalaGame from './models/mancalaGame';
import './App.css';
import MancalaBoard from './modules/Mancala/MancalaBoard';
import MancalaTable from './modules/Mancala/MancalaTable';
import CJFBoard from './modules/CJF/CJFBoard';

function App() {
  return (
    <div>
      <MancalaTable></MancalaTable>
      {/* <CJFBoard></CJFBoard> */}
    </div>
  );
}

export default App;
