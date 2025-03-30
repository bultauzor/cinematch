import { Component } from '@angular/core';
import {HomeFooterComponent} from '../../components/molecules/home-footer/home-footer.component';
import {HomeHeaderComponent} from '../../components/molecules/home-header/home-header.component';
import { MoovieDisplayComponent } from '../../components/molecules/moovie-display/moovie-display.component';

@Component({
  selector: 'app-user-home',
  imports: [
    HomeFooterComponent,
    HomeHeaderComponent,
    MoovieDisplayComponent
  ],
  templateUrl: './user-home.component.html',
  styleUrl: './user-home.component.css'
})
export class UserHomeComponent {

}
