import { Component } from '@angular/core';
import {ButtonComponent} from '../../atoms/button/button.component';
import {MovieCardComponent} from '../../atoms/movie-card/movie-card.component';
import {NgForOf} from '@angular/common';
import {Router, RouterLink} from '@angular/router';
import {WebSocketService} from '../../../services/websocket.service';

@Component({
  selector: 'app-movies-swipe-session-component',
  imports: [
    ButtonComponent,
    MovieCardComponent,
    NgForOf,
    RouterLink
  ],
  templateUrl: './movies-swipe-session-component.component.html',
  styleUrl: './movies-swipe-session-component.component.css'
})
export class MoviesSwipeSessionComponentComponent {

  constructor(private router: Router, private webSocketService: WebSocketService) { }

  movies = JSON.parse(<string>localStorage.getItem("movies"));
  swipeState: string = '';
  activeIndex: number = 0;

  async ngOnInit(): Promise<void> {

    if (this.webSocketService.ws) {
      this.webSocketService.ws.onmessage = (event) => {

        const message = JSON.parse(event.data);

        if (Object.keys(message)[0] === 'Content') {
          this.movies.push(Object.values(message)[0]);
          this.movies = [...this.movies];
        } else if(Object.keys(message)[0] === 'Result'){
          localStorage.setItem("Result", JSON.stringify(Object.values(message)[0]));
          this.router.navigate(['/movies-swipe/result']);
        }
      };
    }
  }

  onSwipe(direction: string, index: number) {
    if (direction === 'left') {
      this.swipeState = 'swipe-left';
      this.reset(index);
      this.webSocketService.sendMessage(JSON.stringify({'Vote':false}));
    } else if(direction === 'right') {
      this.swipeState = 'swipe-right';
      this.reset(index)
      this.webSocketService.sendMessage(JSON.stringify({'Vote':true}));
    }
  }

  reset(index: number){
    setTimeout(() => {
      this.swipeState = '';
      this.activeIndex = index+1;
      document.querySelector('#card-'+index)?.classList.add('hidden')
    }, 300 );
  }
}
