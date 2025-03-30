import { Component } from '@angular/core';
import {HomeFooterComponent} from "../../components/molecules/home-footer/home-footer.component";
import {HomeHeaderComponent} from "../../components/molecules/home-header/home-header.component";
import {
  MoviesSwipeLobbyComponentComponent
} from '../../components/molecules/movies-swipe-lobby-component/movies-swipe-lobby-component.component';

@Component({
  selector: 'app-movies-swipe-lobby',
  imports: [
    HomeFooterComponent,
    HomeHeaderComponent,
    MoviesSwipeLobbyComponentComponent
  ],
  templateUrl: './movies-swipe-lobby.component.html',
  styleUrl: './movies-swipe-lobby.component.css'
})
export class MoviesSwipeLobbyComponent {

}
