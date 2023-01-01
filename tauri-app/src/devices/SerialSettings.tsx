import React, { useState } from 'react';
import { TextField } from '@mui/material';

export interface ISerialData {
  portName: string;
}

interface Props {
  data: ISerialData | null;
  changed: (data: ISerialData) => void;
}

export function SerialSettings(props: Props) {
  const [portName, setPortName] = useState(props.data?.portName || '');

  return <div>
          <TextField label="Port Name" placeholder="COM3" color="primary"
          value={portName} onChange={ev => {
            setPortName(ev.target.value);
            const newVal = { portName: ev.target.value };
            props.changed(newVal);
          }} />
          </div>
}