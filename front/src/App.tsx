import { FC, useState } from 'react';
import Landing from './pages/Landing';
import Login from './pages/Login';
import ManagerDashboard from './pages/ManagerDashboard';
import UserProfile from './pages/UserProfile';

export enum validApps {
  landing,
  loginManager,
  loginUser,
  managerDashboard,
  userProfile,
}

const getApp = (app:validApps, setCurrentApp:React.Dispatch<React.SetStateAction<validApps>>) => {
  switch(app){
    case validApps.landing:
      return (<Landing setCurrentApp={setCurrentApp}/>);
    case validApps.loginManager:
      return (<Login setCurrentApp={setCurrentApp} userType='manager'/>);
    case validApps.loginUser:
      return (<Login setCurrentApp={setCurrentApp} userType='user'/>);
    case validApps.userProfile:
      return (<UserProfile/>);
    case validApps.managerDashboard:
      return (<ManagerDashboard/>);
  }
}

const App: FC = () => {
  const [currentApp, setCurrentApp] = useState<validApps>(validApps.landing);

  return getApp(currentApp, setCurrentApp);
};

export default App;