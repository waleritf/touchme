export type Message = {
  kind: string,
  x: number,
  y: number,
}

export class Ws {
  socket: WebSocket;
  url: string;

  constructor(url: string) {
    this.socket = new WebSocket(url);
    this.url = url;
  }

  isClosed = (): boolean => {
    return (
      !this.socket ||
      this.socket.readyState === WebSocket.CLOSING ||
      this.socket.readyState === WebSocket.CLOSED
    );
  }

  send = (message: object): void => {
    console.log(message);
    this.socket = this.isClosed() ? new WebSocket(this.url) : this.socket;
    this.socket.send(JSON.stringify(message));
  }
}