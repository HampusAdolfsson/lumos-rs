import React, { useState, useEffect } from 'react';
import { Paper, Button, SvgIcon, makeStyles, Theme, createStyles, Grid, Collapse } from '@material-ui/core';
import { DeviceSettings } from './DeviceSettings';
import AddIcon from '@material-ui/icons/Add';
import { IDeviceSpecification } from './DeviceSpecification';
import { DevicesService, IExtendedDeviceSpecification } from './DevicesService';

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
      backgroundColor: "#3c3c3c",
      marginBottom: "15px",
    },
  }),
);

export function DevicesScene() {
  const [devices, setDevices] = useState([] as Array<IExtendedDeviceSpecification>);

  useEffect(() => {
    const subscription = DevicesService.Instance.devices.subscribe(profs => setDevices(profs));
    return () => {
      subscription.unsubscribe();
    };
  });

  const classes = useStyles();

  const deviceComponents = devices.map((dev, i) => {
    return <div className={classes.deviceEntryRoot}>
        <DeviceSettings device={dev.device} enabled={dev.enabled}
          onDeviceDeleted={() => {
            const newDevs: IExtendedDeviceSpecification[] = JSON.parse(JSON.stringify(devices));
            newDevs.splice(i, 1);
            DevicesService.Instance.setDevices(newDevs);
            setDevices(newDevs);
          }}
          onDeviceEnabledChanged={(enabled) => {
            const newDevs: IExtendedDeviceSpecification[] = JSON.parse(JSON.stringify(devices));
            newDevs[i].enabled = enabled;
            DevicesService.Instance.setDevices(newDevs);
            setDevices(newDevs);
          }}
          onDeviceChanged={dev => {
            const newDevs: IExtendedDeviceSpecification[] = JSON.parse(JSON.stringify(devices));
            newDevs[i].device = dev;
            DevicesService.Instance.setDevices(newDevs);
            setDevices(newDevs);
          }}
          />
      </div>
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
      <div className={classes.content}>
        {deviceComponents}
      </div>
    </div>
  )
}

const defaultDevice: IDeviceSpecification = {
  name: '',
  numberOfLeds: 0,
  gamma: 2,
  colorTemp: 5500,
  saturationAdjustment: 0,
  valueAdjustment: 0,
  useAudio: false,
  preferredMonitor: 0,
  type: null,
  wledData: null,
  qmkData: null,
};