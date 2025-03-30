import {Component, OnInit} from '@angular/core';
import {HomeFooterComponent} from '../../components/molecules/home-footer/home-footer.component';
import {HomeHeaderComponent} from '../../components/molecules/home-header/home-header.component';
import {
  MoviesSwipeStartComponentComponent
} from '../../components/molecules/movies-swipe-start-component/movies-swipe-start-component.component';
import {
  MoviePageComponentComponent
} from '../../components/molecules/movie-page-component/movie-page-component.component';
import {ActivatedRoute} from '@angular/router';

@Component({
  selector: 'app-movie-page',
  imports: [
    HomeFooterComponent,
    HomeHeaderComponent,
    MoviePageComponentComponent],
  templateUrl: './movie-page.component.html',
  standalone: true,
  styleUrl: './movie-page.component.css'
})
export class MoviePageComponent{
}
