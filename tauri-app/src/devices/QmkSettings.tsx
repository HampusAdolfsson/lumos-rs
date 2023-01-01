import React, { useState } from 'react';
import { TextField } from '@mui/material';

export interface IQmkData {
  vendorId: number;
  productId: number;
}

interface Props {
  data: IQmkData | null;
  changed: (data: IQmkData) => void;
}

export function QmkSettings(props: Props) {
  const [vendorId, setVendorId] = useState(props.data?.vendorId || 0);
  const [productId, setProductId] = useState(props.data?.productId || 0);

  return <div>
          <TextField label="Vendor ID" color="primary" type="number"
            value={vendorId} onChange={ev => {
              setVendorId(Number(ev.target.value));
              const newVal = { vendorId: Number(ev.target.value), productId };
              props.changed(newVal);
            }} />
          <TextField label="Product ID" color="primary" type="number"
            value={productId} onChange={ev => {
              setProductId(Number(ev.target.value));
              const newVal = { productId: Number(ev.target.value), vendorId };
              props.changed(newVal);
            }} />
         </div>
}