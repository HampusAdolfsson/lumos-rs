import React from 'react';
import { AccordionSummary, AccordionDetails, Typography, Accordion, TextField, Button, Divider, AccordionActions, makeStyles, createStyles, Theme } from '@material-ui/core';
import ExpandMoreIcon from '@material-ui/icons/ExpandMore';
import LockIcon from '@material-ui/icons/Lock';
import CheckCircleIcon from '@material-ui/icons/CheckCircle';

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    heading: {
      fontSize: theme.typography.pxToRem(15),
      flexBasis: '33.33%',
      flexShrink: 0,
    },
    secondaryHeading: {
      fontSize: theme.typography.pxToRem(15),
      color: theme.palette.text.secondary,
    },
    profileDetails: {
      flexDirection: "column",
    },
    divider: {
      marginLeft: 10,
      marginRight: 10,
    },
    deleteButton: {
      color: "#ff5555",
    },
    formField: {
      marginLeft: 5,
      marginRight: 5,
    },
    rectFields: {
      display: "flex",
      flexDirection: "row",
    },
  }),
);

export function ProfileSettings() {
  const classes = useStyles();
  return (
    <>
      <Accordion>
        <AccordionSummary
          expandIcon={<ExpandMoreIcon />}>
          <Typography className={classes.heading}>Test</Typography>
          <div className={classes.secondaryHeading}><LockIcon /> <CheckCircleIcon /></div>
        </AccordionSummary>
        <AccordionDetails className={classes.profileDetails}>
          <TextField label="Window Title Regex" placeholder="^my\s+regex$" variant="outlined" className={classes.formField}/>
          <div className={classes.rectFields} >
            <TextField label="X" placeholder="0" type="number" className={classes.formField}/>
            <TextField label="Width" placeholder="1920" type="number" className={classes.formField}/>
            <TextField label="Y" placeholder="0" type="number" className={classes.formField}/>
            <TextField label="Height" placeholder="1080" type="number" className={classes.formField}/>
          </div>
        </AccordionDetails>
          <Divider />
        <AccordionActions>
          <Button color="default" size="small">Lock</Button>
          <Button color="default" size="small" className={classes.deleteButton}>Delete</Button>
        </AccordionActions>
      </Accordion>
    </>
  );
}