import { Component } from '@angular/core';
import {ButtonComponent} from '../atoms/button/button.component';

@Component({
  selector: 'app-home-content',
  imports: [
    ButtonComponent
  ],
  templateUrl: './home-content.component.html',
  styleUrl: './home-content.component.css'
})
export class HomeContentComponent {

}
