import React, { useState, useEffect } from 'react';
import { Paper, Button, SvgIcon, makeStyles, Theme, createStyles, Grid, Collapse, TableContainer, Table, TableBody, TableRow, TableCell, TableHead, Divider, Toolbar } from '@material-ui/core';
import { DeviceSettings } from './DeviceSettings';
import AddIcon from '@material-ui/icons/Add';
import { IDeviceSpecification } from './DeviceSpecification';
import { DevicesService, IExtendedDeviceSpecification } from './DevicesService';
import { Delete, Settings } from '@material-ui/icons';
import { DeviceEntry } from './DeviceEntry';

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    deleteButton: {
      color: "#ff5555",
    },
    button: {
      marginRight: 30,
    },
    content: {
    },
    deviceEntryRoot: {
      width: "100%",
      // display: "inline-block",
      // backgroundColor: "#3c3c3c",
      // marginBottom: "15px",
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
    return (
        <TableRow key={i}>
          <DeviceEntry device={dev.device} enabled={dev.enabled}
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
        </TableRow>);
  });
  return (
    <div id="devicesScene">
      <TableContainer component={Paper} className={classes.content}>
        <Toolbar>
        <Button variant="outlined" color="primary" disableElevation className={classes.button} startIcon={<AddIcon/>}
          onClick={() => {
              const newDevs = devices.concat([JSON.parse(JSON.stringify(defaultDevice))]);
              DevicesService.Instance.setDevices(newDevs);
              setDevices(newDevs);
            }}>
          Add
        </Button>
        </Toolbar>
        <Divider/>
        <Table>
          <TableBody>
            {deviceComponents}
          </TableBody>
        </Table>
      </TableContainer>
    </div>
  )
}

const defaultDevice: IExtendedDeviceSpecification = {
  enabled: false,
  device: {
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
  }
};