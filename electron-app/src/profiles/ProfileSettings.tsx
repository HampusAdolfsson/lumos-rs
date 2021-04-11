import React, { useState } from 'react';
import { AccordionSummary, AccordionDetails, Typography, Accordion, TextField, Button, Divider, AccordionActions, makeStyles, createStyles, Theme } from '@material-ui/core';
import ExpandMoreIcon from '@material-ui/icons/ExpandMore';
import LockIcon from '@material-ui/icons/Lock';
import CheckCircleIcon from '@material-ui/icons/CheckCircle';
import ReactRegionSelect from 'react-region-select';
import { IProfile } from './Profile';

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
      flexDirection: 'column',
    },
    divider: {
      marginLeft: 10,
      marginRight: 10,
    },
    deleteButton: {
      color: '#ff5555',
    },
    formField: {
      marginLeft: 5,
      marginRight: 5,
    },
    rectFields: {
      marginTop: 10,
      marginBottom: 5,
      display: 'flex',
      width: '100%',
      '& div': {
        // width: 80,
      }
    },
    icon: {
      height: 16,
      color: '#6f6',
    },
    dragIcon: {
      height: 16,
      color: '#aaa',
    },
    active: {
      background: '#525252'
    },
    areaSelectRoot: {
      marginTop: 20,
      marginLeft: 10,
      marginRight: 10,
    },
    areaSelector: {
      background: '#333',
      width: `${1920 / 4}px`,
      height: `${1080 / 4}px`,
      marginLeft: 'auto',
      marginRight: 'auto',
    },
    tooltip: {
      position: 'absolute',
      right: 0, bottom: 0,
      background: '#66666666',
      fontSize: '10px',
      padding: '4px',
      color: '#ffe54c'
    }
  }),
);

export interface Props {
  profile: IProfile;
  onProfileChanged: (profile: IProfile) => void;
  onProfileDeleted: () => void;
  onProfileLocked: () => void;

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

  const relToScreenCoords = (reg: any) => {
    return {
      x: Math.round(reg.x / 100.0 * 1920.0),
      y: Math.round(reg.y / 100.0 * 1080.0),
      width: Math.round(reg.width / 100.0 * 1920.0),
      height: Math.round(reg.height / 100.0 * 1080.0),
    };
  };

  const screenToRelCoords = (reg: any) => {
    return {
      x: reg.x * 100.0 / 1920.0,
      y: reg.y * 100.0 / 1080.0,
      width: reg.width * 100.0 / 1920.0,
      height: reg.height * 100.0 / 1080.0,
    };
  };

  const [region, setRegion] = useState(Object.assign(screenToRelCoords(props.profile.area), { data: {} }));

  const onRegionChange = (reg: any) => {
    setRegion(reg);
    const sc = relToScreenCoords(reg);
    if (sc.x != stateVals.x[0]) { stateVals.x[1](sc.x); }
    if (sc.width != stateVals.width[0]) { stateVals.width[1](sc.width); }
    if (sc.y != stateVals.y[0]) { stateVals.y[1](sc.y); }
    if (sc.height != stateVals.height[0]) { stateVals.height[1](sc.height); }
    setDirty(true);
  }


  const regionRenderer = (regionProps: any) => {
    const sc = relToScreenCoords(region);
    return (
      <div className={classes.tooltip}>
        {`x:${sc.x} w:${sc.width} y:${sc.y} h:${sc.height}`}
      </div>
    );
	};


  return (
    <>
      <Accordion className={(props.isActive ? classes.active : "")}>
        <AccordionSummary
          expandIcon={<ExpandMoreIcon />}>
          <Typography className={classes.heading }>
            {stateVals.regex[0].replace(/[^a-zA-Z0-9_\s]/g, "") || "New Profile"}
                { props.isLocked && <LockIcon className={classes.icon} /> }
                { (props.isActive && !props.isLocked) && <CheckCircleIcon className={classes.icon} /> }
          </Typography>
          <div className={classes.secondaryHeading}>
              <Typography>
                {stateVals.width[0]}x{stateVals.height[0]}
              </Typography>
            </div>
        </AccordionSummary>
        <AccordionDetails className={classes.profileDetails}>
          <TextField label="Window Title Regex" placeholder="^my\s+regex$" variant="outlined" className={classes.formField}
            value={stateVals.regex[0]} onChange={handleInput} name="regex"/>
            <div className={classes.areaSelectRoot}>
              <ReactRegionSelect maxRegions={1}
                                  regions={region ? [region] : []}
                                  onChange={(regions: any) => onRegionChange(regions[0])}
                                  regionRenderer={regionRenderer}
                                  constraint className={classes.areaSelector}>
                <div className={classes.areaSelector} />
              </ReactRegionSelect>

              <div className={classes.rectFields}>
                <TextField label="X" placeholder="0" type="number" className={classes.formField}
                  value={stateVals.x[0]} onChange={handleInput} name="x"/>
                <TextField label="Width" placeholder="1920" type="number" className={classes.formField}
                  value={stateVals.width[0]} onChange={handleInput} name="width"/>
                <TextField label="Y" placeholder="0" type="number" className={classes.formField}
                  value={stateVals.y[0]} onChange={handleInput} name="y"/>
                <TextField label="Height" placeholder="1080" type="number" className={classes.formField}
                  value={stateVals.height[0]} onChange={handleInput} name="height"/>
              </div>
            </div>
        </AccordionDetails>
          <Divider />
        <AccordionActions>
          <Button color="default" size="small" onClick={props.onProfileLocked}>{props.isLocked ? "Unlock" : "Lock"}</Button>
          <Button color="default" size="small" className={classes.deleteButton} onClick={props.onProfileDeleted}>Delete</Button>
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