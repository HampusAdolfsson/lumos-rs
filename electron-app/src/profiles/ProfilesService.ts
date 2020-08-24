import { IProfile } from '../models/Profile';
import { WebsocketService } from '../WebsocketService';
import { BehaviorSubject } from 'rxjs';
import Path from 'path';
import { readFileSync, existsSync, mkdirSync, writeFileSync } from 'fs';

export class ProfilesService {
  private static readonly saveFile = Path.join(process.env.APPDATA || ".", "win-rt-rgb", "profiles.json");

  public static get Instance() {
    if (!this.instance) {
      this.LoadAndInstantiate();
    }
    return this.instance!;
  }

  public static LoadAndInstantiate() {
    if (existsSync(this.saveFile)) {
      const profiles = JSON.parse(readFileSync(this.saveFile).toString());
      this.instance = new ProfilesService(profiles);
    } else {
      this.instance = new ProfilesService([]);
    }
  }

  private static instance: ProfilesService | undefined;

  public readonly profiles: BehaviorSubject<IProfile[]>;

  private constructor(initialProfiles: IProfile[]) {
    this.profiles = new BehaviorSubject(initialProfiles);
    WebsocketService.Instance.sendMessage('profiles', initialProfiles);
  }

  public setProfiles(profiles: IProfile[]) {
    this.profiles.next(profiles);
    WebsocketService.Instance.sendMessage('profiles', profiles);
    if (!existsSync(Path.dirname(ProfilesService.saveFile))) {
      mkdirSync(Path.dirname(ProfilesService.saveFile), { recursive: true });
    }
    writeFileSync(ProfilesService.saveFile, JSON.stringify(profiles));
  }

  public setLocked(profileIndex: number, monitorIndex: number) {
    if (profileIndex < 0 || monitorIndex < 0) {
      return;
    }
    WebsocketService.Instance.sendMessage('lock', { profile: profileIndex, monitor: monitorIndex });
  }

  public setUnlocked() {
    WebsocketService.Instance.sendMessage('lock', {});
  }
}