import { useState, useEffect } from "react";
import { DeleteFilled, SettingFilled, ThunderboltFilled } from "@ant-design/icons";
import { Button, Space, Switch } from "antd";
import { DeviceTypes, IDeviceSpecification } from "./DeviceSpecification";
import { DeviceSettings } from "./DeviceSettings";

interface Props {
  device: IDeviceSpecification;
  onDeviceDeleted: () => void;
  onDeviceChanged: (device: IDeviceSpecification) => void;
}

enum WledPowerStatus {
  ON, OFF, UNREACHABLE
}

export default function DeviceEntryActions(props: Props) {
  const [powerState, setPowerState] = useState(WledPowerStatus.UNREACHABLE);
  const [dialogOpen, setDialogOpen] = useState(false);

  if (props.device.type === DeviceTypes.WLED) {
    useEffect(() => {
      const interval = setInterval(async () => {
        try {
          const res = await fetch(`http://${props.device.wledData?.ipAddress}/json`);
          if (res.status !== 200) {
            setPowerState(WledPowerStatus.UNREACHABLE);
            return;
          }
          const data = await res.json();
          setPowerState(data["state"]["on"] ? WledPowerStatus.ON : WledPowerStatus.OFF);
        } catch (e) {
          setPowerState(WledPowerStatus.UNREACHABLE);
        }
      }, 1000);
      return () => {
        clearInterval(interval);
      };
    }, [props.device.wledData]);
  }

  return (
    <>
      <Space>
        {props.device.type == DeviceTypes.WLED && props.device.wledData?.ipAddress &&
          <Switch checked={powerState === WledPowerStatus.ON} disabled={powerState === WledPowerStatus.UNREACHABLE} onClick={() => {
            const xmlHttp = new XMLHttpRequest();
            xmlHttp.open("GET", `http://${props.device.wledData?.ipAddress}/win&T=2`, true);
            xmlHttp.send(null);
            setPowerState(powerState === WledPowerStatus.ON ? WledPowerStatus.OFF : WledPowerStatus.ON);
          }} checkedChildren={<ThunderboltFilled />} unCheckedChildren={<ThunderboltFilled />} />}
        <Button type="default" onClick={() => { setDialogOpen(true); }}
          icon={<SettingFilled />} />
        <Button onClick={props.onDeviceDeleted} danger
          icon={<DeleteFilled />} />
      </Space>
      <DeviceSettings device={props.device} open={dialogOpen}
        onClosed={() => setDialogOpen(false)}
        onDeviceChanged={device => {setDialogOpen(false); props.onDeviceChanged(device);}}
      />
    </>
  )
}