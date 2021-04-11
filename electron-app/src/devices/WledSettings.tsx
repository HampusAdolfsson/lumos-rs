// import React from 'react';
import React, { useState } from 'react';
import { Button, TextField } from '@material-ui/core';

export interface WledData {
  ipAddress: string;
}

interface Props {
  data: WledData | null;
  enabled: boolean;
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
          }} disabled={!props.enabled} />
          <Button style={{ marginTop: 15, marginLeft: 20 }} size="small" color="secondary" disabled={!ipAddress} onClick={() => {
            const xmlHttp = new XMLHttpRequest();
            xmlHttp.open( "GET", `http://${ipAddress}/win&T=2`, true);
            xmlHttp.send( null );
          }}>Toggle Power</Button>
          </div>
}