import React, { useState } from 'react';
import { AccordionSummary, AccordionDetails, Typography, Accordion, TextField, Button, Divider, AccordionActions, makeStyles, createStyles, Theme } from '@material-ui/core';
import ExpandMoreIcon from '@material-ui/icons/ExpandMore';
import LockIcon from '@material-ui/icons/Lock';
import CheckCircleIcon from '@material-ui/icons/CheckCircle';
import { IProfile } from '../models/Profile';

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    heading: {
      fontSize: theme.typography.pxToRem(15),
      flexBasis: '33.33%',
      flexShrink: 0,
    },
    secondaryHeading: {
      fontSize: theme.typography.pxToRem(15),
      color: theme.palette.text.secondary,
    },
    profileDetails: {
      flexDirection: "column",
    },
    divider: {
      marginLeft: 10,
      marginRight: 10,
    },
    deleteButton: {
      color: "#ff5555",
    },
    formField: {
      marginLeft: 5,
      marginRight: 5,
    },
    rectFields: {
      marginTop: 10,
      display: "flex",
      flexDirection: "row",
    },
  }),
);

export interface Props {
  profile: IProfile;
  onProfileChanged: (profile: IProfile) => void;

  isLocked: boolean;
  isActive: boolean;
}

export function ProfileSettings(props: Props) {
  const classes = useStyles();
  const stateVals = {
    regex: useState(props.profile.regex),
    x: useState(props.profile.area.x),
    y: useState(props.profile.area.y),
    width: useState(props.profile.area.width),
    height: useState(props.profile.area.height),
  };
  const [dirty, setDirty] = useState(false);

  const handleInput = (event: React.ChangeEvent<HTMLTextAreaElement | HTMLInputElement>) => {
    const target = event.target;
    const name = target.name;
    (stateVals as any)[name][1](target.value);
    setDirty(true);
  }

  return (
    <>
      <Accordion>
        <AccordionSummary
          expandIcon={<ExpandMoreIcon />}>
          <Typography className={classes.heading}>{stateVals.regex[0].replace(/[^a-zA-Z0-9_\s]/g, "") || "New Profile"}</Typography>
          <div className={classes.secondaryHeading}>
              <Typography>{ props.profile.area.width}x{props.profile.area.height }</Typography>
              { props.isLocked && <LockIcon /> }
              { (props.isActive && !props.isLocked) && <CheckCircleIcon /> }
            </div>
        </AccordionSummary>
        <AccordionDetails className={classes.profileDetails}>
          <TextField label="Window Title Regex" placeholder="^my\s+regex$" variant="outlined" className={classes.formField}
            value={stateVals.regex[0]} onChange={handleInput} name="regex"/>
          <div className={classes.rectFields} >
            <TextField label="X" placeholder="0" type="number" className={classes.formField}
              value={stateVals.x[0]} onChange={handleInput} name="x"/>
            <TextField label="Width" placeholder="1920" type="number" className={classes.formField}
              value={stateVals.width[0]} onChange={handleInput} name="width"/>
            <TextField label="Y" placeholder="0" type="number" className={classes.formField}
              value={stateVals.y[0]} onChange={handleInput} name="y"/>
            <TextField label="Height" placeholder="1080" type="number" className={classes.formField}
              value={stateVals.height[0]} onChange={handleInput} name="height"/>
          </div>
        </AccordionDetails>
          <Divider />
        <AccordionActions>
          <Button color="default" size="small">{props.isLocked ? "Unlock" : "Lock"}</Button>
          <Button color="default" size="small" className={classes.deleteButton}>Delete</Button>
          <Button color="primary" size="small" disabled={!dirty} onClick={() => {
              setDirty(false);
              props.onProfileChanged({
                regex: stateVals.regex[0],
                area: {
                  x: Number(stateVals.x[0]),
                  y: Number(stateVals.y[0]),
                  width: Number(stateVals.width[0]),
                  height: Number(stateVals.height[0]),
                },
              });
            }}>
            Save
          </Button>
        </AccordionActions>
      </Accordion>
    </>
  );
}