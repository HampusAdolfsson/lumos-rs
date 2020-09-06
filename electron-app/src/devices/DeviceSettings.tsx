import React, { useState } from 'react';
import { TextField, FormControlLabel, Checkbox, Typography, Slider, Button, Tooltip, Divider, Theme, makeStyles, createStyles } from '@material-ui/core';
import { IDeviceSpecification } from './DeviceSpecification';

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    root: {
      '& .MuiInputBase-root': {
        width: "100%",
        marginBottom: theme.spacing(1),
      },
    },
    formField: {
      display: "block",
      marginBottom: "5px",
      width: "100%",
    },
    buttons: {
      float: "right",
      margin: "5px 0",
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
  }),
);

interface Props {
  device: IDeviceSpecification;
  onDeviceDeleted: () => void;
  onDeviceChanged: (device: IDeviceSpecification) => void;
}

export function DeviceSettings(props: Props) {
  const [dirty, setDirty] = useState(false);

  const stateVals = {
    ipAddress: useState(props.device.ipAddress),
    numberOfLeds: useState(props.device.numberOfLeds),
    flipHorizontally: useState(props.device.flipHorizontally),
    saturationAdjustment: useState(props.device.saturationAdjustment),
    blurRadius: useState(props.device.blurRadius),
  };

  const handleInput = (event: React.ChangeEvent<HTMLTextAreaElement | HTMLInputElement>, value?: boolean | number) => {
    const target = event.target;
    const name = target.name;
    const val = target.type === 'checkbox' ? (value === true) : target.value;
    (stateVals as any)[name][1](val);
    setDirty(true);
  }

  const classes = useStyles();
  return (
    <div className={classes.root}>
      <TextField label="IP Address" placeholder="192.168.x.x" variant="outlined" className={classes.formField}
        name="ipAddress" value={stateVals.ipAddress[0]} onChange={handleInput}/>
      <TextField label="Number of LEDs" placeholder="1-490" color="primary" className={classes.formField} type="number"
        name="numberOfLeds" value={stateVals.numberOfLeds[0]} onChange={handleInput}/>
      <FormControlLabel className={classes.formField}
        control={<Checkbox color="primary" name="flipHorizontally" checked={stateVals.flipHorizontally[0]} onChange={handleInput}/>}
        label="Flip LED direction"
      />
      <Typography gutterBottom className={classes.formField}>
        Saturation
      </Typography>
      <Slider aria-labelledby="saturation-slider" color="primary" className={classes.formField} valueLabelDisplay="auto" min={-100} max={100}
        value={stateVals.saturationAdjustment[0]} onChange={(_, val) => { stateVals.saturationAdjustment[1](val as number); setDirty(true); }}/>
      <Typography gutterBottom className={classes.formField}>
        Blur Radius
      </Typography>
      <Slider aria-labelledby="blur-slider" color="primary" className={classes.formField} valueLabelDisplay="auto" marks={true} min={0} max={10}
        value={stateVals.blurRadius[0]} onChange={(_, val) => { stateVals.blurRadius[1](val as number); setDirty(true); }}/>
      <Divider />
      <div className={classes.buttons}>
        <Button size="small" color="secondary" onClick={() => {
          const xmlHttp = new XMLHttpRequest();
          xmlHttp.open( "GET", `http://${stateVals.ipAddress[0]}/win&T=2`, true);
          xmlHttp.send( null );
        }}>Toggle Power</Button>
        <Tooltip title="Delete the device">
          <Button className={classes.deleteButton} size="small" onClick={() => props.onDeviceDeleted()}>Delete</Button>
        </Tooltip>
        <Button color="primary" size="small" disabled={!dirty} onClick={() => {
            setDirty(false);
            props.onDeviceChanged({
              ipAddress: stateVals.ipAddress[0],
              numberOfLeds: Number(stateVals.numberOfLeds[0]),
              flipHorizontally: stateVals.flipHorizontally[0],
              saturationAdjustment: Number(stateVals.saturationAdjustment[0]),
              blurRadius: Number(stateVals.blurRadius[0]),
            });
          }}>
          Save
        </Button>
      </div>
    </div >
  );
}