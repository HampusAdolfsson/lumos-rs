import { Box, Button, Collapse, createStyles, Dialog, DialogActions, DialogContent, DialogTitle, IconButton, makeStyles, Table, TableBody, TableCell, TableRow, TextField, Theme, Typography } from '@material-ui/core';
import { Add, Delete, Edit, KeyboardArrowDown, KeyboardArrowRight, KeyboardArrowUp, Settings } from '@material-ui/icons';
import React, { useState } from 'react';
import { IProfile, IProfileCategory } from './Profile';
import { ProfileEntry } from './ProfileEntry';
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
  category: IProfileCategory;
  onCategoryChanged: (category: IProfileCategory) => void;
  onCategoryDeleted: () => void;
}

export function ProfileCategory(props: Props) {
  const classes = useStyles();
  const [expanded, setExpanded] = useState(false);
  const [open, setOpen] = useState(false);
  const [name, setName] = useState(props.category.name);

  return (
    <>
      <TableRow>
        <TableCell>
          <IconButton onClick={() => setExpanded(!expanded)}>
            { expanded ? <KeyboardArrowDown/> : <KeyboardArrowRight/> }
          </IconButton>
          <Typography variant="subtitle1" display="inline">{props.category.name || "New Category"}</Typography>
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
      <TableRow>
        <TableCell style={{ paddingBottom: 0, paddingTop: 0, backgroundColor: '#484848' }} colSpan={6}>
          <Collapse in={expanded} timeout="auto" unmountOnExit>
            <Box margin={1}>
              <Table size="small">
                <TableBody>
                  {props.category.profiles.map((profile, i) => (
                    <TableRow>
                      <ProfileEntry profile={profile} isActive={false}
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
              <Button color="primary" disableElevation startIcon={<Add/>}
                onClick={() => {
                    const newCategory = JSON.parse(JSON.stringify(props.category));
                    newCategory.profiles.push(JSON.parse(JSON.stringify(defaultProfile)));
                    props.onCategoryChanged(newCategory);
                  }}>
                Add Profile
              </Button>
            </Box>
          </Collapse>
        </TableCell>
      </TableRow>
      <Dialog open={open}>
        <DialogTitle>Set Name</DialogTitle>
        <DialogContent>
          <TextField value={name} onChange={(ev) => {setName(ev.target.value);}}/>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setOpen(false)}>Cancel</Button>
          <Button color="primary" onClick={() => {
            const newCategory = JSON.parse(JSON.stringify(props.category));
            newCategory.name = name;
            props.onCategoryChanged(newCategory);
            setOpen(false);
          }}>Save</Button>
        </DialogActions>
      </Dialog>
    </>
  );
}

const defaultProfile: IProfile = {
  regex: '',
  area: {
    x: 0, y: 0,
    width: 1920, height: 1080,
  },
};