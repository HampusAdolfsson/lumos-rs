import { IProfile, IProfileCategory } from './Profile';
import { WebsocketService } from '../WebsocketService';
import { BehaviorSubject } from 'rxjs';
import { DevicesService } from '../devices/DevicesService';
import * as Fs from '@tauri-apps/api/fs';
import * as Path from '@tauri-apps/api/path';

export class ProfilesService {
  private static readonly profilesSaveFile = async() => Path.join(await Path.appDataDir(), "profiles.json");
  private static readonly idSaveFile = async() => Path.join(await Path.appDataDir(), "profiles_id.json");

  public static async Instance() {
    if (!this.instance) {
      await this.LoadAndInstantiate();
    }
    return this.instance!;
  }

  public static async LoadAndInstantiate() {
    let profiles: IProfileCategory[] = [];
    let nextId = 0;
    const profilesSaveFile = await this.profilesSaveFile();
    if (await Fs.exists(profilesSaveFile)) {
      profiles = JSON.parse(await Fs.readTextFile(profilesSaveFile));
      profiles.forEach(cat => {
        cat.profiles.forEach(prof => {
          prof.areas.forEach(area => {
            // @ts-ignore
            if (area.selector === "*") {
              area.selector = undefined;
            }
          });
        });
      });
    }
    const idSaveFile = await this.idSaveFile();
    if (await Fs.exists(idSaveFile)) {
      nextId = JSON.parse(await Fs.readTextFile(idSaveFile)).nextId;
    }
    this.instance = new ProfilesService(profiles, nextId);
  }

  private static instance: ProfilesService | undefined;

  public readonly categories: BehaviorSubject<IProfileCategory[]>;
  public readonly activeProfiles = new BehaviorSubject<Map<number, number>>(new Map());

  private nextId: number;

  private constructor(initialCategories: IProfileCategory[], nextId: number) {
    this.categories = new BehaviorSubject(initialCategories);
    this.nextId = nextId;

    this.sendProfiles(initialCategories);
    WebsocketService.Instance.receivedMessage.subscribe(async message => {
      if (message.subject === 'activeProfile') {
        const hadProfile = this.activeProfiles.value.size > 0;
        const monitorIndex = message.contents["monitor"];
        let newMap = new Map(this.activeProfiles.value);
        if (message.contents["profile"] != undefined) {
          newMap.set(monitorIndex, message.contents["profile"]);
        } else {
          newMap.delete(monitorIndex);
        }
        const hasProfile = newMap.size > 0;
        if (hasProfile != hadProfile) {
          const service = await DevicesService.Instance();
          service.setDevices(hasProfile ? service.devices.value : [], false);
        }
        this.activeProfiles.next(newMap);
      }
    });
  }

  public setProfiles(categories: IProfileCategory[]) {
    this.categories.next(categories);
    this.sendProfiles(categories);
    ProfilesService.profilesSaveFile().then(async profilesSaveFile => {
      let profilesSaveDir = await Path.dirname(profilesSaveFile);
      if (!Fs.exists(profilesSaveDir)) {
        Fs.createDir(profilesSaveDir, { recursive: true });
      }
      Fs.writeTextFile(profilesSaveFile, JSON.stringify(categories));
    });
  }

  public createProfile(): IProfile {
    const profile: IProfile = {
      id: this.nextId,
      regex: '',
      priority: undefined,
      areas: [{
        selector: undefined,
        direction: "both",
        x: { px: 0 },
        y: { px: 0 },
        width: { percentage: 100 },
        height: { percentage: 100 },
      }]
    };
    this.nextId += 1;

    ProfilesService.idSaveFile().then(idSaveFile => {
      Fs.writeTextFile(idSaveFile, JSON.stringify({ nextId: this.nextId }));
    });
    return profile;
  }

  private sendProfiles(categories: IProfileCategory[]) {
    categories = categories.filter(category => category.enabled);
    const flattenedProfiles = categories.flatMap(category => {
      const profiles: IProfile[] = JSON.parse(JSON.stringify(category.profiles));
      profiles.forEach(profile => {
        if (profile.priority === undefined) {
          profile.priority = category.priority;
        }
      });
      return profiles;
    });
    WebsocketService.Instance.sendMessage('profiles', flattenedProfiles);
  }

}