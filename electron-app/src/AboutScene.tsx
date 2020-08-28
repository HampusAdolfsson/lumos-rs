import React, { useEffect, useState } from 'react';
import { TextField, makeStyles, Theme, createStyles } from '@material-ui/core';
import { ipcRenderer, IpcRendererEvent } from 'electron';

const useStyles = makeStyles(() =>
  createStyles({
    aboutScene: {
      textAlign: 'center',
      paddingTop: '1em',
    },
    logView: {
      width: 600,
      "& .MuiInputBase-input": {
        fontSize: 12,
      }
    }
  }),
);

export function AboutScene() {
  const [logMsgs, setLogMsgs] = useState("");
  useEffect(() => {
    const logMessageHandler = (event: IpcRendererEvent, message: any) => {
      setLogMsgs(logMsgs + message);
    }
    if (!logMsgs) {
      ipcRenderer.once('logsReply', (event, reply) => {
        setLogMsgs(reply.toString());
      });
      ipcRenderer.send('logsRequest');
    }
    ipcRenderer.on('log', logMessageHandler);
    return () => { ipcRenderer.removeListener('log', logMessageHandler); }
  });

  const classes = useStyles();
  return (
    <div className={classes.aboutScene}>
      <p>Created by <strong>Hampus Adolfsson</strong>.</p>
      <p>Check out this project on <a href="https://github.com/HampusAdolfsson/win-rt-rgb">GitHub</a>!</p>
      <TextField className={classes.logView}
        value={logMsgs}
        variant="outlined"
        rows={14}
        disabled
        multiline />
    </div>
  );
}