import { Component } from '@angular/core';
import {HomeFooterComponent} from "../../components/molecules/home-footer/home-footer.component";
import {HomeHeaderComponent} from "../../components/molecules/home-header/home-header.component";
import {
  MoviesSwipeSessionComponentComponent
} from '../../components/molecules/movies-swipe-session-component/movies-swipe-session-component.component';

@Component({
  selector: 'app-movies-swipe-session',
  imports: [
    HomeFooterComponent,
    HomeHeaderComponent,
    MoviesSwipeSessionComponentComponent
  ],
  templateUrl: './movies-swipe-session.component.html',
  styleUrl: './movies-swipe-session.component.css'
})
export class MoviesSwipeSessionComponent {

}
