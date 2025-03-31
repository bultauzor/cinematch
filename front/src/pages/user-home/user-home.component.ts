import { Component } from '@angular/core';
import {HomeFooterComponent} from '../../components/molecules/home-footer/home-footer.component';
import {HomeHeaderComponent} from '../../components/molecules/home-header/home-header.component';
import { MoovieDisplayComponent } from '../../components/molecules/moovie-display/moovie-display.component';
import {NotificationCardComponent} from '../../components/molecules/notification-card/notification-card.component';
import {NgForOf, NgIf} from '@angular/common';
import {environment} from '../../environments/environment';
import {ButtonComponent} from '../../components/atoms/button/button.component';
import {FriendPopupComponent} from '../../components/molecules/friend-popup/friend-popup.component';

@Component({
  selector: 'app-user-home',
  imports: [
    HomeFooterComponent,
    HomeHeaderComponent,
    MoovieDisplayComponent,
    NotificationCardComponent,
    NgForOf,
    ButtonComponent,
    NgIf,
    FriendPopupComponent
  ],
  templateUrl: './user-home.component.html',
  styleUrl: './user-home.component.css'
})
export class UserHomeComponent {
  friends_invitation: FriendRequest[] = [];
  session_invitation: SessionRequest[] = [];
  friend_popup: boolean = false;

  async ngOnInit(): Promise<void> {
    await this.request()
    setInterval(this.request, 30000)
  }

  async refreshComponent() {
    await this.request()
  }

  async request() {
    const token = localStorage.getItem('token');

    const session_invitations_result = await fetch(environment.api_url + "/session", {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
        'Authorization': `Bearer ${token}`
      },
    })
    this.session_invitation = await session_invitations_result.json();
    const friend_invitations_result = await fetch(environment.api_url + "/invitations", {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
        'Authorization': `Bearer ${token}`
      },
    })
    this.friends_invitation = await friend_invitations_result.json();
  }

  togglePopup(){
    console.log("aaaaaaa")
    this.friend_popup = !this.friend_popup;
  }
}



type SessionRequest = {
  owner_id: string,
  session_id: string,
  owner_username: string
}

type FriendRequest = {
  user_id: string,
  friend_id: string,
  user_username: string,
  user_avatar?: string,
}
