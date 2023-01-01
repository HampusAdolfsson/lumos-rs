import React, { useState } from 'react';
import { WledSettings } from './WledSettings'
import { TextField, Typography, Slider, Button, Divider, Theme, makeStyles, createStyles, Chip, Dialog, DialogActions, DialogTitle, DialogContent } from '@mui/material';
import { DeviceTypes, IDeviceSpecification, SamplingTypes } from './DeviceSpecification';
import { QmkSettings } from './QmkSettings';
import { SerialSettings } from './SerialSettings';
import { styled } from '@mui/material/styles';
import { TextFieldProps } from '@mui/material/TextField';
import Box, { BoxProps } from '@mui/material/Box';
import { ChipProps } from '@mui/material/Chip';

const DeviceTypeChip = styled(Chip)<ChipProps>(() => ({
  marginRight: "8px",
  marginBottom: "18px",
}));

const FormField = styled(TextField)<TextFieldProps>(() => ({
  marginBottom: "20px",
}));

const HorizontalBox = styled(Box)<BoxProps>(() => ({
  display: "flex",
  '& > *': {
    flex: 1,
  },
  '& > *:first-child': {
    marginRight: 40,
    textAlign: "right"
  }
}));

interface Props {
  device: IDeviceSpecification;
  open: boolean;
  onClosed: () => void;
  onDeviceChanged: (device: IDeviceSpecification) => void;
}

export function DeviceSettings(props: Props) {
  const [dirty, setDirty] = useState(false);
  const [device, setDevice] = useState(props.device);

  const setField = (name: keyof IDeviceSpecification, val: any) => {
    let newDevice = JSON.parse(JSON.stringify(device));
    newDevice[name] = val;
    setDevice(newDevice);
    setDirty(true);
  }

  const handleInput = (event: React.ChangeEvent<HTMLTextAreaElement | HTMLInputElement>, value?: boolean | number) => {
    const target = event.target;
    const name = target.name;
    let val: string | number | boolean = target.type === 'checkbox' ? (value === true) : target.value;
    if (target.type == 'number') val = Number(val);
    setField(name as keyof IDeviceSpecification, val);
  }

  const deviceComponent = device.type === DeviceTypes.WLED ?
    <WledSettings data={props.device.wledData} changed={val => { setField("wledData", val); }} /> : (
      device.type === DeviceTypes.QMK ? <QmkSettings data={props.device.qmkData} changed={val => { setField("qmkData", val) }}/> : (
      device.type === DeviceTypes.Serial ? <SerialSettings data={props.device.serialData} changed={val => { setField("serialData", val) }}/> : undefined
    ));

  return (
    <Dialog open={props.open} onClose={props.onClosed} scroll="paper">
      <DialogTitle>
        Device Settings
      </DialogTitle>
      <DialogContent>
        <HorizontalBox>
          <FormField label="Name" color="primary" variant="filled"
            name="name" value={device.name} onChange={handleInput} />
          <FormField label="Number of LEDs" placeholder="1-490" color="primary" type="number" variant="outlined"
            name="numberOfLeds" value={device.numberOfLeds} onChange={handleInput} />
        </HorizontalBox>
        <div>
          <DeviceTypeChip label="Horizontal" color={device.samplingType == SamplingTypes.Horizonal ? "secondary" : "default"}
            onClick={ () => { setField("samplingType", SamplingTypes.Horizonal); } } />
          <DeviceTypeChip label="Vertical" color={device.samplingType == SamplingTypes.Vertical ? "secondary" : "default"}
            onClick={ () => { setField("samplingType", SamplingTypes.Vertical); } } />
        </div>
        <HorizontalBox>
              <Typography gutterBottom >
                Audio Amount (%)
              </Typography>
              <Slider color="secondary" valueLabelDisplay="auto" min={0} max={100} step={5}
              value={device.audioAmount} onChange={(_, val) => { setField("audioAmount", val as number); }}/>
        </HorizontalBox>
        <HorizontalBox>
          <Typography gutterBottom >
            Color Temperature (K)
          </Typography>
          <Slider color="secondary" valueLabelDisplay="auto" min={2000} max={10000} step={100}
            value={device.colorTemp} onChange={(_, val) => { setField("colorTemp", val as number); }}/>
        </HorizontalBox>
        <HorizontalBox>
          <Typography gutterBottom >
            Gamma
          </Typography>
          <Slider color="secondary" valueLabelDisplay="auto" min={1.0} max={3.0} step={0.1}
            value={device.gamma} onChange={(_, val) => { setField("gamma", val as number); }} />
        </HorizontalBox>
        <HorizontalBox>
          <Typography gutterBottom >
            Saturation Increase (%)
          </Typography>
          <Slider color="secondary" valueLabelDisplay="auto" min={0} max={100} step={5}
            value={device.saturationAdjustment} onChange={(_, val) => { setField("saturationAdjustment", val as number); }} />
        </HorizontalBox>
        <HorizontalBox>
          <Typography gutterBottom >
            Value Increase (%)
          </Typography>
          <Slider color="secondary" valueLabelDisplay="auto" min={0} max={100} step={5}
            value={device.valueAdjustment} onChange={(_, val) => { setField("valueAdjustment", val as number); }} />
        </HorizontalBox>
        <Divider sx={{ marginTop: "5px", marginBottom: "15px" }}/>
        <DeviceTypeChip label="WLED" color={device.type == DeviceTypes.WLED ? "secondary" : "default"}
          onClick={ () => { setField("type", DeviceTypes.WLED); } } />
        <DeviceTypeChip label="Qmk" color={device.type == DeviceTypes.QMK ? "secondary" : "default"}
          onClick={ () => { setField("type", DeviceTypes.QMK); } } />
        <DeviceTypeChip label="Serial (Adalight)" color={device.type == DeviceTypes.Serial ? "secondary" : "default"}
          onClick={ () => { setField("type", DeviceTypes.Serial); } } />
        {deviceComponent}
      </DialogContent>
      <Divider />
      <DialogActions>
        <Box sx={{
          marginLeft: "auto !important",
          "& button": {
            marginLeft: 6,
          }
        }}>
          <Button size="small" onClick={() => props.onClosed()}>Cancel</Button>
          <Button color="primary" size="small" disabled={!dirty} onClick={() => {
              setDirty(false);
              props.onDeviceChanged(device);
            }}>
            Save
          </Button>
        </Box>
      </DialogActions>
    </Dialog>
  );
}