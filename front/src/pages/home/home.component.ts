import { Component } from '@angular/core';
import {HomeFooterComponent} from '../../components/molecules/home-footer/home-footer.component';
import {HomeHeaderComponent} from '../../components/molecules/home-header/home-header.component';
import {HomeContentComponent} from '../../components/molecules/home-content/home-content.component';

@Component({
  selector: 'app-home',
  imports: [
    HomeFooterComponent,
    HomeHeaderComponent,
    HomeContentComponent
  ],
  templateUrl: './home.component.html',
  styleUrl: './home.component.css'
})
export class HomeComponent {

}
