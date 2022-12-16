import '../App.css';


import { Route, Routes } from 'react-router-dom';

import Navbar from '../components/nav/Nav';

import Home from './Home';
import styles from '../App.module.css';
import Events from './Events';
import Ticket from '../components/ticket/ticket';

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