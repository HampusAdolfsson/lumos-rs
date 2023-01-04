import { useState, useEffect } from 'react';
import { Button, Card, Space, Switch, Table, theme } from 'antd';
import { PlusOutlined } from '@ant-design/icons'
import { DeviceTypes, SamplingTypes } from './DeviceSpecification';
import { DevicesService, IExtendedDeviceSpecification } from './DevicesService';
import DeviceEntryActions from './DeviceEntryActions';
import { ColumnsType } from 'antd/es/table';

export function DevicesScene() {
  const [devices, setDevices] = useState([] as Array<IExtendedDeviceSpecification>);

  useEffect(() => {
    const subscription = DevicesService.Instance().then(service => service.devices.subscribe(profs => setDevices(profs)));
    return () => {
      subscription.then(sub => sub.unsubscribe());
    };
  });

  const columns: ColumnsType<IExtendedDeviceSpecification> = [
    {
      key: "nameAndEnabled",
      render: (_, device, i) => (
        <Space>
          <Switch checked={device.enabled} onChange={async(checked) => {
            const newDevs: IExtendedDeviceSpecification[] = JSON.parse(JSON.stringify(devices));
            newDevs[i].enabled = checked;
            (await DevicesService.Instance()).setDevices(newDevs, true);
            setDevices(newDevs);
          }}/>
          <span>{device.device.name}</span>
        </Space>
      ),
    },
    {
      key: "ledinfo",
      render: (_, device) => <span>{device.device.numberOfLeds} LEDs</span>,
    },
    {
      key: "location",
      render: (_, device) => (
          device.device.type == DeviceTypes.WLED ?
          <>WLED - <a target="_blank" href={"http://"+device.device.wledData?.ipAddress}>{device.device.wledData?.ipAddress}</a></> :
          device.device.type == DeviceTypes.QMK ?
          <>Qmk - {truncate(`${device.device.qmkData?.productId.toString(16).toUpperCase()}/${device.device.qmkData?.vendorId.toString(16).toUpperCase()}`, 14)} </> :
          <>Serial - {device.device.serialData?.portName} </>
      ),
    },
    {
      key: "actions",
      align: "right",
      render: (_, device, i) =>
        (<DeviceEntryActions device={device.device} onDeviceChanged={async(dev) => {
            const newDevs: IExtendedDeviceSpecification[] = JSON.parse(JSON.stringify(devices));
            newDevs[i].device = dev;
            (await DevicesService.Instance()).setDevices(newDevs, true);
            setDevices(newDevs);
          }} onDeviceDeleted={async() => {
            const newDevs: IExtendedDeviceSpecification[] = JSON.parse(JSON.stringify(devices));
            newDevs.splice(i, 1);
            (await DevicesService.Instance()).setDevices(newDevs, true);
            setDevices(newDevs);
          }}/>)
    }
  ];
  return (
    <div id="devicesScene">
    <Card style={{ background: "#ffffff11" }} title="Devices" extra={<Button type="primary" icon={<PlusOutlined/>} onClick={async() => {
        const newDevs = devices.concat([JSON.parse(JSON.stringify(defaultDevice))]);
        (await DevicesService.Instance()).setDevices(newDevs, true);
        setDevices(newDevs);
      }}>Add</Button>}>
      <Table dataSource={devices.map((dev, i) => { return { ...dev, key: i };})} columns={columns}
        pagination={false} showHeader={false}/>
    </Card>
    </div>
  )
}

const defaultDevice: IExtendedDeviceSpecification = {
  enabled: false,
  device: {
    name: '',
    numberOfLeds: 0,
    samplingType: SamplingTypes.Horizonal,
    gamma: 2,
    colorTemp: 5500,
    saturationAdjustment: 0,
    valueAdjustment: 0,
    audioAmount: 0,
    type: null,
    wledData: null,
    qmkData: null,
    serialData: null,
  }
};

function truncate(str: string, maxLength: number): string {
  if (str.length < maxLength) return str;
  return str.substring(0, maxLength - 1) + "…";
}