import { ReactiveFormsModule } from '@angular/forms';
import {NgModule} from '@angular/core';
import {BrowserModule, HammerModule} from '@angular/platform-browser';

@NgModule({
  imports: [
    ReactiveFormsModule,
    BrowserModule,
    HammerModule
  ],
})
export class AppModule { }
