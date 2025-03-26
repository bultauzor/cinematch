import { Component } from '@angular/core';
import {ButtonComponent} from "../../atoms/button/button.component";
import {MovieCardComponent} from "../../atoms/movie-card/movie-card.component";
import {RouterLink} from '@angular/router';

@Component({
  selector: 'app-movies-swipe-result-component',
  imports: [
    ButtonComponent,
    MovieCardComponent,
    RouterLink
  ],
  templateUrl: './movies-swipe-result-component.component.html',
  styleUrl: './movies-swipe-result-component.component.css'
})
export class MoviesSwipeResultComponentComponent {

}
