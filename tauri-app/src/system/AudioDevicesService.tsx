import { WebsocketService } from '../WebsocketService';
import { BehaviorSubject } from 'rxjs';
import * as Fs from '@tauri-apps/api/fs';
import * as Path from '@tauri-apps/api/path';

export class AudioDevicesService {
  private static readonly saveFile = async() => Path.join(await Path.appDataDir(), "audio_devices.json");

  public static async Instance() {
    if (!this.instance) {
      this.instance = this.LoadAndInstantiate();
    }
    return this.instance;
  }

  public static async LoadAndInstantiate() {
    const saveFile = await this.saveFile();
    if (await Fs.exists(saveFile)) {
      const devices = JSON.parse(await Fs.readTextFile(saveFile));
      return new AudioDevicesService(devices);
    } else {
      return new AudioDevicesService([]);
    }
  }

  private static instance: Promise<AudioDevicesService> | undefined = undefined;

  public readonly audioDevices: BehaviorSubject<string[]>;

  private constructor(initialDevices: string[]) {
    this.audioDevices = new BehaviorSubject(initialDevices);

    WebsocketService.Instance.sendMessage('audio-devices', initialDevices);
  }

  public async setAudioDevices(audioDevices: string[], doSave: boolean) {
    this.audioDevices.next(audioDevices);
    WebsocketService.Instance.sendMessage('audio-devices', audioDevices);
    if (doSave) {
      const saveFile = await AudioDevicesService.saveFile();
      const saveDir = await Path.dirname(saveFile);
      if (!await Fs.exists(saveDir)) {
        await Fs.createDir(saveDir, { recursive: true });
      }
      await Fs.writeTextFile(saveFile, JSON.stringify(audioDevices));
    }
  }
}