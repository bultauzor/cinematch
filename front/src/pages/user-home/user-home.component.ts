import { Component } from "@angular/core";
import { HomeFooterComponent } from "../../components/molecules/home-footer/home-footer.component";
import { HomeHeaderComponent } from "../../components/molecules/home-header/home-header.component";
import { MoovieDisplayComponent } from "../../components/molecules/moovie-display/moovie-display.component";
import { NotificationCardComponent } from "../../components/molecules/notification-card/notification-card.component";
import { NgForOf } from "@angular/common";
import { environment } from "../../environments/environment";
import {FilterListComponent} from "../../components/molecules/filter-list/filter-list.component";
import {ContentType} from "../../models/api";

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
export class UserHomeComponent {
  friends_invitation: FriendRequest[] = [];
  session_invitation: SessionRequest[] = [];

  async ngOnInit(): Promise<void> {
    await this.request();
    setInterval(this.request, 30000);
  }

  constructor(private filtersService: FiltersService) {
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
