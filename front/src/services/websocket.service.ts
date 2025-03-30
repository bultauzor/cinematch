import { Injectable } from '@angular/core';
import {environment} from '../environments/environment';

@Injectable({
  providedIn: 'root'
})
export class WebSocketService {
  ws: WebSocket | null = null;
  session_id: string | null = null;

  joinSession(sessionId: string, token: string): void {

    this.session_id = sessionId;

    if (this.ws) {
      console.warn('WebSocket already connected');
      return;
    }

    const wsUrl = `${environment.api_url.replace(/^https:\/\//, "wss://").replace(/^http:\/\//, "ws://")}/session/${sessionId}`;
    this.ws = new WebSocket(wsUrl, encodeURIComponent(token));

    this.ws.onopen = () => {
      console.log('WebSocket connection opened');
    };

    this.ws.onmessage = (event) => {
      console.log('Received message:', event.data);
    };

    this.ws.onerror = (error) => {
      console.error('WebSocket error:', error);
    };

    this.ws.onclose = () => {
      console.log('WebSocket connection closed');
    };
  }

  sendMessage(message: string): void {
    if (this.ws && this.ws.readyState === WebSocket.OPEN) {
      this.ws.send(message);
    } else {
      console.warn('WebSocket is not connected');
    }
  }

  closeConnection(): void {
    if (this.ws) {
      this.ws.close();
      this.ws = null;
    }
  }

  isConnected(): boolean {
    return this.ws ? this.ws.readyState === WebSocket.OPEN : false;
  }
}
