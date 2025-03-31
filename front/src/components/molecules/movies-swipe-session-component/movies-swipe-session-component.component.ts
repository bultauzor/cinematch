import { Component } from '@angular/core';
import {ButtonComponent} from '../../atoms/button/button.component';
import {MovieCardComponent} from '../../atoms/movie-card/movie-card.component';
import {NgForOf} from '@angular/common';
import {Router, RouterLink} from '@angular/router';
import {WebSocketService} from '../../../services/websocket.service';
import {environment} from '../../../environments/environment';
import {Content} from '../../../models/api';

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

  movies_id = JSON.parse(<string>localStorage.getItem("movies"));
  movies: Content[] = [];
  swipeState: string = '';
  activeIndex: number = 0;

  async ngOnInit(): Promise<void> {
    for (const movie_id of this.movies_id) {
      this.movies.push(await this.getMovie(movie_id))
    }
    this.movies = [...this.movies];
    console.log(this.movies[0].title)
    if (this.webSocketService.ws) {
      this.webSocketService.ws.onmessage = async (event) => {

        const message = JSON.parse(event.data);

        if (Object.keys(message)[0] === 'Content') {
          this.movies.push(await this.getMovie(Object.values(message)[0] as string));
          this.movies = [...this.movies];
        } else if (Object.keys(message)[0] === 'Result') {
          localStorage.setItem("Result", JSON.stringify(await this.getMovie(Object.values(message)[0] as string)));
          await this.router.navigate(['/movies-swipe/result']);
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

  async getMovie(index: string) {
    const token = localStorage.getItem('token');
    if (token != null) {
      const response = await fetch(environment.api_url + "/content/" + index, {
        method: "GET",
        headers: {
          "Content-Type": "application/json",
          'Authorization': `Bearer ${token}`
        },
      })
      return response.json();
    }
  }

  protected readonly String = String;
}
