import { app, BrowserWindow, Tray, Menu } from 'electron';
import * as path from 'path';
import * as url from 'url';
import installExtension, { REACT_DEVELOPER_TOOLS, REDUX_DEVTOOLS } from 'electron-devtools-installer';
import { ChildProcess, spawn } from 'child_process';

let mainWindow: Electron.BrowserWindow | null;
let isQuitting = false;
let backendProcess: ChildProcess | undefined;

app.setName('win-rt-rgb');

function createWindow() {
  mainWindow = new BrowserWindow({
    title: 'win-rt-rgb',
    width: 1200,
    height: 700,
    backgroundColor: '#202020',
    webPreferences: {
      nodeIntegration: true
    }
  });

  if (process.env.NODE_ENV === 'development') {
    mainWindow.loadURL('http://localhost:4000');
  } else {
    mainWindow.loadURL(
      url.format({
        pathname: path.join(__dirname, 'renderer/index.html'),
        protocol: 'file:',
        slashes: true
      })
    );
  }

  mainWindow.on('closed', () => {
    mainWindow = null;
  });
  mainWindow.on('close', (event: Event) => {
    if (!isQuitting) {
      event.preventDefault();
      mainWindow?.hide();
    }
  });
  mainWindow.on('minimize', (event: Event) => {
    event.preventDefault();
    mainWindow?.hide();
  });
}

app.whenReady().then(() => {
  backendProcess = spawn(path.join(__dirname, 'assets/backend/win-rt-rgb.exe'));
  createWindow();
  const tray = new Tray(path.join(__dirname, 'assets/icon.png'));
  tray.setToolTip('win-rt-rgb -- Realtime RGB suite');
  const contextMenu = Menu.buildFromTemplate([
    { label: 'Open', click: () => {
      if (!mainWindow) {
        createWindow();
      }
      mainWindow?.show();
    }},
    { label: 'Exit', click: () => {
      isQuitting = true;
      app.quit();
    }},
  ]);
  tray.setContextMenu(contextMenu);
  tray.on('double-click', () => {
    mainWindow?.show();
  });

  if (process.env.NODE_ENV === 'development') {
    installExtension(REACT_DEVELOPER_TOOLS)
      .then((name) => console.log(`Added Extension:  ${name}`))
      .catch((err) => console.log('An error occurred: ', err));
    installExtension(REDUX_DEVTOOLS)
      .then((name) => console.log(`Added Extension:  ${name}`))
      .catch((err) => console.log('An error occurred: ', err));
  }
});

app.on('window-all-closed', () => {
  // On OS X it is common for applications and their menu bar
  // to stay active until the user quits explicitly with Cmd + Q
  if (process.platform !== 'darwin') {
    app.quit();
  }
});
app.on('before-quit', () => {
  backendProcess?.kill();
}),
app.allowRendererProcessReuse = true;
