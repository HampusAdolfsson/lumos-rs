import React, { useState } from 'react';
import { Input } from 'antd';

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
          <Input placeholder="Port Name"
          value={portName} onChange={ev => {
            setPortName(ev.target.value);
            const newVal = { portName: ev.target.value };
            props.changed(newVal);
          }} />
          </div>
}