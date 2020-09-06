import React, { useState, useEffect } from 'react';
import { Paper, Button, SvgIcon, makeStyles, Theme, createStyles, Grid } from '@material-ui/core';
import { DeviceSettings } from './DeviceSettings';
import AddIcon from '@material-ui/icons/Add';
import { IDeviceSpecification } from './DeviceSpecification';
import { DevicesService } from './DevicesService';

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
      width: "100%",
      display: "inline-block",
      padding: "20px 20px 0 20px",
      backgroundColor: "#3c3c3c",
    },
    deviceLeft: {
      marginRight: "6%",
    },
  }),
);

export function DevicesScene() {
  const [devices, setDevices] = useState([] as Array<IDeviceSpecification>);

  useEffect(() => {
    const subscription = DevicesService.Instance.devices.subscribe(profs => setDevices(profs));
    return () => {
      subscription.unsubscribe();
    };
  });

  const classes = useStyles();

  const deviceComponents = devices.map((dev, i) => {
    return <Grid item xs={6} key={dev.ipAddress} >
      <Paper elevation={1} className={classes.deviceEntryRoot}>
        <DeviceSettings device={dev}
          onDeviceDeleted={() => {
            const newDevs: IDeviceSpecification[] = JSON.parse(JSON.stringify(devices));
            newDevs.splice(i, 1);
            DevicesService.Instance.setDevices(newDevs);
            setDevices(newDevs);
          }}
          onDeviceChanged={dev => {
            const newDevs: IDeviceSpecification[] = JSON.parse(JSON.stringify(devices));
            newDevs[i] = dev;
            DevicesService.Instance.setDevices(newDevs);
            setDevices(newDevs);
          }}
          />
      </Paper>
    </Grid>
  });
  return (
    <div id="devicesScene">
      <div>
        <Button color="primary" variant="contained" disableElevation className={classes.button} startIcon={<AddIcon/>}
          onClick={() => {
              const newDevs = devices.concat([JSON.parse(JSON.stringify(defaultDevice))]);
              DevicesService.Instance.setDevices(newDevs);
              setDevices(newDevs);
            }}>
          Add
        </Button>
      </div>
      <Grid container spacing={5} className={classes.content}>
        {deviceComponents}
      </Grid>
    </div>
  )
}

const defaultDevice: IDeviceSpecification = {
  ipAddress: '',
  numberOfLeds: 0,
  blurRadius: 0,
  saturationAdjustment: 0,
  flipHorizontally: false,
};