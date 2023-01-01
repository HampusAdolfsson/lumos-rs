import { IDeviceSpecification } from './DeviceSpecification';
import { WebsocketService } from '../WebsocketService';
import { BehaviorSubject } from 'rxjs';
import * as Fs from '@tauri-apps/api/fs';
import * as Path from '@tauri-apps/api/path';

export interface IExtendedDeviceSpecification {
  device: IDeviceSpecification;
  enabled: boolean;
}

export class DevicesService {
  private static readonly saveFile = async() => Path.join(await Path.appDataDir(), "devices.json");

  public static async Instance() {
    if (!this.instance) {
      await this.LoadAndInstantiate();
    }
    return this.instance!;
  }

  public static async LoadAndInstantiate() {
    const saveFile = await this.saveFile();
    if (await Fs.exists(saveFile)) {
      const devices = JSON.parse(await Fs.readTextFile(saveFile));
      this.instance = new DevicesService(devices);
    } else {
      this.instance = new DevicesService([]);
    }
  }

  private static instance: DevicesService | undefined;

  public readonly devices: BehaviorSubject<IExtendedDeviceSpecification[]>;

  private constructor(initialDevices: IExtendedDeviceSpecification[]) {
    this.devices = new BehaviorSubject(initialDevices);

    WebsocketService.Instance.sendMessage('devices', initialDevices);
  }

  public async setDevices(devices: IExtendedDeviceSpecification[], doSave: boolean) {
    this.devices.next(devices);
    WebsocketService.Instance.sendMessage('devices', devices);
    if (doSave) {
      const saveFile = await DevicesService.saveFile();
      const saveDir = await Path.dirname(saveFile);
      if (!await Fs.exists(saveDir)) {
        await Fs.createDir(saveDir, { recursive: true });
      }
      await Fs.writeTextFile(saveFile, JSON.stringify(devices));
    }
  }
}