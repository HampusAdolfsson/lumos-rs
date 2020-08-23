import React, { useState, useEffect } from 'react';
import { ProfileSettings } from './ProfileSettings';
import { Button, Divider, makeStyles, createStyles, Theme } from '@material-ui/core';
import AddIcon from '@material-ui/icons/Add'
import { IProfile } from '../models/Profile';
import { WebsocketService } from '../WebsocketService';

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    divider: {
      marginTop: 10,
      marginBottom: 10,
    },
    profilesScene: {
      maxWidth: 800,
    },
    button: {
      marginRight: 10,
    },
  }),
);


export function ProfilesScene() {
  const [profiles, setProfiles] = useState([] as Array<IProfile>);
  const [lockIndex, setLockIndex] = useState(undefined as (number | undefined));
  const [activeIndex, setActiveIndex] = useState(undefined as (number | undefined));

  useEffect(() => {
    const subscription = WebsocketService.Instance.receivedProfiles.subscribe(profs => setProfiles(profs));
    return () => {
      subscription.unsubscribe();
    };
  });

  const profileComponents = profiles.map((prof, i) => {
    return <ProfileSettings key={i} profile={prof} isActive={activeIndex === i} isLocked={lockIndex === i}
      onProfileChanged={prof => {
        console.log(prof);
        const newProfs: IProfile[] = JSON.parse(JSON.stringify(profiles));
        newProfs[i] = prof;
        WebsocketService.Instance.sendProfiles(newProfs);
        setProfiles(newProfs);
      }}/>
  });

  const classes = useStyles();
  return (
    <div className={classes.profilesScene}>
      {profileComponents}
      {profiles.length > 0 && <Divider className={classes.divider}/>}
      <div>
        <Button color="primary" variant="contained" disableElevation className={classes.button} startIcon={<AddIcon />}
          onClick={() => { setProfiles(profiles.concat([JSON.parse(JSON.stringify(defaultProfile))])); }}>
          Add
        </Button>
        <Button variant="outlined" disableElevation className={classes.button}>Apply</Button>
      </div>
    </div>
  );
}

const defaultProfile: IProfile = {
  regex: '',
  area: {
    x: 0, y: 0,
    width: 1920, height: 1080,
  },
};