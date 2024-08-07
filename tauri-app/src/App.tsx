import React, { useState,useEffect } from 'react';
import { render } from 'react-dom';

import { DevicesScene } from './devices/DevicesScene';
import { SystemScene } from './system/SystemScene';
import { AboutScene } from './AboutScene';
import appIcon from './assets/icon.png';

import 'normalize.css'
import './styles/Global.css';
import { ProfilesScene } from './profiles/ProfilesScene';
import { WebsocketService } from './WebsocketService';
import { InfoCircleTwoTone, FilterTwoTone, AlertTwoTone, SettingTwoTone } from "@ant-design/icons"
import { Layout, Menu, App as AntApp, ConfigProvider, theme, Divider, Alert } from "antd";
import { purple } from '@ant-design/colors';
import { ProfilesService } from './profiles/ProfilesService';
import { DevicesService } from './devices/DevicesService';
import { AudioDevicesService } from './system/AudioDevicesService';
const { Sider, Header, Content } = Layout;
const { useToken } = theme;

const mainElement = document.createElement('div');
mainElement.setAttribute('id', 'root');
document.body.appendChild(mainElement);

function App() {
  const [scene, setScene] = useState("devices");
  const [showBackendError, setShowBackendError] = useState(false);

  useEffect(() => {
    WebsocketService.Instance.connected.then(connected => {
      if (!connected) {
        setShowBackendError(true);
      }
    });
  }, []);
  useEffect(() => {
    // hacky way to force services to initialize on startup
    ProfilesService.Instance();
    DevicesService.Instance();
    AudioDevicesService.Instance();
  }, []);

  const colors = {
    primary: purple.primary,
    bgBase: "#ffffff00",
  };

  const { token: defaultToken } = useToken();
  return (
    <ConfigProvider theme={{
      algorithm: theme.darkAlgorithm,
      token: {
        colorPrimary: colors.primary,
        colorBgLayout: "#00000000",
        borderRadius: 4,
        colorLink: colors.primary
      },
      components: {
        Segmented: {
          colorBgLayout: "black",
        }
      }
    }}>
      <AntApp className='fill-vertical'>
        <Layout className='fill-vertical' style={{backgroundColor: "#000000df"}}>
          <Header style={{ background: colors.bgBase }}>
            <img src={appIcon} style={{ width: 64, height: 64 }}/>
            <span style={{fontSize: "24px", color: "white", position: "absolute" }}>lumos-rs</span>
          </Header>
          <Divider style={{ margin: 0, backgroundColor: defaultToken.colorBorder }}/>
          <Layout className='fill-vertical'>
            <Sider className='fill-vertical' style={{ background: colors.bgBase }}>
              <Menu style={{ background: "none", border: "none" }} mode="inline" selectedKeys={[scene]} items={[
                { label: "Devices",  key: "devices", icon: React.createElement(AlertTwoTone, { twoToneColor: "salmon" }) },
                { label: "Profiles", key: "profiles", icon: React.createElement(FilterTwoTone, { twoToneColor: "moccasin" }) },
                { label: "System",   key: "system", icon: React.createElement(SettingTwoTone, { twoToneColor: "darkseagreen" }) },
                { label: "About",    key: "about", icon: React.createElement(InfoCircleTwoTone, { twoToneColor: "#6666ff" }) },
              ]} onSelect={info => setScene(info.key)}
              />
            </Sider>
            <Content className="scene" style={{ backgroundColor: colors.bgBase }}>
              {showBackendError && <Alert type="error" showIcon style={{ marginBottom: 20 }}
                message="Unable to connect to backend. Try restarting the application." />}
              {scene === "devices"  && <DevicesScene/>}
              {scene === "profiles" && <ProfilesScene/>}
              {scene === "system"   && <SystemScene/>}
              {scene === "about"    && <AboutScene/>}
            </Content>
          </Layout>
        </Layout>
      </AntApp>
    </ConfigProvider>
  );
}

render(<App />, mainElement);
