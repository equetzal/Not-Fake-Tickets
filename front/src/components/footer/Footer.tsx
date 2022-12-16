import { FC } from 'react';
import { validApps } from '../../App';
import styles from './Footer.module.css';

interface Props{
	setCurrentApp: React.Dispatch<React.SetStateAction<validApps>>,
  }

const Footer: FC<Props> = ({setCurrentApp}:Props) => {

  return( 
  	<div className={styles.footer}>
		<h2>No Fake Tickets LLC</h2>
		<p>Shalala All right reserved.</p>
		<a onClick={() => {setCurrentApp(validApps.loginManager)}}>Managing an event?, Sign In</a>
	</div>
  );
};

export default Footer;