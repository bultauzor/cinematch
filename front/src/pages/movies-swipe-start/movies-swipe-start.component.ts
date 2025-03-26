import { Component } from '@angular/core';
import {HomeFooterComponent} from "../../components/molecules/home-footer/home-footer.component";
import {HomeHeaderComponent} from "../../components/molecules/home-header/home-header.component";
import {
  MoviesSwipeStartComponentComponent
} from '../../components/molecules/movies-swipe-start-component/movies-swipe-start-component.component';

@Component({
  selector: 'app-movies-swipe-start',
  imports: [
    HomeFooterComponent,
    HomeHeaderComponent,
    MoviesSwipeStartComponentComponent
  ],
  templateUrl: './movies-swipe-start.component.html',
  styleUrl: './movies-swipe-start.component.css'
})
export class MoviesSwipeStartComponent {

}
