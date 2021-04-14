import { Button, createStyles, Dialog, DialogActions, DialogContent, DialogTitle, Icon, IconButton, makeStyles, Switch, TableCell, Theme, Typography } from '@material-ui/core';
import { Delete, PowerSettingsNew, Settings, WbIncandescent } from '@material-ui/icons';
import React, { useState } from 'react';
import { DeviceSettings } from './DeviceSettings';
import { IDeviceSpecification, DeviceTypes } from './DeviceSpecification';

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    deleteButton: {
      color: "#ff5555"
    },
  }),
);

interface Props {
  device: IDeviceSpecification;
  enabled: boolean;
  onDeviceDeleted: () => void;
  onDeviceChanged: (device: IDeviceSpecification) => void;
  onDeviceEnabledChanged: (enabled: boolean) => void;
}

export function DeviceEntry(props: Props) {
  const classes = useStyles();
  const [enabled, setEnabled] = useState(props.enabled);
  const [dialogOpen, setDialogOpen] = useState(false);

  return <React.Fragment>
            <TableCell component="th" scope="row" >
              <Switch checked={enabled} onChange={(_, v) => { setEnabled(v); props.onDeviceEnabledChanged(v); }}/>
              <Typography variant="subtitle1" display="inline">{props.device.name}</Typography>
            </TableCell>
            <TableCell align="right" >{props.device.numberOfLeds} LEDs</TableCell>
            <TableCell align="right" >
              {props.device.type == 0 ?
                "WLED - " + props.device.wledData?.ipAddress :
                "Qmk - " + props.device.qmkData?.name}
            </TableCell>
            <TableCell align="right" >
              {props.device.type == DeviceTypes.WLED && props.device.wledData?.ipAddress &&
                <IconButton color="primary" onClick={() => {
                  const xmlHttp = new XMLHttpRequest();
                  xmlHttp.open( "GET", `http://${props.device.wledData?.ipAddress}/win&T=2`, true);
                  xmlHttp.send( null );
                }}><PowerSettingsNew/></IconButton>}
              <IconButton onClick={() => {setDialogOpen(true);}}>
                <Settings/>
              </IconButton>
              <IconButton onClick={props.onDeviceDeleted}>
                <Delete className={classes.deleteButton}/>
              </IconButton>
            </TableCell>
            <DeviceSettings device={props.device} open={dialogOpen}
              onClosed={() => {setDialogOpen(false);}}
              onDeviceChanged={(device) => {setDialogOpen(false); props.onDeviceChanged(device);}}
            />
         </React.Fragment>
}