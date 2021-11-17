import { Box, Button, Collapse, createStyles, Dialog, DialogActions, DialogContent, DialogTitle, IconButton, makeStyles, Table, TableBody, TableCell, TableRow, TextField, Theme, Typography } from '@material-ui/core';
import { Add, Delete, Edit, KeyboardArrowDown, KeyboardArrowRight, KeyboardArrowUp, Settings } from '@material-ui/icons';
import React, { useState } from 'react';
import { IProfile, IProfileCategory } from './Profile';
import { ProfileEntry } from './ProfileEntry';
import { ProfileSettings } from './ProfileSettings';
import { ProfilesService } from './ProfilesService';

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    deleteButton: {
      color: "#ff5555"
    },
    icon: {
      height: 16,
    },
    addButton: {
      marginTop: 10,
    },
    formField: {
      width: "100%",
      paddingBottom: 5
    }
  }),
);

interface Props {
  category: IProfileCategory;
  onCategoryChanged: (category: IProfileCategory) => void;
  onCategoryDeleted: () => void;
  activeProfiles: Map<number, number>;
}

const circledNumbers = ["①", "②"];

export function ProfileCategory(props: Props) {
  const classes = useStyles();
  const [expanded, setExpanded] = useState(false);
  const [open, setOpen] = useState(false);
  const [name, setName] = useState(props.category.name);
  const [priority, setPriority] = useState(props.category.priority);

  const activeMonitors: number[] = [];
  const activeProfilesRows: JSX.Element[] = [];
  props.activeProfiles.forEach((val, key) => {
    if (props.category.profiles.map(prof => prof.id).includes(val)) {
      activeMonitors.push(key);
      activeProfilesRows.push((
        <TableRow>
        <TableCell style={{ paddingBottom: 0, paddingTop: 0, backgroundColor: '#484848', paddingLeft: 80 }} colSpan={6}>
          <Box margin={1}>
            <Typography variant="subtitle1" display="inline" color="primary">{circledNumbers[key]}  </Typography>
            {props.category.profiles.find(prof => prof.id === val)?.regex}
          </Box>
        </TableCell>
        </TableRow>
      ));
    }
  });

  return (
    <>
      <TableRow>
        <TableCell>
          <IconButton onClick={() => setExpanded(!expanded)}>
            { expanded ? <KeyboardArrowDown/> : <KeyboardArrowRight/> }
          </IconButton>
          <Typography variant="subtitle1" display="inline">{props.category.name || "New Category"}</Typography>
          {/* <Typography variant="subtitle1" display="inline" color="primary"> {activeMonitors.map(monitor => circledNumbers[monitor]).join(" ")}</Typography> */}
        </TableCell>
        <TableCell>
          {props.category.profiles.length} profile(s)
        </TableCell>
        <TableCell align="right" >
          <IconButton onClick={() => {setOpen(true);}}>
            <Edit/>
          </IconButton>
          <IconButton  onClick={props.onCategoryDeleted}>
            <Delete className={classes.deleteButton}/>
          </IconButton>
        </TableCell>
      </TableRow>
        {!expanded && activeProfilesRows}
      <TableRow>
        <TableCell style={{ paddingBottom: 0, paddingTop: 0, backgroundColor: '#484848' }} colSpan={6}>
          <Collapse in={expanded} timeout="auto" unmountOnExit>
            <Box margin={1}>
              <Table size="small">
                <TableBody>
                  {props.category.profiles.map((profile, i) => (
                    <TableRow>
                      <ProfileEntry profile={profile} activeOnMonitors={allKeys(props.activeProfiles, profile.id)}
                        onProfileChanged={prof => {
                          const newCategory = JSON.parse(JSON.stringify(props.category));
                          newCategory.profiles[i] = prof;
                          props.onCategoryChanged(newCategory);
                        }}
                        onProfileDeleted={() => {
                          const newCategory = JSON.parse(JSON.stringify(props.category));
                          newCategory.profiles.splice(i, 1);
                          props.onCategoryChanged(newCategory);
                        }}
                      />
                    </TableRow>
                  ))}
                </TableBody>
              </Table>
              <Button color="primary" disableElevation startIcon={<Add/>} className={classes.addButton}
                onClick={() => {
                    const newCategory = JSON.parse(JSON.stringify(props.category));
                    newCategory.profiles.push(JSON.parse(JSON.stringify(ProfilesService.Instance.createProfile())));
                    props.onCategoryChanged(newCategory);
                  }}>
                Add Profile
              </Button>
            </Box>
          </Collapse>
        </TableCell>
      </TableRow>
      <Dialog open={open}>
        <DialogTitle>Category Properties</DialogTitle>
        <DialogContent>
          <TextField label="Name" value={name} onChange={(ev) => {setName(ev.target.value);}} className={classes.formField} />
          <TextField label="Priority" type="number" value={priority} onChange={(ev) => {setPriority(Number(ev.target.value));}}
            className={classes.formField}/>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setOpen(false)}>Cancel</Button>
          <Button color="primary" onClick={() => {
            const newCategory: IProfileCategory = JSON.parse(JSON.stringify(props.category));
            newCategory.name = name;
            newCategory.priority = priority;
            props.onCategoryChanged(newCategory);
            setOpen(false);
          }}>Save</Button>
        </DialogActions>
      </Dialog>
    </>
  );
}

/**
 * Returns all keys that map to a given value
 */
function allKeys<T, U>(map: Map<T, U>, target: U): T[] {
  const results: T[] = [];
  map.forEach((val, key) => {
    if (val === target) results.push(key);
  });
  return results;
}
