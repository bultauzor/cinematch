import { Component } from '@angular/core';
import {ButtonComponent} from '../../atoms/button/button.component';
import {MovieCardComponent} from '../../atoms/movie-card/movie-card.component';
import {NgForOf} from '@angular/common';
import {Router, RouterLink} from '@angular/router';

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

  constructor(private router: Router) { }

  movies = [
    { title: "Movie 1" },
    { title: "Movie 2" },
    { title: "Movie 3" },
    { title: "Movie 4" },
    { title: "Movie 5" },
    { title: "Movie 6" }
  ];
  swipeState: string = '';
  activeIndex: number = 0;

  onSwipe(direction: string, index: number) {
    if (direction === 'left') {
      this.swipeState = 'swipe-left';
      this.reset(index);
      console.log('left');
    } else if(direction === 'right') {
      this.swipeState = 'swipe-right';
      this.reset(index)
      console.log('right')
    }
    if(this.movies.length-1 == index){
      setTimeout(() => {
        this.router.navigate(['/movies-swipe/result']);
      }, 300);
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
