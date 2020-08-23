import { Subject } from 'rxjs';
import 'core-js/stable';
import 'regenerator-runtime/runtime';

export class WebsocketService {
  public static get Instance() {
    if (!this.instance) {
      this.instance = new WebsocketService();
    }
    return this.instance;
  }

  private static instance: WebsocketService | undefined;

  private static readonly PORT = 9901;

  private readonly socket: Promise<WebSocket>;

  readonly connected: Promise<boolean>;
  readonly receivedMessage = new Subject<{ subject: string, contents: any}>();


  constructor() {
    this.socket = new Promise((resolve, reject) => {
      try {
        const websocket = new WebSocket(`ws://${window.location.hostname}:${WebsocketService.PORT}`);
        websocket.addEventListener('open', () => resolve(websocket) );
        websocket.addEventListener('error', (ev: Event) => reject());
      } catch (e) {
        reject(e);
      }
    });
    this.socket.then(socket => {
      socket.onmessage = this.handleMessage.bind(this);
    });
    this.connected = this.socket.then(() => true).catch(() => false);
  }

  async sendMessage(subject: string, contents: any): Promise<void> {
    const message = {
      subject,
      contents,
    };
    (await this.socket).send(JSON.stringify(message));
  }

  private handleMessage(event: MessageEvent): void {
    const data = JSON.parse(event.data);
    const subject = data['subject'];
    const contents = data['contents'];
    this.receivedMessage.next({ subject, contents });
  }
}
