import {Component, NgZone} from '@angular/core';
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
  constructor(private router: Router, private webSocketService: WebSocketService, private ngZone: NgZone) {}

  friends: friend[] = []
  friends_username: string[] = [];
  participants: string[] = [];
  filters: string[] = [];

  async ngOnInit(): Promise<void> {
    const token = localStorage.getItem('token');
    if (token != null) {
      const responseGetFriends = await fetch(environment.api_url + "/friends", {
        method: "GET",
        headers: {
          "Content-Type": "application/json",
          'Authorization': `Bearer ${token}`
        },

      })
      this.ngZone.run(async () => {
        this.friends = await responseGetFriends.json();
        this.friends_username = this.friends.map(friend => friend.friend_username);
      });
    }
  }

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
              this.friends.filter((elem) => elem.friend_username == username)[0].friend_id),
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

type friend = {
  user_id: string,
  friend_id: string,
  friend_username: string
}
