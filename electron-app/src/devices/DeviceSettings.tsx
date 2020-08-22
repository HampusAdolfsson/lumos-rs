import React from 'react';
import { TextField, FormControlLabel, Checkbox, Typography, Slider, Button, Tooltip, Divider, Theme, makeStyles, createStyles } from '@material-ui/core';

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
    deleteButton: {
      color: "#ff5555",
      margin: "5px 0",
      float: "right",
    },
    rightAligner: {
      width: "100%",
      justifyContent: "right",
    },
  }),
);

export function DeviceSettings() {
  const classes = useStyles();
  return (
    <div className={classes.root}>
      <TextField label="IP Address" placeholder="192.168.x.x" variant="outlined" className={classes.formField}/>
      <TextField label="Number of LEDs" placeholder="1-490" color="primary" className={classes.formField} type="number"/>
      <FormControlLabel className={classes.formField}
        control={<Checkbox color="primary"/>}
        label="Flip LED direction"
      />
      <Typography gutterBottom className={classes.formField}>
        Saturation
      </Typography>
      <Slider aria-labelledby="saturation-slider" color="primary" className={classes.formField} valueLabelDisplay="auto" min={-100} max={100}/>
      <Typography gutterBottom className={classes.formField}>
        Blur Radius
      </Typography>
      <Slider aria-labelledby="blur-slider" color="primary" className={classes.formField} valueLabelDisplay="auto" marks={true} min={0} max={10} />
      <Divider />
        <Tooltip title="Delete the device">
          <Button className={classes.deleteButton} size="small">Delete</Button>
        </Tooltip>
    </div >
  );
}