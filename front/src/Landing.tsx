import './App.css';


import { Route, Routes } from 'react-router-dom';

import Navbar from './Nav';

import Home from './pages/Home';
import styles from './App.module.css';

function land(){
    return (
        <>
          <Navbar />3
          <div className={styles.container}>
            <Routes>
              <Route path="/" element={<Home />} />
            </Routes>
          </div>
        </>
      );
}


export default land;