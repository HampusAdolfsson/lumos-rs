import React from 'react';
import { render } from 'react-dom';

import '@fontsource/roboto/300.css';
import '@fontsource/roboto/400.css';
import '@fontsource/roboto/500.css';
import '@fontsource/roboto/700.css';
import { CssBaseline, AppBar, Alert, Tabs, Tab, Typography, ThemeProvider } from '@mui/material';
import { DevicesScene } from './devices/DevicesScene';
import { AboutScene } from './AboutScene';

import './styles/Global.css';
import { ProfilesScene } from './profiles/ProfilesScene';
import { WebsocketService } from './WebsocketService';
import { Cast, Crop, Info } from '@mui/icons-material';
import { createTheme } from '@mui/material/styles';
import { appWindow } from "@tauri-apps/api/window";
import { UnlistenFn } from "@tauri-apps/api/event";

const mainElement = document.createElement('div');
mainElement.setAttribute('id', 'root');
document.body.appendChild(mainElement);

let theme = createTheme({
  palette: {
    mode: 'dark',
    primary: {
      main: '#ffb300',
    },
    secondary: {
      main: '#ab47bc',
    },
  }
});

interface State {
  visibleScene: number;
  showBackendError: boolean;
  tabValue: number;
}

class App extends React.Component<{}, State> {

  constructor(props: {}) {
    super(props);
    this.state = {
      visibleScene: 0,
      showBackendError: false,
      tabValue: 0,
    };
    WebsocketService.Instance.connected.then(connected => {
      if (!connected) {
        this.setState({
          showBackendError: true,
        });
      }
    });
  }

  setScene(i: number) {
    this.setState({
      visibleScene: i,
    });
  }

  private unlisten: UnlistenFn | undefined = undefined;
  componentDidMount() {
    appWindow.onCloseRequested(async() => {
        WebsocketService.Instance.sendMessage("shutdown", {});
    }).then(ul => this.unlisten = ul);
  }
  componentWillUnmount(): void {
    this.unlisten?.();
  }

  render() {
    const value = this.state.tabValue;
    return (
      <>
        <ThemeProvider theme={theme} >
          <CssBaseline />
          <AppBar position="sticky" color="default">
            <Typography variant="h4" color="textSecondary" style={{position: "fixed", top: 15, left: 20 }}>lumos-rs</Typography>
            <Tabs value={value} onChange={(_, val) => { this.setState({tabValue: val}); }} centered
                  indicatorColor="primary"
                  textColor="primary" >
              <Tab icon={<Cast/>} label="Devices" />
              <Tab icon={<Crop/>} label="Profiles" />
              <Tab icon={<Info/>} label="About" />
            </Tabs>
          </AppBar>
          <div className="scene">
            {this.state.showBackendError && <Alert severity="error" variant="filled" style={{ marginBottom: 20 }}>
              Unable to connect to backend. Try restarting the application.</Alert>}
            {value == 0 && <DevicesScene/>}
            {value == 1 && <ProfilesScene/>}
            {value == 2 && <AboutScene/>}
          </div>
        </ThemeProvider>
      </>
    );
  }
}

render(<App />, mainElement);
