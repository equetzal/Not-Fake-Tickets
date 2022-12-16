import { FC } from "react";
import { validApps } from "../App";
import "../App.css";

interface Props {
  userType: "manager" | "user";
  setCurrentApp: React.Dispatch<React.SetStateAction<validApps>>;
}

const Login: FC<Props> = ({ userType, setCurrentApp }: Props) => {
  return (
    <div className="login">
      Here you are supposed to do something with your wallet
      <button
        onClick={() => {
          setCurrentApp(
            userType == "manager"
              ? validApps.managerDashboard
              : validApps.userProfile
          );
        }}
      >
        Log In
      </button>
    </div>
  );
};

export default Login;
