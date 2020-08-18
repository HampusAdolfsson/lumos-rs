import React from 'react';
import { Paper, Button, SvgIcon, makeStyles, Theme, createStyles } from '@material-ui/core';
import { DeviceSettings } from './DeviceSettings';
import AddIcon from '@material-ui/icons/Add';

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    deleteButton: {
      color: "#ff5555",
    },
    button: {
      marginRight: 30,
    },
    content: {
      paddingTop: 30,
      paddingBottom: 30,
      maxWidth: 800,
    },
    deviceEntryRoot: {
      width: "47%",
      display: "inline-block",
      padding: "40px",
      backgroundColor: "#3c3c3c",
    },
    deviceLeft: {
      marginRight: "6%",
    },
  }),
);

export function DevicesScene() {
  const classes = useStyles();
  return (
    <div id="devicesScene">
      <div>
        <Button color="primary" variant="contained" disableElevation className={classes.button}>
          <AddIcon />
          Add
        </Button>
        <Button variant="outlined" disableElevation>Apply</Button>
      </div>
      <div className={classes.content}>
        <Paper elevation={1} className={classes.deviceEntryRoot + " " + classes.deviceLeft}>
          <DeviceSettings />
        </Paper>
        <Paper elevation={1} className={classes.deviceEntryRoot}>
          <DeviceSettings />
        </Paper>
      </div>
    </div>
  )
}