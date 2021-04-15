import React, { useState } from 'react';
import { TextField, Button, makeStyles, createStyles, Theme, Dialog, DialogTitle, DialogActions, DialogContent } from '@material-ui/core';
import ReactRegionSelect from 'react-region-select';
import { IProfile } from './Profile';

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    profileDetails: {
      flexDirection: 'column',
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
    dragIcon: {
      height: 16,
      color: '#aaa',
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
  open: boolean;
  onProfileChanged: (profile: IProfile) => void;
  onClosed: () => void;
}

export function ProfileSettings(props: Props) {
  const classes = useStyles();
  const [profile, setProfile] = useState(props.profile);
  const [dirty, setDirty] = useState(false);


  const handleInput = (event: React.ChangeEvent<HTMLTextAreaElement | HTMLInputElement>) => {
    const target = event.target;
    const name = target.name;
    let newProfile = JSON.parse(JSON.stringify(profile));
    newProfile[name] = target.value;
    setProfile(newProfile);
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
    let newProfile = JSON.parse(JSON.stringify(profile));
    if (sc.x != profile.area.x) { newProfile.area.x = sc.x; }
    if (sc.width != profile.area.width) { newProfile.area.width = sc.width; }
    if (sc.y != profile.area.y) { newProfile.area.y = sc.y; }
    if (sc.height != profile.area.height) { newProfile.area.height = sc.height; }
    setProfile(newProfile);
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
      <Dialog open={props.open} onClose={props.onClosed}>
        <DialogTitle>
          Profile Settings
        </DialogTitle>
        <DialogContent className={classes.profileDetails}>
          <TextField label="Window Title Regex" placeholder="^my\s+regex$" variant="outlined" className={classes.formField}
            value={profile.regex} onChange={handleInput} name="regex"/>
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
                  value={profile.area.x} onChange={handleInput} name="x"/>
                <TextField label="Width" placeholder="1920" type="number" className={classes.formField}
                  value={profile.area.width} onChange={handleInput} name="width"/>
                <TextField label="Y" placeholder="0" type="number" className={classes.formField}
                  value={profile.area.y} onChange={handleInput} name="y"/>
                <TextField label="Height" placeholder="1080" type="number" className={classes.formField}
                  value={profile.area.height} onChange={handleInput} name="height"/>
              </div>
            </div>
        </DialogContent>
        <DialogActions>
          <Button color="default" size="small" onClick={props.onClosed}>Cancel</Button>
          <Button color="primary" size="small" disabled={!dirty} onClick={() => {
              setDirty(false);
              props.onProfileChanged(profile);
            }}>
            Save
          </Button>
        </DialogActions>
      </Dialog>
    </>
  );
}