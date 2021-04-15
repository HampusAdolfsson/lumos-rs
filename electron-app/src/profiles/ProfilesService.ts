import { IProfileCategory } from './Profile';
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

  public readonly categories: BehaviorSubject<IProfileCategory[]>;
  public readonly activeProfile = new BehaviorSubject<number | undefined>(undefined);

  private constructor(initialCategories: IProfileCategory[]) {
    this.categories = new BehaviorSubject(initialCategories);

    WebsocketService.Instance.sendMessage('profiles', initialCategories.flatMap(cat => cat.profiles));
    WebsocketService.Instance.receivedMessage.subscribe(message => {
      if (message.subject === 'activeProfile') {
        this.activeProfile.next(message.contents);
      }
    });
  }

  public setProfiles(categories: IProfileCategory[]) {
    this.categories.next(categories);
    WebsocketService.Instance.sendMessage('profiles', categories.flatMap(cat => cat.profiles));
    if (!existsSync(Path.dirname(ProfilesService.saveFile))) {
      mkdirSync(Path.dirname(ProfilesService.saveFile), { recursive: true });
    }
    writeFileSync(ProfilesService.saveFile, JSON.stringify(categories));
  }
}