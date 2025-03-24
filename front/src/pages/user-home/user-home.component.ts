import { Component } from '@angular/core';
import {HomeFooterComponent} from '../../components/molecules/home-footer/home-footer.component';
import {HomeHeaderComponent} from '../../components/molecules/home-header/home-header.component';

@Component({
  selector: 'app-user-home',
  imports: [
    HomeFooterComponent,
    HomeHeaderComponent
  ],
  templateUrl: './user-home.component.html',
  styleUrl: './user-home.component.css'
})
export class UserHomeComponent {

}
