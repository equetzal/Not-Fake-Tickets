import '../App.css';


import { Route, Routes } from 'react-router-dom';
import { FC } from 'react';

import Navbar from '../components/nav/Nav';

import Home from './Home';
import styles from '../App.module.css';
import Events from './Events';
import Footer from '../components/footer/Footer';


const Landing: FC = () => {
    return (
        <>
          <Navbar />
          <div className={styles.container}>
            <Routes>
              <Route path="/" element={<Home />} />
              <Route path="/Events" element={<Events />} />
            </Routes>
          </div>
          <Footer/>
        </>
      );
}


export default Landing;