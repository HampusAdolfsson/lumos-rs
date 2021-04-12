import React, { useState } from 'react';
import { WledSettings } from './WledSettings'
import { TextField, Typography, Slider, Button, Tooltip, Divider, Theme, makeStyles, createStyles, Card, CardActions, CardContent, FormControl, InputLabel, Select, MenuItem, FormControlLabel, Checkbox, Switch, Chip } from '@material-ui/core';
import { IDeviceSpecification } from './DeviceSpecification';
import { stat } from 'original-fs';
import { QmkSettings } from './QmkSettings';

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    formField: {
      marginBottom: "20px",
    },
    buttons: {
      marginLeft: "auto !important",
      "& button": {
        marginLeft: 6,
      }
    },
    deleteButton: {
      color: "#ff5555",
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
  onDeviceDeleted: () => void;
  onDeviceEnabledChanged: (enabled: boolean) => void;
  onDeviceChanged: (device: IDeviceSpecification) => void;
}

enum DeviceTypes {
  WLED,
  QMK,
}

export function DeviceSettings(props: Props) {
  const [dirty, setDirty] = useState(false);
  const [enabled, setEnabled] = useState(true);
  const [device, setDevice] = useState(props.device);

  const setField = (name: string, val: any) => {
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
    setField(name, val);
  }

  const deviceComponent = device.type === DeviceTypes.WLED ?
    <WledSettings enabled={enabled} data={props.device.wledData} changed={val => { setField("wledData", val); }} /> : (
    device.type === DeviceTypes.QMK ? <QmkSettings enabled={enabled} data={props.device.qmkData} changed={val => { setField("qmkData", val) }}/> : undefined
  );

  const classes = useStyles();
  return (
    <Card elevation={1}>
      <CardContent>
        <div className={classes.horizontal}>
          <TextField label="Name" color="primary" className={classes.formField} variant="filled"
            name="name" value={device.name} onChange={handleInput} disabled={!enabled} />
          <TextField label="Number of LEDs" placeholder="1-490" color="primary" className={classes.formField} type="number" variant="outlined"
            name="numberOfLeds" value={device.numberOfLeds} onChange={handleInput} disabled={!enabled} />
        </div>
        <div className={classes.horizontal}>
          <TextField label="Preferred Monitor" color="primary" type="number" className={classes.formField}
            name="preferredMonitor" value={device.preferredMonitor} onChange={handleInput} disabled={!enabled} />
          <FormControlLabel className={classes.formField} disabled={!enabled}
            control={<Checkbox color="primary" name="useAudio" checked={device.useAudio} onChange={handleInput}/>}
            label="Use Audio"
          />
        </div>
        <div className={classes.horizontal}>
          <Typography gutterBottom >
            Color Temperature (K)
          </Typography>
          <Slider color="primary" valueLabelDisplay="auto" min={2000} max={10000} disabled={!enabled} step={100}
            value={device.colorTemp} onChange={(_, val) => { setField("colorTemp", val as number); }}/>
        </div>
        <div className={classes.horizontal}>
          <Typography gutterBottom >
            Gamma
          </Typography>
          <Slider color="primary" valueLabelDisplay="auto" min={1.0} max={3.0} step={0.1}
            value={device.gamma} onChange={(_, val) => { setField("gamma", val as number); }} disabled={!enabled}/>
        </div>
        <div className={classes.horizontal}>
          <Typography gutterBottom >
            Saturation Increase (%)
          </Typography>
          <Slider color="primary" valueLabelDisplay="auto" min={0} max={100} step={5}
            value={device.saturationAdjustment} onChange={(_, val) => { setField("saturationAdjustment", val as number); }} disabled={!enabled}/>
        </div>
        <div className={classes.horizontal}>
          <Typography gutterBottom >
            Value Increase (%)
          </Typography>
          <Slider color="primary" valueLabelDisplay="auto" min={0} max={100} step={5}
            value={device.valueAdjustment} onChange={(_, val) => { setField("valueAdjustment", val as number); }} disabled={!enabled}/>
        </div>
        <Divider className={classes.divider} />
        <Chip label="WLED" color={device.type == DeviceTypes.WLED ? "secondary" : "default"} disabled={!enabled}
          onClick={ () => { setField("type", DeviceTypes.WLED); } } className={classes.typeChip} />
        <Chip label="Qmk" color={device.type == DeviceTypes.QMK ? "secondary" : "default"} disabled={!enabled}
          onClick={ () => { setField("type", DeviceTypes.QMK); } } className={classes.typeChip} />
        {deviceComponent}
      </CardContent>
      <Divider />
      <CardActions>
        <Switch name="enabled" checked={enabled} onChange={(_, v) => setEnabled(v)}/>
        <div className={classes.buttons}>
          <Tooltip title="Delete the device">
            <Button className={classes.deleteButton} size="small" onClick={() => props.onDeviceDeleted()}>Delete</Button>
          </Tooltip>
          <Button color="primary" size="small" disabled={!dirty || !enabled} onClick={() => {
              setDirty(false);
              props.onDeviceChanged(device);
            }}>
            Save
          </Button>
        </div>
      </CardActions>
    </Card>
  );
}