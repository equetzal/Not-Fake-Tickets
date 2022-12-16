import { FC } from 'react';
import styles from './Footer.module.css';

interface Props{
}

const Footer: FC<Props> = () => {

  return( 
  	<div className={styles.footer}>
		<h2>No Fake Tickets LLC</h2>
		<p>Shalala All right reserved.</p>
		<a>Managing an event?, Sign In</a>
	</div>
  );
};

export default Footer;