import {Component, Input} from '@angular/core';
import {ButtonComponent} from '../../atoms/button/button.component';
import {RouterLink} from '@angular/router';
import {NgIf} from '@angular/common';

@Component({
  selector: 'app-home-header',
  imports: [
    ButtonComponent,
    RouterLink,
    NgIf
  ],
  templateUrl: './home-header.component.html',
  styleUrl: './home-header.component.css'
})
export class HomeHeaderComponent {
  @Input() type: 'classic' | 'signin' | 'signup' = 'classic';
}
