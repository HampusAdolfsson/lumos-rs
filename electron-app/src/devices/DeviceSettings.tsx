import React from 'react';
import { TextField, FormControlLabel, Checkbox, Typography, Slider, Button, Tooltip } from '@material-ui/core';
import DeleteIcon from '@material-ui/icons/Delete';

export class DeviceSettings extends React.Component {
  render() {
    return (
      <div className="deviceSettings">
        <TextField id="outlined-basic" label="IP Address" placeholder="192.168.x.x" variant="outlined" className="formField"/>
        <TextField id="outlined-basic" label="Number of LEDs" placeholder="1-490" className="formField"/>
        <FormControlLabel className="formField"
          control={<Checkbox color="primary"/>}
          label="Flip LED direction"
        />
        <Typography id="saturation-slider" gutterBottom className="formField">
          Saturation
        </Typography>
        <Slider aria-labelledby="saturation-slider" className="formField" valueLabelDisplay="auto" min={-100} max={100}/>
        <Typography id="blur-slider" gutterBottom className="formField">
          Blur Radius
        </Typography>
        <Slider aria-labelledby="blur-slider" className="formField" valueLabelDisplay="auto" marks={true} min={0} max={10} />
        <Tooltip title="Delete the device">
          <Button color="secondary" variant="outlined" id="remove" > <DeleteIcon /></Button>
        </Tooltip>
      </div >
    );
  }
}