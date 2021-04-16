import { app, BrowserWindow, Tray, Menu, ipcMain } from 'electron';
import * as path from 'path';
import * as url from 'url';
import installExtension, { REACT_DEVELOPER_TOOLS, REDUX_DEVTOOLS } from 'electron-devtools-installer';
import { ChildProcess, spawn } from 'child_process';

let mainWindow: Electron.BrowserWindow | null;
let isQuitting = false;

let backendProcess: ChildProcess | undefined;
let backendProcessOutput: string = "";

app.setName('win-rt-rgb');

function createWindow() {
  mainWindow = new BrowserWindow({
    title: 'win-rt-rgb',
    width: 1000,
    height: 1000,
    backgroundColor: '#202020',
    webPreferences: {
      nodeIntegration: true
    },
    icon: path.join(__dirname, 'assets/icon.png'),
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

}

app.whenReady().then(() => {
  if (process.env.NODE_ENV !== 'development') {
    backendProcess = spawn(path.join(__dirname, 'assets/backend/win-rt-rgb.exe'));
  }
  backendProcess?.on('exit', code => {
    backendProcessOutput += `Backend process exited with code ${code}.`;
    mainWindow?.webContents.send('log', `Backend process exited with code ${code}.`)
  });
  backendProcess?.stdout?.on('data', data => {
    backendProcessOutput += data.toString();
    mainWindow?.webContents.send('log', data.toString());
  });
  ipcMain.on('logsRequest', (event) => {
    event.reply('logsReply', backendProcessOutput);
  });

  createWindow();

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
