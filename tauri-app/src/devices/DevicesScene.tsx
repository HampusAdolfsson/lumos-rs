import { useState, useEffect } from 'react';
import { Paper, Button, TableContainer, Table, TableBody, TableRow, Divider, Toolbar } from '@mui/material';
import AddIcon from '@mui/icons-material/Add';
import { SamplingTypes } from './DeviceSpecification';
import { DevicesService, IExtendedDeviceSpecification } from './DevicesService';
import { DeviceEntry } from './DeviceEntry';

export function DevicesScene() {
  const [devices, setDevices] = useState([] as Array<IExtendedDeviceSpecification>);

  useEffect(() => {
    const subscription = DevicesService.Instance().then(service => service.devices.subscribe(profs => setDevices(profs)));
    return () => {
      subscription.then(sub => sub.unsubscribe());
    };
  });

  const deviceComponents = devices.map((dev, i) => {
    return (
        <TableRow key={i}>
          <DeviceEntry device={dev.device} enabled={dev.enabled}
            onDeviceDeleted={async() => {
              const newDevs: IExtendedDeviceSpecification[] = JSON.parse(JSON.stringify(devices));
              newDevs.splice(i, 1);
              (await DevicesService.Instance()).setDevices(newDevs, true);
              setDevices(newDevs);
            }}
            onDeviceEnabledChanged={async(enabled) => {
              const newDevs: IExtendedDeviceSpecification[] = JSON.parse(JSON.stringify(devices));
              newDevs[i].enabled = enabled;
              (await DevicesService.Instance()).setDevices(newDevs, true);
              setDevices(newDevs);
            }}
            onDeviceChanged={async dev => {
              const newDevs: IExtendedDeviceSpecification[] = JSON.parse(JSON.stringify(devices));
              newDevs[i].device = dev;
              (await DevicesService.Instance()).setDevices(newDevs, true);
              setDevices(newDevs);
            }}
          />
        </TableRow>);
  });
  return (
    <div id="devicesScene">
      <TableContainer component={Paper}>
        <Table>
          <TableBody>
            {deviceComponents}
          </TableBody>
        </Table>
        <Toolbar>
          <Button variant="outlined" color="primary" disableElevation sx={{ marginRight: 30 }} startIcon={<AddIcon/>}
          onClick={async() => {
          const newDevs = devices.concat([JSON.parse(JSON.stringify(defaultDevice))]);
          (await DevicesService.Instance()).setDevices(newDevs, true);
          setDevices(newDevs);
          }}>
          Add Device
        </Button>
        </Toolbar>
      </TableContainer>
    </div>
  )
}

const defaultDevice: IExtendedDeviceSpecification = {
  enabled: false,
  device: {
    name: '',
    numberOfLeds: 0,
    samplingType: SamplingTypes.Horizonal,
    gamma: 2,
    colorTemp: 5500,
    saturationAdjustment: 0,
    valueAdjustment: 0,
    audioAmount: 0,
    type: null,
    wledData: null,
    qmkData: null,
    serialData: null,
  }
};