import { Component } from '@angular/core';
import {ButtonComponent} from '../../atoms/button/button.component';
import {FilterListComponent} from '../filter-list/filter-list.component';
import {Router} from '@angular/router';
import {environment} from '../../../environments/environment';
import {WebSocketService} from '../../../services/websocket.service';

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
  constructor(private router: Router, private webSocketService: WebSocketService) {}

  friends = [{username: "alyrow", id: "ae14c115-dcff-4e20-8108-bf27087765db"},{username: "Skynox", id: ""},{username:"Lemieldesdauphin", id: ""}]
  friends_username: string[] = this.friends.map(friend => friend.username);
  participants: string[] = [];
  filters: string[] = [];

  async startSession() {

    if (this.participants.length > 0) {
      const token = localStorage.getItem('token');
      if(token != null) {

        const responseCreationSession = await fetch(environment.api_url + "/session", {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
            'Authorization': `Bearer ${token}`
          },
          body: JSON.stringify({
            participants: this.participants.map((username) =>
              this.friends.filter((elem) => elem.username == username)[0].id),
            filters: this.filters
          }),
        })
        const sessionId = await responseCreationSession.json();

        this.webSocketService.joinSession(sessionId, token);
        await this.router.navigate(['/movies-swipe/lobby']);
      }
    } else {
      document.querySelector(".error-message")?.classList.add("visible");
    }
  }

  onFiltersChanged(filtersType: string, selectedItems: string[]) {
    if(filtersType == 'friends') {
      this.participants = selectedItems;
    } else if(filtersType == 'filters'){
      this.filters = selectedItems;
    }
  }

  protected readonly Object = Object;
}
