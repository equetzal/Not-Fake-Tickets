import '../App.css';


import { Route, Routes } from 'react-router-dom';
import { FC } from 'react';

import Navbar from '../components/nav/Nav';

import Home from './Home';
import styles from '../App.module.css';
import Events from './Events';
import Footer from '../components/footer/Footer';

import { validApps } from '../App';
import Ticket_nav from '../components/ticket_nav/ticket_nav';

interface Props{
  setCurrentApp: React.Dispatch<React.SetStateAction<validApps>>,
}

const Landing: FC<Props> = ({setCurrentApp}:Props) => {
    return (
        <>
          <Navbar setCurrentApp={setCurrentApp}/>
          <div className={styles.container}>
            <Routes>
              <Route path="/" element={<Home />} />
              <Route path="/Events" element={<Events />} />
            </Routes>
          </div>
          <Ticket_nav />
          <Footer setCurrentApp={setCurrentApp}/>
        </>
      );
}


export default Landing;