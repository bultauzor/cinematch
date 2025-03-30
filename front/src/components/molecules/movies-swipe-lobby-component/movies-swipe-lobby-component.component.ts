import { Component } from '@angular/core';
import {WebSocketService} from '../../../services/websocket.service';
import {environment} from '../../../environments/environment';
import {Router} from '@angular/router';

@Component({
  selector: 'app-movies-swipe-lobby-component',
  imports: [],
  templateUrl: './movies-swipe-lobby-component.component.html',
  styleUrl: './movies-swipe-lobby-component.component.css'
})
export class MoviesSwipeLobbyComponentComponent {
  totalParticipants: number = 0;
  connectedParticipants: number = 1;

  constructor(private router: Router,private webSocketService: WebSocketService) {}

  async ngOnInit(): Promise<void> {

    console.log(this.webSocketService.session_id);

    const token = localStorage.getItem('token');

    const responseSession = await fetch(environment.api_url + "/session/"+this.webSocketService.session_id+"/info", {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
        'Authorization': `Bearer ${token}`
      },
    })
    const session : SessionInfo = await responseSession.json();

    this.totalParticipants = session.participants.length;

    if (this.webSocketService.ws) {
      this.webSocketService.ws.onmessage = (event) => {

        const message = JSON.parse(event.data);

        if (Object.keys(message)[0] === 'UserJoined') {
          this.connectedParticipants++;
        } else if (Object.keys(message)[0] === 'UserLeaved') {
          this.connectedParticipants--;
        } else if(Object.keys(message)[0] === "Content"){
          localStorage.setItem("movies",JSON.stringify(Object.values(message)[0]));
          this.router.navigate(['/movies-swipe/session']);
        }
      };
    }
  }
}

type SessionInfo = {
  participants: string[],
  filters: string[]
}
