import {Component, NgZone, OnInit} from "@angular/core";
import {HomeFooterComponent} from "../../components/molecules/home-footer/home-footer.component";
import {HomeHeaderComponent} from "../../components/molecules/home-header/home-header.component";
import {MoovieDisplayComponent} from "../../components/molecules/moovie-display/moovie-display.component";
import {NotificationCardComponent} from "../../components/molecules/notification-card/notification-card.component";
import {NgForOf} from "@angular/common";
import {environment} from "../../environments/environment";
import {FilterListComponent} from "../../components/molecules/filter-list/filter-list.component";
import {ContentType} from "../../models/api";
import {HttpClient, HttpHeaders} from "@angular/common/http";
import {FiltersService} from "../../services/filters.service";
import {FiltersService} from "../../services/filters.service";
import {Router} from '@angular/router';

@Component({
  selector: "app-user-home",
  imports: [
    HomeFooterComponent,
    HomeHeaderComponent,
    MoovieDisplayComponent,
    NotificationCardComponent,
    NgForOf,
    FilterListComponent
  ],
  templateUrl: "./user-home.component.html",
  styleUrl: "./user-home.component.css",
})
export class UserHomeComponent implements OnInit {
  genres: string[] = []
  friends: friend[] = []
  friends_username: string[] = [];

  friends_invitation: FriendRequest[] = [];
  session_invitation: SessionRequest[] = [];

  async ngOnInit(): Promise<void> {
    await this.request();
    setInterval(this.request, 30000);
    await this.request()
    setInterval(this.request, 30000)

    const apiUrl = environment.api_url + `/content/genres`;
    const token = localStorage.getItem('token');
    const headers = new HttpHeaders({
      'Authorization': `Bearer ${token}`
    });

    this.http.get<string[]>(apiUrl, {headers}).subscribe(
      (response) => {
        this.genres = response
      },
    );

    if (token != null) {
      const responseGetFriends = await fetch(environment.api_url + "/friends", {
        method: "GET",
        headers: {
          "Content-Type": "application/json",
          'Authorization': `Bearer ${token}`
        },

      })
      await this.ngZone.run(async () => {
        this.friends = await responseGetFriends.json();
        this.friends_username = this.friends.map(friend => friend.friend_username);
      });
    }
  }

  constructor(private http: HttpClient, private filtersService: FiltersService, private ngZone: NgZone) {
  }

  onFiltersChanged(filtersType: string, selectedItems: string[]) {
    switch (filtersType) {
      case 'genres':
        this.filtersService.setGenres(selectedItems);
        break
      case 'type':
        if (selectedItems.length == 0 || selectedItems.length == 2)
          this.filtersService.setContentType(null);
        else this.filtersService.setContentType(selectedItems[0] as ContentType)
        break
      case 'friends':
        let friends = selectedItems.map((username: string) =>
          this.friends.filter((elem) => elem.friend_username == username)[0].friend_id)
        this.filtersService.setUsersInput(friends)
        break
      case 'not_seen':
        let ns = selectedItems.map((username: string) =>
          this.friends.filter((elem) => elem.friend_username == username)[0].friend_id)
        this.filtersService.setUsersInput(ns)
        break
    }
  }

  async refreshComponent() {
    await this.request();
  }

  async request() {
    const token = localStorage.getItem("token");

    const session_invitations_result = await fetch(
      environment.api_url + "/session",
      {
        method: "GET",
        headers: {
          "Content-Type": "application/json",
          "Authorization": `Bearer ${token}`,
        },
      },
    );
    this.session_invitation = await session_invitations_result.json();
    const friend_invitations_result = await fetch(
      environment.api_url + "/invitations",
      {
        method: "GET",
        headers: {
          "Content-Type": "application/json",
          "Authorization": `Bearer ${token}`,
        },
      },
    );
    this.friends_invitation = await friend_invitations_result.json();
  }


}

type SessionRequest = {
  owner_id: string;
  session_id: string;
  owner_username: string;
};

type FriendRequest = {
  user_id: string;
  friend_id: string;
  user_username: string;
  user_avatar?: string;
};

type friend = {
  user_id: string,
  friend_id: string,
  friend_username: string
}
