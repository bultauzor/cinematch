import { Component } from '@angular/core';
import {HomeFooterComponent} from '../../components/molecules/home-footer/home-footer.component';
import {HomeHeaderComponent} from '../../components/molecules/home-header/home-header.component';
import {FormAuthComponent} from '../../components/molecules/form-auth/form-auth.component';

@Component({
  selector: 'app-signin',
  imports: [
    HomeFooterComponent,
    HomeHeaderComponent,
    FormAuthComponent
  ],
  templateUrl: './signin.component.html',
  styleUrl: './signin.component.css'
})
export class SigninComponent {

}
