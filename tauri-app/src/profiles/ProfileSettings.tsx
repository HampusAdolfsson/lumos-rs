import React, { useState } from 'react';
import { TextField, Button, Dialog, DialogTitle, DialogActions, DialogContent, Typography } from '@mui/material';
import { IProfile } from './Profile';
import { AreaSpecificationsParser, composeAreaSpecifications } from './parsing/AreaSpecificationParser';
import { tokenize } from './parsing/Lexer';
import { StringReader } from './parsing/StringReader';
import { styled } from '@mui/material/styles';
import { TextFieldProps } from '@mui/material/TextField';

const FormField = styled(TextField)<TextFieldProps>(() => ({
  marginLeft: 5,
  marginRight: 5,
  marginBottom: 10,
  width: 500
}));

export interface Props {
  profile: IProfile;
  open: boolean;
  onProfileChanged: (profile: IProfile) => void;
  onClosed: () => void;
}

export function ProfileSettings(props: Props) {
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
        <DialogContent sx={{ flexDirection: "column" }}>
          <FormField label="Window Title Regex" placeholder="^my\s+regex$" variant="outlined"
            value={profile.regex} onChange={handleInput} name="regex"/>
          <FormField label="Definition(s)" multiline value={areaSpecification}
            onChange={ev => {
              setAreaSpecification(ev.target.value);
              setDirty(true);
              }} sx={{ "& .MuiOutlinedInput-root": { fontFamily: "Roboto Mono" }}}/>
          <Typography style={{color: "red"}}>{errorMsg}</Typography>
          <TextField label="Priority" type="number" value={priority} onChange={ev => {setPriority(ev.target.value); setDirty(true);}}/>
        </DialogContent>
        <DialogActions>
          <Button color="info" size="small" onClick={props.onClosed}>Cancel</Button>
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