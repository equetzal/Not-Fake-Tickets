import { FC } from 'react';
import styles from './ticket.module.css';

const Ticket: FC = () => {
  return( 
  <div className='ticket'>
    <h1>Nombre evento</h1>
    <h3>Descripcion</h3>
    <p>Lugar</p>
    <p>Fecha</p>
    <h2>Costo</h2>
    </div>
  );
};

export default Ticket;