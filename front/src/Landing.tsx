import './App.css';


import { Route, Routes } from 'react-router-dom';

import Navbar from './Nav';

import Home from './pages/Home';
import styles from './App.module.css';
import Events from './pages/Events';
import Ticket from './ticket';

function land(){
    return (
        <>
          <Navbar />
          <div className={styles.container}>
            <Routes>
              <Route path="/" element={<Home />} />
              <Route path="/Events" element={<Events />} />
            </Routes>
          </div>
          <Ticket />
        </>
      );
}


export default land;