import { FC } from 'react';
import styles from './Ticket.module.css';

interface Props{
	eventName:string,
	location:string,
	dates:string,
  costs: string,
  description: string,
}

const Ticket: FC<Props> = ({eventName, location, dates, costs, description}:Props) => {
  return( 
    <div className={styles.ticket_container}>
    <img src={require('./../../assets/img/placeholder.png')} alt='Ticket'/>
    <h1>{eventName}</h1>
    <h3>{description}</h3>
    <p>{location}</p>
    <p>{dates}</p>
    <h2>Costo(s)</h2>
    <p className={styles.costo_ticket}>{costs}</p>
    </div>
  );
};

export default Ticket;