import { Component } from '@angular/core';
import {ButtonComponent} from "../../atoms/button/button.component";
import {MovieCardComponent} from "../../atoms/movie-card/movie-card.component";
import {Router} from '@angular/router';
import {WebSocketService} from '../../../services/websocket.service';

@Component({
  selector: 'app-movies-swipe-result-component',
  imports: [
    ButtonComponent,
    MovieCardComponent,
  ],
  templateUrl: './movies-swipe-result-component.component.html',
  styleUrl: './movies-swipe-result-component.component.css'
})
export class MoviesSwipeResultComponentComponent {

  constructor(private router: Router, private webSocketService: WebSocketService) { }

  result = JSON.parse(<string>localStorage.getItem("Result"));

  async ngOnInit(): Promise<void> {

    if (this.webSocketService.ws) {
      this.webSocketService.ws.onmessage = (event) => {

        const message = JSON.parse(event.data);

        if(Object.keys(message)[0] === 'Content'){
          localStorage.setItem("movies",JSON.stringify(Object.values(message)[0]));
          this.router.navigate(['/movies-swipe/session']);
        }
      };
    }
  }

  restart(){
    alert("Restart request sent, it is necessary to have at least half of the session participants' requests");
    this.webSocketService.sendMessage(JSON.stringify({'Restart':null}));
  }
}
