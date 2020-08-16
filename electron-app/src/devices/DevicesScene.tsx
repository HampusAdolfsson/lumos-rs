import React from 'react';
import { Paper, Button, SvgIcon } from '@material-ui/core';
import { DeviceSettings } from './DeviceSettings';
import AddIcon from '@material-ui/icons/Add';
import './Style.css';

export class DevicesScene extends React.Component {
  render() {
    return (
      <div id="devicesScene">
        <div id="deviceControls">
          <Button color="primary" variant="contained" disableElevation>
            <AddIcon />
            Add
            </Button>
          <Button variant="outlined" disableElevation>Apply</Button>
        </div>
        <div id="deviceList">
          <Paper elevation={1} className="deviceEntryRoot deviceLeft">
            <DeviceSettings />
          </Paper>
          <Paper elevation={1} className="deviceEntryRoot">
            <DeviceSettings />
          </Paper>
        </div>
      </div>
    )
  }
}