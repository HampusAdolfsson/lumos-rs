import React, { useState, useEffect } from 'react';
import { ProfileSettings } from './ProfileSettings';
import { Button, Divider, makeStyles, createStyles, Theme } from '@material-ui/core';
import AddIcon from '@material-ui/icons/Add'
import { IProfile } from '../models/Profile';
import { ProfilesService } from './ProfilesService';
import { MonitorDialog } from './MonitorDialog';

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
  const [lockCandidateIndex, setlockCandidateIndex] = useState(undefined as (number | undefined));

  useEffect(() => {
    const subscription = ProfilesService.Instance.profiles.subscribe(profs => setProfiles(profs));
    return () => {
      subscription.unsubscribe();
    };
  });

  const profileComponents = profiles.map((prof, i) => {
    return <ProfileSettings key={prof.regex} profile={prof} isActive={activeIndex === i} isLocked={lockIndex === i}
      onProfileChanged={prof => {
        const newProfs: IProfile[] = JSON.parse(JSON.stringify(profiles));
        newProfs[i] = prof;
        ProfilesService.Instance.setProfiles(newProfs);
        setProfiles(newProfs);
      }}
      onProfileDeleted={() => {
        const newProfs: IProfile[] = JSON.parse(JSON.stringify(profiles));
        newProfs.splice(i, 1);
        ProfilesService.Instance.setProfiles(newProfs);
        setProfiles(newProfs);
      }}
      onProfileLocked={() => {
        if (i === lockIndex) {
          ProfilesService.Instance.setLocked(undefined);
          setLockIndex(undefined);
        } else {
          setlockCandidateIndex(i);
        }
      }}
      />
  });

  const classes = useStyles();
  return (
    <div className={classes.profilesScene}>
      {profileComponents}
      {profiles.length > 0 && <Divider className={classes.divider}/>}
      <div>
        <Button color="primary" variant="contained" disableElevation className={classes.button} startIcon={<AddIcon />}
          onClick={() => {
              const newProfs = profiles.concat([JSON.parse(JSON.stringify(defaultProfile))]);
              ProfilesService.Instance.setProfiles(newProfs);
              setProfiles(newProfs);
            }}>
          Add
        </Button>
      </div>
      <MonitorDialog open={lockCandidateIndex !== undefined} onCancel={() => setlockCandidateIndex(undefined)}
        onSuccess={idx => {
          ProfilesService.Instance.setLocked(lockCandidateIndex!, idx);
          setLockIndex(lockCandidateIndex);
          setlockCandidateIndex(undefined);
        }}/>
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