import { Component } from '@angular/core';
import {ButtonComponent} from '../../atoms/button/button.component';
import {FilterListComponent} from '../filter-list/filter-list.component';
import {Router} from '@angular/router';

@Component({
  selector: 'app-movies-swipe-start-component',
  imports: [
    ButtonComponent,
    FilterListComponent
  ],
  templateUrl: './movies-swipe-start-component.component.html',
  styleUrl: './movies-swipe-start-component.component.css'
})
export class MoviesSwipeStartComponentComponent {
  constructor(private router: Router) {}

  friends: string[] = [];
  filters: string[] = [];

  startSession(){
    console.log(this.friends, this.filters)
    if(this.friends.length > 0) {
      this.router.navigate(['/movies-swipe/session']);
    } else {
      document.querySelector(".error-message")?.classList.add("visible");
    }
  }

  onFiltersChanged(filtersType: string, selectedItems: string[]) {
    if(filtersType == 'friends') {
      this.friends = selectedItems;
    } else if(filtersType == 'filters'){
      this.filters = selectedItems;
    }
  }
}
