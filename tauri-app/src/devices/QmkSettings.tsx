import React, { useState } from 'react';
import { InputNumber, Space, Tooltip } from 'antd';

export interface IQmkData {
  vendorId: number;
  productId: number;
}

interface Props {
  data: IQmkData | null;
  changed: (data: IQmkData) => void;
}

export function QmkSettings(props: Props) {
  const [vendorId, setVendorId] = useState(props.data?.vendorId ?? 0);
  const [productId, setProductId] = useState(props.data?.productId ?? 0);

  return <Space>
          <Tooltip trigger={["focus"]} title="Vendor ID" placement="bottom">
            <InputNumber placeholder="Vendor ID"
              value={vendorId} onChange={val => {
                setVendorId(val ?? 0);
                const newVal = { vendorId: val ?? 0, productId };
                props.changed(newVal);
              }} />
          </Tooltip>
          <Tooltip trigger={["focus"]} title="Product ID" placement="bottom">
            <InputNumber placeholder="Product ID"
              value={productId} onChange={val => {
                setProductId(val ?? 0);
                const newVal = { productId: val ?? 0, vendorId };
                props.changed(newVal);
              }} />
          </Tooltip>
         </Space>
}