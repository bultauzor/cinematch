import { Component, inject, Input } from "@angular/core";
import { Router } from "@angular/router";
import { ButtonComponent } from "../../atoms/button/button.component";
import { InputFormComponent } from "../../atoms/input-form/input-form.component";
import { NgIf } from "@angular/common";
import { environment } from "../../../environments/environment";
import { AuthToken , ApiError } from "../../../models/api"

@Component({
  selector: "app-form-auth",
  imports: [
    ButtonComponent,
    InputFormComponent,
    NgIf,
  ],
  templateUrl: "./form-auth.component.html",
  styleUrl: "./form-auth.component.css",
})
export class FormAuthComponent {
  router = inject(Router);

  @Input()
  name: "SIGN UP" | "SIGN IN" = "SIGN UP";

  username: string = "";
  password: string = "";
  confirmPassword: string = "";

  usernameError: string = "";
  passwordError: string = "";
  confirmPasswordError: string = "";

  onSubmit() {
    this.resetErrors();

    let isValid = true;

    if (!this.username.trim()) {
      this.usernameError = "Username is required";
      isValid = false;
    } else if (this.username.length < 2) {
      this.usernameError = "Username must be at least 2 characters";
      isValid = false;
    } else if (this.username.length > 32) {
      this.usernameError = "Username must be at most 32 characters";
      isValid = false;
    }

    if (!this.password.trim()) {
      this.passwordError = "Password is required";
      isValid = false;
    } else if (this.password.length < 4) {
      this.passwordError = "Password must be at least 4 characters";
      isValid = false;
    } else if (this.password.length > 128) {
      this.passwordError = "Password must be at most 128 characters";
      isValid = false;
    }

    if (this.name == "SIGN UP" && !this.confirmPassword.trim()) {
      this.confirmPasswordError = "Confirm password is required";
      isValid = false;
    } else if (
      this.name == "SIGN UP" && this.password != this.confirmPassword
    ) {
      this.confirmPasswordError =
        "Confirm password must be identical as the password";
      isValid = false;
    }

    if (!isValid) return;

    if (this.name === "SIGN UP") {
      this.signUp();
    } else {
      this.signIn();
    }
  }

  async handleAuthResponse(response: Response) {
    const json_response = await response.json();

    if (response.ok) {
      const parsed_json: AuthToken = json_response;
      localStorage.setItem("token", parsed_json.token);
      this.router.navigate(["/home"]);
    } else {
      const parsed_json: ApiError = json_response;
      let error = "Username or password is wrong";

      if (response.status == 500) {
        error = "Internal Server Error";
      } else if (
        response.status == 400 && parsed_json.error.includes("already exists")
      ) {
        error = JSON.parse(parsed_json.error).detail;
      }

      this.usernameError = this.passwordError = error;
    }
  }

  async signUp() {
    const response = await fetch(environment.api_url + "/register", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        username: this.username,
        password: this.password,
      }),
    });

    await this.handleAuthResponse(response);
  }

  async signIn() {
    const response = await fetch(environment.api_url + "/auth", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        username: this.username,
        password: this.password,
      }),
    });
    await this.handleAuthResponse(response);
  }

  resetErrors() {
    this.usernameError = "";
    this.passwordError = "";
    this.confirmPasswordError = "";
  }
}
