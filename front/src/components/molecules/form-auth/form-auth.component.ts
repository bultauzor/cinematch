import {Component, Input} from '@angular/core';
import {ButtonComponent} from '../../atoms/button/button.component';
import {InputFormComponent} from '../../atoms/input-form/input-form.component';
import {NgIf} from '@angular/common';
import {environment} from '../../../environments/environment';

@Component({
  selector: 'app-form-auth',
  imports: [
    ButtonComponent,
    InputFormComponent,
    NgIf,
  ],
  templateUrl: './form-auth.component.html',
  styleUrl: './form-auth.component.css'
})
export class FormAuthComponent {
  @Input() name: 'SIGN UP' | 'SIGN IN' = 'SIGN UP';

  username: string = '';
  password: string = '';
  confirmPassword: string = '';

  usernameError: string = '';
  passwordError: string = '';
  confirmPasswordError: string = '';

  onSubmit() {
    this.resetErrors();

    let isValid = true;

    if (!this.username.trim()) {
      this.usernameError = 'Username is required';
      isValid = false;
    } else if (this.username.length <= 2) {
      this.usernameError = 'Username must be at least 2 characters';
      isValid = false;
    } else if (this.username.length >= 32) {
      this.usernameError = 'Username must be at most 32 characters';
      isValid = false;
    }

    if (!this.password.trim()) {
      this.passwordError = 'Password is required';
      isValid = false;
    } else if (this.password.length <= 4) {
      this.passwordError = 'Password must be at least 4 characters';
      isValid = false;
    } else if (this.password.length >= 128) {
      this.passwordError = 'Password must be at most 128 characters';
      isValid = false;
    }

    if (this.name == 'SIGN UP' && !this.confirmPassword.trim()) {
      this.confirmPasswordError = 'Confirm password is required';
      isValid = false;
    } else if ( this.name == 'SIGN UP' && this.password != this.confirmPassword) {
      this.confirmPasswordError = 'Confirm password must be identical as the password';
      isValid = false;
    }

    if (!isValid) return;

    if (this.name === 'SIGN UP') {
      this.signUp();
    } else {
      this.signIn();
    }
  }

  async signUp() {
    console.log('Signup:', this.username, this.password);
    const response = await fetch(environment.api_url+"/register", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({username: this.username, password: this.password}),
    })
    console.log(response);
  }

  async signIn() {
    console.log('Signin:', this.username, this.password);
    const response = await fetch(environment.api_url+"/auth", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({username: this.username, password: this.password}),
    })
    console.log(response);
  }

  resetErrors() {
    this.usernameError = '';
    this.passwordError = '';
    this.confirmPasswordError = '';
  }
}
