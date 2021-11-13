import React, { useState } from 'react';
import { WledSettings } from './WledSettings'
import { TextField, Typography, Slider, Button, Tooltip, Divider, Theme, makeStyles, createStyles, Card, CardActions, CardContent, FormControl, InputLabel, Select, MenuItem, FormControlLabel, Checkbox, Switch, Chip, Dialog, DialogActions, DialogTitle, DialogContent } from '@material-ui/core';
import { DeviceTypes, IDeviceSpecification } from './DeviceSpecification';
import { stat } from 'original-fs';
import { QmkSettings } from './QmkSettings';

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    root: {
      minWidth: "500px",
    },
    formField: {
      marginBottom: "20px",
    },
    buttons: {
      marginLeft: "auto !important",
      "& button": {
        marginLeft: 6,
      }
    },
    rightAligner: {
      width: "100%",
      justifyContent: "right",
    },
    divider: {
      marginTop: 5,
      marginBottom: 15,
    },
    horizontal: {
      display: "flex",
      '& > *': {
        flex: 1,
      },
      '& > *:first-child': {
        marginRight: 40,
        textAlign: "right"
      }
    },
    wide: {
      width: "100%"
    },
    typeChip: {
      marginRight: 8,
      marginBottom: 18
    },
  }),
);

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
    device.type === DeviceTypes.QMK ? <QmkSettings data={props.device.qmkData} changed={val => { setField("qmkData", val) }}/> : undefined
  );

  const classes = useStyles();
  return (
    <Dialog open={props.open} onClose={props.onClosed} scroll="paper">
      <DialogTitle>
        Device Settings
      </DialogTitle>
      <DialogContent>
        <div className={classes.horizontal}>
          <TextField label="Name" color="primary" className={classes.formField} variant="filled"
            name="name" value={device.name} onChange={handleInput} />
          <TextField label="Number of LEDs" placeholder="1-490" color="primary" className={classes.formField} type="number" variant="outlined"
            name="numberOfLeds" value={device.numberOfLeds} onChange={handleInput} />
        </div>
        <div className={classes.horizontal}>
          <TextField label="Preferred Monitor" type="number" className={classes.formField}
            name="preferredMonitor" value={device.preferredMonitor} onChange={handleInput} />
            <span>
              <Typography gutterBottom >
                Audio Amount (%)
              </Typography>
              <Slider color="secondary" valueLabelDisplay="auto" min={0} max={100} step={5}
              value={device.audioAmount} onChange={(_, val) => { setField("audioAmount", val as number); }}/>
            </span>
        </div>
        <div className={classes.horizontal}>
          <Typography gutterBottom >
            Color Temperature (K)
          </Typography>
          <Slider color="secondary" valueLabelDisplay="auto" min={2000} max={10000} step={100}
            value={device.colorTemp} onChange={(_, val) => { setField("colorTemp", val as number); }}/>
        </div>
        <div className={classes.horizontal}>
          <Typography gutterBottom >
            Gamma
          </Typography>
          <Slider color="secondary" valueLabelDisplay="auto" min={1.0} max={3.0} step={0.1}
            value={device.gamma} onChange={(_, val) => { setField("gamma", val as number); }} />
        </div>
        <div className={classes.horizontal}>
          <Typography gutterBottom >
            Saturation Increase (%)
          </Typography>
          <Slider color="secondary" valueLabelDisplay="auto" min={0} max={100} step={5}
            value={device.saturationAdjustment} onChange={(_, val) => { setField("saturationAdjustment", val as number); }} />
        </div>
        <div className={classes.horizontal}>
          <Typography gutterBottom >
            Value Increase (%)
          </Typography>
          <Slider color="secondary" valueLabelDisplay="auto" min={0} max={100} step={5}
            value={device.valueAdjustment} onChange={(_, val) => { setField("valueAdjustment", val as number); }} />
        </div>
        <Divider className={classes.divider} />
        <Chip label="WLED" color={device.type == DeviceTypes.WLED ? "secondary" : "default"}
          onClick={ () => { setField("type", DeviceTypes.WLED); } } className={classes.typeChip} />
        <Chip label="Qmk" color={device.type == DeviceTypes.QMK ? "secondary" : "default"}
          onClick={ () => { setField("type", DeviceTypes.QMK); } } className={classes.typeChip} />
        {deviceComponent}
      </DialogContent>
      <Divider />
      <DialogActions>
        <div className={classes.buttons}>
          <Button size="small" onClick={() => props.onClosed()}>Cancel</Button>
          <Button color="primary" size="small" disabled={!dirty} onClick={() => {
              setDirty(false);
              props.onDeviceChanged(device);
            }}>
            Save
          </Button>
        </div>
      </DialogActions>
    </Dialog>
  );
}