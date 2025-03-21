import { Component } from '@angular/core';
import {ButtonComponent} from '../atoms/button/button.component';

@Component({
  selector: 'app-home-header',
  imports: [
    ButtonComponent
  ],
  templateUrl: './home-header.component.html',
  styleUrl: './home-header.component.css'
})
export class HomeHeaderComponent {

}
