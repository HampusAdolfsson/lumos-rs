import React from 'react';
import { render } from 'react-dom';
import { GlobalStyle } from './styles/GlobalStyle';

import Button from '@material-ui/core/Button';
import { CssBaseline, makeStyles, createMuiTheme } from '@material-ui/core';
import { ThemeProvider } from '@material-ui/core';

const mainElement = document.createElement('div');
mainElement.setAttribute('id', 'root');
document.body.appendChild(mainElement);

let theme = createMuiTheme({
  palette: {
    type: 'dark',
    primary: {
      light: '#df78ef',
      main: '#ab47bc',
      dark: '#790e8b',
      contrastText: '#fff',
    },
    secondary: {
      light: '#439889',
      main: '#00695c',
      dark: '#003d33',
      contrastText: '#fff',
    },
  }
});

const App = () => {
  return (
    <>
      <ThemeProvider theme={theme} >
        <CssBaseline />
        <GlobalStyle />
        <Button variant="contained" color="primary">
          Button
        </Button>
      </ThemeProvider>
    </>
  );
}

render(<App />, mainElement);
