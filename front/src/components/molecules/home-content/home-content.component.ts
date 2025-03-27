import { Component } from '@angular/core';
import {ButtonComponent} from '../../atoms/button/button.component';
import {RouterLink} from '@angular/router';

@Component({
  selector: 'app-home-content',
  imports: [
    ButtonComponent,
    RouterLink
  ],
  templateUrl: './home-content.component.html',
  styleUrl: './home-content.component.css'
})
export class HomeContentComponent {

}
