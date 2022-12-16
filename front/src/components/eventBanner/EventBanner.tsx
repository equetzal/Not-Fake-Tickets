import { FC } from 'react';
import styles from './EventBanner.module.css';

interface Props{
	eventName:string,
	location:string,
	dates:string,
}

const EventBanner: FC<Props> = ({eventName, location, dates}:Props) => {
  return( 
  	<div className={styles.eventBanner}>
		<img className={styles.eventBannerImage} src={require('./../../assets/img/placeholder.png')} />
		<div className={styles.eventBannerInnerBox}>
			<h1>{eventName}</h1>
			<p>{location}</p>
			<p>{dates}</p>
			<button>Buy Tickets</button>
		</div>
	</div>
  );
};

export default EventBanner;