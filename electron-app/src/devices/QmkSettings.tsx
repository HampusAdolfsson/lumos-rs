import React, { useState } from 'react';
import { TextField } from '@material-ui/core';

export interface QmkData {
  name: string;
}

interface Props {
  data: QmkData | null;
  enabled: boolean;
  changed: (data: QmkData) => void;
}

export function QmkSettings(props: Props) {
  const [devName, setDevName] = useState(props.data?.name || '');

  return <div>
          <TextField label="USB Device Name" color="primary"
            value={devName} onChange={ev => {
              setDevName(ev.target.value);
              const newVal = { name: ev.target.value };
              props.changed(newVal);
            }} disabled={!props.enabled} />
         </div>
}