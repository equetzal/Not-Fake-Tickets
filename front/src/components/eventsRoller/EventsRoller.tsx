import { FC } from 'react';
import EventBanner from '../eventBanner/EventBanner';
import styles from './EventsRoller.module.css';

interface Props{
}

interface EventData{
	eventName:string,
	location:string,
	dates:string,
}

const EventsRoller: FC<Props> = () => {

	const eventsData:EventData[] = [
		{eventName: '31 Minutos', location: 'Auditorio Nacional', dates: '15,16 Dic 2022'},
		{eventName: 'ODESZA', location: 'Climate Pledge Arena', dates: '20 Dic 2022'},
		{eventName: 'Louis the Child', location: 'Paradise Tulum', dates: '29 Dic 2022'},
	]
	

  return( 
  	<div className={styles.eventRoller}>
		{eventsData.map(dataset => {
			return(<EventBanner eventName={dataset.eventName} location={dataset.location} dates={dataset.dates}/>);
		})}
		
	</div>
  );
};

export default EventsRoller;