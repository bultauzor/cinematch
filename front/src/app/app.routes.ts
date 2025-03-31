import { Routes } from "@angular/router";
import { HomeComponent } from "../pages/home/home.component";
import { SigninComponent } from "../pages/signin/signin.component";
import { SignupComponent } from "../pages/signup/signup.component";
import { UserHomeComponent } from "../pages/user-home/user-home.component";
import { MoviesSwipeStartComponent } from "../pages/movies-swipe-start/movies-swipe-start.component";
import { MoviesSwipeSessionComponent } from "../pages/movies-swipe-session/movies-swipe-session.component";
import { MoviesSwipeResultComponent } from "../pages/movies-swipe-result/movies-swipe-result.component";
import { authGuard } from "../guards/auth.guard";
import { unauthGuard } from "../guards/unauth.guard";
import {MoviesSwipeLobbyComponent} from '../pages/movies-swipe-lobby/movies-swipe-lobby.component';
import {MoviePageComponent} from '../pages/movie-page/movie-page.component';




export const routes: Routes = [
  { path: "", component: HomeComponent , canActivate: [unauthGuard] },
  { path: "signin", component: SigninComponent , canActivate: [unauthGuard]},
  { path: "signup", component: SignupComponent , canActivate: [unauthGuard]},
  { path: "home", component: UserHomeComponent, canActivate: [authGuard]},
  {
    path: "movies-swipe/start",
    component: MoviesSwipeStartComponent,
    canActivate: [authGuard],
  },
  {
    path: "movies-swipe/lobby",
    component: MoviesSwipeLobbyComponent,
    canActivate: [authGuard],
  },
  {
    path: "movies-swipe/session",
    component: MoviesSwipeSessionComponent,
    canActivate: [authGuard],
  },
  {
    path: "movies-swipe/result",
    component: MoviesSwipeResultComponent,
    canActivate: [authGuard],
  },
  {
    path: "page/:id",
    component:MoviePageComponent,
    canActivate: [authGuard]
  }
];
