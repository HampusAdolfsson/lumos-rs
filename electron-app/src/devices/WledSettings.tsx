// import React from 'react';
import React, { useState } from 'react';
import { Button, TextField } from '@material-ui/core';

export interface WledData {
  ipAddress: string;
}

interface Props {
  data: WledData | null;
  changed: (data: WledData) => void;
}

export function WledSettings(props: Props) {
  const [ipAddress, setIpAddress] = useState(props.data?.ipAddress || '');

  return <div>
          <TextField label="IP Address" placeholder="192.168.x.x" color="primary"
          value={ipAddress} onChange={ev => {
            setIpAddress(ev.target.value);
            const newVal = { ipAddress: ev.target.value };
            props.changed(newVal);
          }} />
          </div>
}