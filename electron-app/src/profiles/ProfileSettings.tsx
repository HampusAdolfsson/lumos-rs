import React, { useState } from 'react';
import { TextField, Button, makeStyles, createStyles, Theme, Dialog, DialogTitle, DialogActions, DialogContent, TextareaAutosize, Typography } from '@material-ui/core';
// @ts-ignore
import { IProfile } from './Profile';
import { AreaSpecificationsParser, composeAreaSpecifications } from './parsing/AreaSpecificationParser';
import { tokenize } from './parsing/Lexer';
import { StringReader } from './parsing/StringReader';

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    profileDetails: {
      flexDirection: 'column',
    },
    formField: {
      marginLeft: 5,
      marginRight: 5,
      marginBottom: 10,
      width: 500
    },
    definitions: {
      fontFamily: "Roboto Mono"
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
  const [areaSpecification, setAreaSpecification] = useState(composeAreaSpecifications(props.profile.areas));
  const [dirty, setDirty] = useState(false);
  const [errorMsg, setErrorMsg] = useState("");


  const handleInput = (event: React.ChangeEvent<HTMLTextAreaElement | HTMLInputElement>) => {
    const target = event.target;
    const name = target.name;
    let newProfile = JSON.parse(JSON.stringify(profile));
    newProfile[name] = target.value;
    setProfile(newProfile);
    setDirty(true);
  }

  const [priority, setPriority] = useState(props.profile.priority?.toString() ?? "");

  return (
    <>
      <Dialog open={props.open} onClose={props.onClosed}>
        <DialogTitle>
          Profile Settings
        </DialogTitle>
        <DialogContent className={classes.profileDetails}>
          <TextField label="Window Title Regex" placeholder="^my\s+regex$" variant="outlined" className={classes.formField}
            value={profile.regex} onChange={handleInput} name="regex"/>
          <TextField className={`${classes.formField}`} label="Definition(s)" multiline value={areaSpecification}
            onChange={ev => {
              setAreaSpecification(ev.target.value);
              setDirty(true);
              }} InputProps={{ classes: { input: classes.definitions }}}/>
          <Typography style={{color: "red"}}>{errorMsg}</Typography>
          <TextField label="Priority" type="number" value={priority} onChange={ev => {setPriority(ev.target.value); setDirty(true);}}/>
        </DialogContent>
        <DialogActions>
          <Button color="default" size="small" onClick={props.onClosed}>Cancel</Button>
          <Button color="primary" size="small" disabled={!dirty} onClick={() => {
              try {
                const areas = new AreaSpecificationsParser().parse(tokenize(new StringReader(areaSpecification)));
                profile.areas = areas;
                profile.priority = priority === "" || priority === undefined ? undefined : Number(priority);
                setDirty(false);
                props.onProfileChanged(profile);
              } catch(e) {
                setErrorMsg(e as string);
              }
            }}>
            Save
          </Button>
        </DialogActions>
      </Dialog>
    </>
  );
}