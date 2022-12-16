import { FC, useState } from "react";
import styles from "./ManagerDashboard.module.css";

interface PropsSideBar {
  setContent: React.Dispatch<React.SetStateAction<AvailableContents>>;
}

const SideBar: FC<PropsSideBar> = ({ setContent }: PropsSideBar) => {
  return (
    <div className={styles.dashboardSidebar}>
      SideBar
      <ul>
        <li onClick={() => setContent(AvailableContents.ejemplo)}>ejemplo</li>
        <li onClick={() => setContent(AvailableContents.popo)}>popo</li>
      </ul>
    </div>
  );
};

interface PropsContent {
  contentToLoad: AvailableContents;
}

const Content: FC<PropsContent> = ({ contentToLoad }: PropsContent) => {
  return <div>{`Content ${AvailableContents[contentToLoad]}`}</div>;
};

enum AvailableContents {
  ejemplo,
  popo,
}

const ManagerDashboard: FC = () => {
  const [content, setContent] = useState<AvailableContents>(
    AvailableContents.ejemplo
  );

  return (
    <div className={styles.dashboardContainer}>
      <SideBar setContent={setContent} />
      <Content contentToLoad={content} />
    </div>
  );
};

export default ManagerDashboard;
