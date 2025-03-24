import { Component } from '@angular/core';
import {HomeFooterComponent} from "../../components/molecules/home-footer/home-footer.component";
import {HomeHeaderComponent} from "../../components/molecules/home-header/home-header.component";
import {FormAuthComponent} from '../../components/molecules/form-auth/form-auth.component';

@Component({
  selector: 'app-signup',
  imports: [
    HomeFooterComponent,
    HomeHeaderComponent,
    FormAuthComponent
  ],
  templateUrl: './signup.component.html',
  styleUrl: './signup.component.css'
})
export class SignupComponent {

}
