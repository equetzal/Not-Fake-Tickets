import { FC, useState } from "react";
import styles from "./ManagerDashboard.module.css";

interface PropsSideBar {
  setContent: React.Dispatch<React.SetStateAction<AvailableContents>>;
}

const SideBar: FC<PropsSideBar> = ({ setContent }: PropsSideBar) => {
  return (
    <div className={styles.dashboardSidebar}>
      <div className={styles.sidebarTitle}>
        <h3>No Fake Tikets</h3>
      </div>
      <div className={styles.buttonsContainer}>
        <div
          className={styles.sideBarButton}
          onClick={() => setContent(AvailableContents.YourEvents)}
        >
          Tus Eventos
        </div>
        <div
          className={styles.sideBarButton}
          onClick={() => setContent(AvailableContents.CreateEvent)}
        >
          Create Event
        </div>
      </div>
      <div
        className={styles.exitButton}
        onClick={() => setContent(AvailableContents.CreateEvent)}
      >
        Salir
      </div>
    </div>
  );
};

interface PropsContent {
  contentToLoad: AvailableContents;
}

const Content: FC<PropsContent> = ({ contentToLoad }: PropsContent) => {
  return (
    <div
      className={styles.contentContainer}
    >{`Content ${AvailableContents[contentToLoad]}`}</div>
  );
};

enum AvailableContents {
  YourEvents,
  CreateEvent,
}

const ManagerDashboard: FC = () => {
  const [content, setContent] = useState<AvailableContents>(
    AvailableContents.YourEvents
  );

  return (
    <div className={styles.dashboardContainer}>
      <SideBar setContent={setContent} />
      <Content contentToLoad={content} />
    </div>
  );
};

export default ManagerDashboard;
