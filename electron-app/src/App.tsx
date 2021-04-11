import React from 'react';
import { render } from 'react-dom';

import 'fontsource-roboto';
import { CssBaseline, createMuiTheme } from '@material-ui/core';
import { ThemeProvider } from '@material-ui/core';
import { Sidebar } from './Sidebar';
import { DevicesScene } from './devices/DevicesScene';
import { AboutScene } from './AboutScene';

import './styles/Global.css';
import { ProfilesScene } from './profiles/ProfilesScene';
import Alert from '@material-ui/lab/Alert';
import { WebsocketService } from './WebsocketService';
import { ProfilesService } from './profiles/ProfilesService';

const mainElement = document.createElement('div');
mainElement.setAttribute('id', 'root');
document.body.appendChild(mainElement);

let theme = createMuiTheme({
  palette: {
    type: 'dark',
    primary: {
      light: '#ffe54c',
      main: '#ffb300',
      dark: '#c68400',
      contrastText: '#000',
    },
    secondary: {
      light: '#df78ef',
      main: '#ab47bc',
      dark: '#790e8b',
      contrastText: '#fff',
    },
  }
});

const scenes: [JSX.Element, string][] = [
  [<DevicesScene />, "Devices"],
  [<ProfilesScene />, "Profiles"],
  [<AboutScene />, "About"],
];

interface State {
  visibleScene: number;
  showBackendError: boolean;
}

class App extends React.Component<{}, State> {

  constructor(props: {}) {
    super(props);
    this.state = {
      visibleScene: 0,
      showBackendError: false,
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

  componentDidMount() {
    ProfilesService.LoadAndInstantiate();
  }

  render() {
    const sceneNames = scenes.map(scene => scene[1]);
    return (
      <>
        <ThemeProvider theme={theme} >
          <CssBaseline />
          <Sidebar scenes={sceneNames} selectedScene={this.state.visibleScene} onSceneChanged={this.setScene.bind(this)} />
          <div className="scene">
          { this.state.showBackendError &&
            <Alert severity="error" variant="filled" style={{marginBottom: 30}}>Unable to connect to backend. Try restarting the program.</Alert> }
            {scenes[this.state.visibleScene][0]}
          </div>
        </ThemeProvider>
      </>
    );
  }
}

render(<App />, mainElement);
