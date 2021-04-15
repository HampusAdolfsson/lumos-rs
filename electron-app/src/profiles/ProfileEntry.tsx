import React, { useState } from 'react';
import { makeStyles, createStyles, Theme, TableCell, IconButton } from '@material-ui/core';
import CheckCircleIcon from '@material-ui/icons/CheckCircle';
import { IProfile } from './Profile';
import { Delete, Settings } from '@material-ui/icons';
import { ProfileSettings } from './ProfileSettings';

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    deleteButton: {
      color: "#ff5555"
    },
    icon: {
      height: 16,
    },
  }),
);

interface Props {
  profile: IProfile;
  onProfileChanged: (profile: IProfile) => void;
  onProfileDeleted: () => void;

  isActive: boolean;
}

export function ProfileEntry(props: Props) {
  const classes = useStyles();
  const [dialogOpen, setDialogOpen] = useState(false);

  return (
    <>
      <TableCell>
        { props.isActive && <CheckCircleIcon color="secondary" className={classes.icon} /> }
        {props.profile.regex || "New Profile"}
      </TableCell>
      <TableCell>
        {props.profile.area.width}x{props.profile.area.height}
      </TableCell>
      <TableCell align="right" >
        <IconButton onClick={() => {setDialogOpen(true);}}>
          <Settings fontSize="small"/>
        </IconButton>
        <IconButton onClick={props.onProfileDeleted}>
        <Delete fontSize="small" className={classes.deleteButton}/>
        </IconButton>
      </TableCell>
      <ProfileSettings open={dialogOpen} onClosed={() => setDialogOpen(false)}
        onProfileChanged={(profile) => { setDialogOpen(false); props.onProfileChanged(profile); }}
        profile={props.profile}/>
    </>
  );
}