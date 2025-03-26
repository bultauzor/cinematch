import { Component } from '@angular/core';
import {HomeFooterComponent} from "../../components/molecules/home-footer/home-footer.component";
import {HomeHeaderComponent} from "../../components/molecules/home-header/home-header.component";
import {
  MoviesSwipeResultComponentComponent
} from '../../components/molecules/movies-swipe-result-component/movies-swipe-result-component.component';

@Component({
  selector: 'app-movies-swipe-result',
  imports: [
    HomeFooterComponent,
    HomeHeaderComponent,
    MoviesSwipeResultComponentComponent
  ],
  templateUrl: './movies-swipe-result.component.html',
  styleUrl: './movies-swipe-result.component.css'
})
export class MoviesSwipeResultComponent {

}
