import { FC } from 'react';
import styles from './Ticket.module.css';

const Ticket: FC = () => {
  return( 
    <div className={styles.ticket_container}>
    <img src={require('./../../assets/img/placeholder.png')} />
    <h1>Nombre evento</h1>
    <h3>Descripcion</h3>
    <p>Lugar</p>
    <p>Fecha</p>
    <h2>Costo(s)</h2>
    <p className={styles.costo_ticket}>$xxx - $xxx</p>
    </div>
  );
};

export default Ticket;