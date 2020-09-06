import { IDeviceSpecification } from './DeviceSpecification';
import { WebsocketService } from '../WebsocketService';
import { BehaviorSubject } from 'rxjs';
import Path from 'path';
import { readFileSync, existsSync, mkdirSync, writeFileSync } from 'fs';

export class DevicesService {
  private static readonly saveFile = Path.join(process.env.APPDATA || ".", "win-rt-rgb", "devices.json");

  public static get Instance() {
    if (!this.instance) {
      this.LoadAndInstantiate();
    }
    return this.instance!;
  }

  public static LoadAndInstantiate() {
    if (existsSync(this.saveFile)) {
      const profiles = JSON.parse(readFileSync(this.saveFile).toString());
      this.instance = new DevicesService(profiles);
    } else {
      this.instance = new DevicesService([]);
    }
  }

  private static instance: DevicesService | undefined;

  public readonly devices: BehaviorSubject<IDeviceSpecification[]>;

  private constructor(initialDevices: IDeviceSpecification[]) {
    this.devices = new BehaviorSubject(initialDevices);

    WebsocketService.Instance.sendMessage('devices', initialDevices);
  }

  public setDevices(devices: IDeviceSpecification[]) {
    this.devices.next(devices);
    WebsocketService.Instance.sendMessage('devices', devices);
    if (!existsSync(Path.dirname(DevicesService.saveFile))) {
      mkdirSync(Path.dirname(DevicesService.saveFile), { recursive: true });
    }
    writeFileSync(DevicesService.saveFile, JSON.stringify(devices));
  }
}