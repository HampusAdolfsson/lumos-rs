import React, { useState } from 'react';
import { WledSettings } from './WledSettings'
import { DeviceTypes, IDeviceSpecification, SamplingTypes } from './DeviceSpecification';
import { QmkSettings } from './QmkSettings';
import { SerialSettings } from './SerialSettings';
import { Input, Modal, Segmented, Slider, Divider, InputNumber } from 'antd';
import "./DeviceSettings.css";

interface Props {
  device: IDeviceSpecification;
  open: boolean;
  onClosed: () => void;
  onDeviceChanged: (device: IDeviceSpecification) => void;
}

export function DeviceSettings(props: Props) {
  const [dirty, setDirty] = useState(false);
  const [device, setDevice] = useState(props.device);

  function setField<K extends keyof IDeviceSpecification>(name: K, val: IDeviceSpecification[K]) {
    let newDevice = JSON.parse(JSON.stringify(device));
    newDevice[name] = val;
    setDevice(newDevice);
    setDirty(true);
  }

  const deviceComponent = device.type === DeviceTypes.WLED ?
    <WledSettings data={props.device.wledData} changed={val => { setField("wledData", val); }} /> : (
      device.type === DeviceTypes.QMK ? <QmkSettings data={props.device.qmkData} changed={val => { setField("qmkData", val) }}/> : (
      device.type === DeviceTypes.Serial ? <SerialSettings data={props.device.serialData} changed={val => { setField("serialData", val) }}/> : undefined
    ));

  return (
    <Modal open={props.open} title="Device Settings" okText="Save" onCancel={props.onClosed} onOk={() => {
      setDirty(false);
      props.onDeviceChanged(device);
    }} okButtonProps={{disabled: !dirty}}>
        <div className="form-row">
          <Input placeholder="Name" style={{ flex: 2 }}
            name="name" value={device.name} onChange={ev => setField("name", ev.target.value)} />
          <InputNumber placeholder="Number of LEDs" addonAfter="LEDs"
            value={device.numberOfLeds} onChange={val => setField("numberOfLeds", val ?? 0)} />
        </div>
        <Segmented options={["Horizontal", "Vertical"]} value={device.samplingType === SamplingTypes.Horizonal ? "Horizontal" : "Vertical"}
          onChange={value => { setField("samplingType", value === "Horizontal" ? SamplingTypes.Horizonal : SamplingTypes.Vertical ) }} />
        <Divider style={{marginBlock: 10}}/>
        <table style={{width: "100%"}}>
          <tr>
            <td>
              Audio Amount (%)
            </td>
            <td style={{width: "300px"}}>
              <Slider min={0} max={100} step={5} value={device.audioAmount} onChange={val => { setField("audioAmount", val); }}/>
            </td>
          </tr>
          <tr>
            <td>
              Color Temperature (K)
            </td>
            <td>
              <Slider min={2000} max={10000} step={100} value={device.colorTemp} onChange={val => { setField("colorTemp", val); }}/>
            </td>
          </tr>
          <tr>
            <td>
              Gamma
            </td>
            <td>
              <Slider min={1.0} max={3.0} step={0.1} value={device.gamma} onChange={val => { setField("gamma", val); }} />
            </td>
          </tr>
          <tr>
            <td>
              Saturation Increase (%)
            </td>
            <td>
              <Slider min={0} max={100} step={5} value={device.saturationAdjustment} onChange={val => { setField("saturationAdjustment", val); }} />
            </td>
          </tr>
          <tr>
            <td>
              Value Increase (%)
            </td>
            <td>
              <Slider min={0} max={100} step={5} value={device.valueAdjustment} onChange={val => { setField("valueAdjustment", val); }} />
            </td>
          </tr>
        </table>
        <Divider style={{marginBlock: 10}}/>
        <div style={{ marginBottom: 10 }}>
          <Segmented options={["WLED", "Qmk", "Serial (Adalight)"]}
            value={device.type === DeviceTypes.WLED ? "WLED" : device.type === DeviceTypes.QMK ? "Qmk" : "Serial (Adalight)"}
            onChange={val => {
              setField("type", val === "WLED" ? DeviceTypes.WLED : val === "Qmk" ? DeviceTypes.QMK : DeviceTypes.Serial);
            }} />
        </div>
        {deviceComponent}
    </Modal>
  );
}