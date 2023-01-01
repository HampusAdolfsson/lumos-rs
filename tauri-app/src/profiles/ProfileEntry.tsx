import { useState } from 'react';
import { TableCell, IconButton, Typography } from '@mui/material';
import { IAreaSpecification, IProfile, MonitorDistance } from './Profile';
import { Delete, Settings } from '@mui/icons-material';
import { ProfileSettings } from './ProfileSettings';

interface Props {
  profile: IProfile;
  onProfileChanged: (profile: IProfile) => void;
  onProfileDeleted: () => void;

  activeOnMonitors: number[];
}

const circledNumbers = ["①", "②"];

export function ProfileEntry(props: Props) {
  const [dialogOpen, setDialogOpen] = useState(false);

  return (
    <>
      <TableCell>
        { props.activeOnMonitors.map(number => (
          <Typography color="primary" sx={{ height: 16 }} display="inline">{circledNumbers[number]} </Typography>
        ))}
        {props.profile.regex || "New Profile"}
      </TableCell>
      <TableCell>
        {props.profile.areas.map(areaSize).join(", ")}
      </TableCell>
      <TableCell align="right" >
        <IconButton onClick={() => {setDialogOpen(true);}}>
          <Settings fontSize="small"/>
        </IconButton>
        <IconButton onClick={props.onProfileDeleted}>
        <Delete fontSize="small" sx={{ color: "#ff5555" }}/>
        </IconButton>
      </TableCell>
      <ProfileSettings open={dialogOpen} onClosed={() => setDialogOpen(false)}
        onProfileChanged={(profile) => { setDialogOpen(false); props.onProfileChanged(profile); }}
        profile={props.profile}/>
    </>
  );
}

function areaSize(area: IAreaSpecification) {
  const distToStr = (dist: MonitorDistance) => {
    if ("px" in dist) {
      return dist.px.toString();
    } else {
      return dist.percentage + "%";
    }
  }

  return distToStr(area.width) + "x" + distToStr(area.height);
}