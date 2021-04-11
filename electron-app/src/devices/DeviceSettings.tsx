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

  const stateVals = {
    name: useState(props.device.name),
    numberOfLeds: useState(props.device.numberOfLeds),
    gamma: useState(props.device.gamma),
    colorTemp: useState(props.device.colorTemp),
    saturationAdjustment: useState(props.device.saturationAdjustment),
    valueAdjustment: useState(props.device.valueAdjustment),
    useAudio: useState(props.device.useAudio),
    preferredMonitor: useState(props.device.preferredMonitor),
    type: useState(props.device.type),
    wledData: useState(props.device.wledData),
    qmkData: useState(props.device.qmkData),
  };

  const handleInput = (event: React.ChangeEvent<HTMLTextAreaElement | HTMLInputElement>, value?: boolean | number) => {
    const target = event.target;
    const name = target.name;
    const val = target.type === 'checkbox' ? (value === true) : target.value;
    (stateVals as any)[name][1](val);
    setDirty(true);
  }

  const deviceComponent = stateVals.type[0] === DeviceTypes.WLED ?
    <WledSettings enabled={enabled} data={props.device.wledData} changed={val => {stateVals.wledData[1](val); setDirty(true); }} /> : (
    stateVals.type[0] === DeviceTypes.QMK ? <QmkSettings enabled={enabled} data={props.device.qmkData} changed={val => {stateVals.qmkData[1](val); setDirty(true); }}/> : undefined
  );

  const classes = useStyles();
  return (
    <Card elevation={1}>
      <CardContent>
        <div className={classes.horizontal}>
          <TextField label="Name" color="primary" className={classes.formField} variant="filled"
            name="name" value={stateVals.name[0]} onChange={handleInput} disabled={!enabled} />
          <TextField label="Number of LEDs" placeholder="1-490" color="primary" className={classes.formField} type="number" variant="outlined"
            name="numberOfLeds" value={stateVals.numberOfLeds[0]} onChange={handleInput} disabled={!enabled} />
        </div>
        <div className={classes.horizontal}>
          <TextField label="Preferred Monitor" color="primary" type="number" className={classes.formField}
            name="preferredMonitor" value={stateVals.preferredMonitor[0]} onChange={handleInput} disabled={!enabled} />
          <FormControlLabel className={classes.formField} disabled={!enabled}
            control={<Checkbox color="primary" name="useAudio" checked={stateVals.useAudio[0]} onChange={handleInput}/>}
            label="Use Audio"
          />
        </div>
        <div className={classes.horizontal}>
          <Typography gutterBottom >
            Color Temperature (K)
          </Typography>
          <Slider color="primary" valueLabelDisplay="auto" min={2000} max={10000} disabled={!enabled} step={100}
            value={stateVals.colorTemp[0]} onChange={(_, val) => { stateVals.colorTemp[1](val as number); setDirty(true); }}/>
        </div>
        <div className={classes.horizontal}>
          <Typography gutterBottom >
            Gamma
          </Typography>
          <Slider color="primary" valueLabelDisplay="auto" min={1.0} max={3.0} step={0.1}
            value={stateVals.gamma[0]} onChange={(_, val) => { stateVals.gamma[1](val as number); setDirty(true); }} disabled={!enabled}/>
        </div>
        <div className={classes.horizontal}>
          <Typography gutterBottom >
            Saturation Increase (%)
          </Typography>
          <Slider color="primary" valueLabelDisplay="auto" min={0} max={100} disabled={!enabled} step={5}
            value={stateVals.saturationAdjustment[0]} onChange={(_, val) => { stateVals.saturationAdjustment[1](val as number); setDirty(true); }}/>
        </div>
        <div className={classes.horizontal}>
          <Typography gutterBottom >
            Value Increase (%)
          </Typography>
          <Slider color="primary" valueLabelDisplay="auto" min={0} max={100} step={5}
            value={stateVals.valueAdjustment[0]} onChange={(_, val) => { stateVals.valueAdjustment[1](val as number); setDirty(true); }} disabled={!enabled}/>
        </div>
        <Divider className={classes.divider} />
        <Chip label="WLED" color={stateVals.type[0] == DeviceTypes.WLED ? "secondary" : "default"} disabled={!enabled}
          onClick={ () => { stateVals.type[1](DeviceTypes.WLED); setDirty(true); } } className={classes.typeChip} />
        <Chip label="Qmk" color={stateVals.type[0] == DeviceTypes.QMK ? "secondary" : "default"} disabled={!enabled}
          onClick={ () => { stateVals.type[1](DeviceTypes.QMK); setDirty(true); } } className={classes.typeChip} />
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
              props.onDeviceChanged({
                name: stateVals.name[0],
                gamma: stateVals.gamma[0],
                colorTemp: stateVals.colorTemp[0],
                numberOfLeds: Number(stateVals.numberOfLeds[0]),
                saturationAdjustment: Number(stateVals.saturationAdjustment[0]),
                valueAdjustment: Number(stateVals.valueAdjustment[0]),
                useAudio: stateVals.useAudio[0],
                preferredMonitor: Number(stateVals.preferredMonitor[0]),
                type: Number(stateVals.type[0]),
                wledData: stateVals.wledData[0],
                qmkData: stateVals.qmkData[0],
              });
            }}>
            Save
          </Button>
        </div>
      </CardActions>
    </Card>
  );
}