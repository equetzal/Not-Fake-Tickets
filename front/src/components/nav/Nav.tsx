import { FC, ReactNode } from 'react';
import { Link, useMatch, useResolvedPath } from 'react-router-dom';
import { validApps } from '../../App';

import styles from './Navbar.module.css';

interface Props{
  setCurrentApp: React.Dispatch<React.SetStateAction<validApps>>,
}

const Navbar: FC<Props> = ({setCurrentApp}:Props) => {
  return (
    <nav className={styles.nav}>
      <Link to="/" className={styles.siteTitle}><h3>No Fake Tickets</h3>
      </Link>
      <ul>
        <CustomLink to="/Events">Events</CustomLink>
        <CustomLink to="/Collection">Collections</CustomLink>
        <CustomLink to="/About">About</CustomLink>
      </ul>
    </nav>
  );
};

const CustomLink: FC<{ to: string; children: ReactNode }> = ({
  to,
  children,
  ...props
}) => {
  const resolvedPath = useResolvedPath(to);
  const isActive = useMatch({ path: resolvedPath.pathname, end: true });

  return (
    <li className={isActive ? styles.active : ''}>
      <Link to={to} {...props}>
        {children}
      </Link>
    </li>
  );
};

export default Navbar;
