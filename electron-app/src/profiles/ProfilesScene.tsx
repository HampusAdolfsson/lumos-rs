import React from 'react';
import { ProfileSettings } from './ProfileSettings';
import { Button, Divider, makeStyles, createStyles, Theme } from '@material-ui/core';
import AddIcon from '@material-ui/icons/Add'

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
  const classes = useStyles();
  return (
    <div className={classes.profilesScene}>
      <ProfileSettings />
      <ProfileSettings />
      <Divider className={classes.divider}/>
      <div>
        <Button color="primary" variant="contained" disableElevation className={classes.button}>
          <AddIcon />
          Add
          </Button>
        <Button variant="outlined" disableElevation className={classes.button}>Apply</Button>
      </div>
    </div>
  );
}