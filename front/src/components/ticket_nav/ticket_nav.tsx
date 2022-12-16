import { FC } from 'react';

import styles from './ticket_nav.module.css'
import Ticket from '../ticket/Ticket';
import { FaArrowAltCircleRight, FaArrowAltCircleLeft } from 'react-icons/fa';

interface Props{
}

interface EventData{
	eventName:string,
	location:string,
	dates:string,
    costs: string,
    description: string,
}

const Ticket_nav: FC<Props> = () => {

    const eventsData:EventData[] = [
		{eventName: '31 Minutos', location: 'Auditorio Nacional', dates: '15,16 Dic 2022',costs: '$xxx-xxx',description: 'aaa'},
		{eventName: 'ODESZA', location: 'Climate Pledge Arena', dates: '20 Dic 2022',costs: '$xxx-xxx',description: 'aaa'},
		{eventName: 'Louis the Child', location: 'Paradise Tulum', dates: '29 Dic 2022',costs: '$xxx-xxx',description: 'aaa'},
	]

    return (
        <div className={styles.nav}>
            <table>
                <tr>
                {eventsData.map(dataset => {
			        return(
                    <th>
                        <Ticket eventName={dataset.eventName} location={dataset.location} dates={dataset.dates} costs={dataset.costs} description={dataset.description}/>
                    </th>
                    );
		        })}
                </tr>
            </table>
        </div>
      );
}


export default Ticket_nav;