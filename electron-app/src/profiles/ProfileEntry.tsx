import React, { useState } from 'react';
import { AccordionSummary, AccordionDetails, Typography, Accordion, TextField, Button, Divider, AccordionActions, makeStyles, createStyles, Theme, TableCell, TableRow, IconButton } from '@material-ui/core';
import ExpandMoreIcon from '@material-ui/icons/ExpandMore';
import LockIcon from '@material-ui/icons/Lock';
import CheckCircleIcon from '@material-ui/icons/CheckCircle';
import { IProfile } from './Profile';
import { Delete, LockOpen, Settings } from '@material-ui/icons';
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
  onProfileLocked: () => void;

  isLocked: boolean;
  isActive: boolean;
}

export function ProfileEntry(props: Props) {
  const classes = useStyles();
  const [dialogOpen, setDialogOpen] = useState(false);

  return (
    <>
      <TableCell>
        { props.isLocked && <LockIcon className={classes.icon} color="secondary" /> }
        { (props.isActive && !props.isLocked) && <CheckCircleIcon color="secondary" className={classes.icon} /> }
        <Typography variant="subtitle1" display="inline">{props.profile.regex.replace(/[^a-zA-Z0-9_-\s]+/g, " ") || "New Profile"}</Typography>
      </TableCell>
      <TableCell>
        {props.profile.area.width}x{props.profile.area.height}
      </TableCell>
      <TableCell align="right" >
        <IconButton onClick={() => {
          props.onProfileLocked();
        }}> { props.isLocked ? <LockOpen color="primary"/> : <LockIcon/> } </IconButton>
        <IconButton onClick={() => {setDialogOpen(true);}}>
        <Settings/>
        </IconButton>
        <IconButton onClick={props.onProfileDeleted}>
        <Delete className={classes.deleteButton}/>
        </IconButton>
      </TableCell>
      <ProfileSettings open={dialogOpen} onClosed={() => setDialogOpen(false)}
        onProfileChanged={(profile) => { setDialogOpen(false); props.onProfileChanged(profile); }}
        profile={props.profile}/>
    </>
  );
}