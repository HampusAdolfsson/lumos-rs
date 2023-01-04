import React, { useState } from 'react';
import { Input } from 'antd';

export interface IWledData {
  ipAddress: string;
}

interface Props {
  data: IWledData | null;
  changed: (data: IWledData) => void;
}

export function WledSettings(props: Props) {
  const [ipAddress, setIpAddress] = useState(props.data?.ipAddress || '');

  return <div>
          <Input placeholder="IP Address"
            value={ipAddress} onChange={ev => {
              setIpAddress(ev.target.value);
              const newVal = { ipAddress: ev.target.value };
              props.changed(newVal);
            }} />
          </div>
}