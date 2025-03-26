import { Routes } from '@angular/router';
import {HomeComponent} from '../pages/home/home.component';
import {SigninComponent} from '../pages/signin/signin.component';
import {SignupComponent} from '../pages/signup/signup.component';
import {UserHomeComponent} from '../pages/user-home/user-home.component';
import {MoviesSwipeStartComponent} from '../pages/movies-swipe-start/movies-swipe-start.component';
import {MoviesSwipeSessionComponent} from '../pages/movies-swipe-session/movies-swipe-session.component';
import {MoviesSwipeResultComponent} from '../pages/movies-swipe-result/movies-swipe-result.component';

export const routes: Routes = [
  { path: '', component: HomeComponent },
  { path: 'signin', component: SigninComponent },
  { path: 'signup', component: SignupComponent },
  { path: 'home', component: UserHomeComponent },
  { path: 'movies-swipe/start', component: MoviesSwipeStartComponent },
  { path: "movies-swipe/session", component: MoviesSwipeSessionComponent},
  { path: "movies-swipe/result", component: MoviesSwipeResultComponent}
];
