import {Component, Input} from '@angular/core';
import {ButtonComponent} from '../../atoms/button/button.component';
import {RouterLink} from '@angular/router';
import {NgIf} from '@angular/common';
import {AvatarComponent} from '../../atoms/avatar/avatar.component';
import {InputSearchComponent} from '../../atoms/input-search/input-search.component';
import {OrSeparatorComponent} from '../../atoms/or-separator/or-separator.component';

@Component({
  selector: 'app-home-header',
  imports: [
    ButtonComponent,
    RouterLink,
    NgIf,
    AvatarComponent,
    InputSearchComponent,
    OrSeparatorComponent
  ],
  templateUrl: './home-header.component.html',
  styleUrl: './home-header.component.css'
})
export class HomeHeaderComponent {
  @Input() type: 'classic' | 'signin' | 'signup' | 'user_home' | 'user_classic' = 'classic';
}
