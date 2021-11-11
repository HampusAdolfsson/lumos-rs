import React, { useState } from 'react';
import { TextField } from '@material-ui/core';

export interface IQmkData {
  hardwareId: string;
}

interface Props {
  data: IQmkData | null;
  changed: (data: IQmkData) => void;
}

export function QmkSettings(props: Props) {
  const [hardwareId, setHardwareId] = useState(props.data?.hardwareId || "");

  return <div>
          <TextField label="Hardware ID" color="primary"
            value={hardwareId} onChange={ev => {
              setHardwareId(ev.target.value);
              const newVal = { hardwareId: ev.target.value };
              props.changed(newVal);
            }} />
         </div>
}