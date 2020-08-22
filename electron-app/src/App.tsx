import React from 'react';
import { render } from 'react-dom';

import { CssBaseline, makeStyles, createMuiTheme } from '@material-ui/core';
import { ThemeProvider } from '@material-ui/core';
import { Sidebar } from './Sidebar';
import { DevicesScene } from './devices/DevicesScene';
import { AboutScene } from './AboutScene';

import './styles/Global.css';
import { ProfilesScene } from './profiles/ProfilesScene';

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
}

class App extends React.Component<{}, State> {

  constructor(props: {}) {
    super(props);
    this.state = {
      visibleScene: 0,
    };
  }

  setScene(i: number) {
    this.setState({
      visibleScene: i,
    });
  }

  render() {
    const sceneNames = scenes.map(scene => scene[1]);
    return (
      <>
        <ThemeProvider theme={theme} >
          <CssBaseline />
          <Sidebar scenes={sceneNames} selectedScene={this.state.visibleScene} onSceneChanged={this.setScene.bind(this)} />
          <div className="scene">
            {scenes[this.state.visibleScene][0]}
          </div>
        </ThemeProvider>
      </>
    );
  }
}

render(<App />, mainElement);
