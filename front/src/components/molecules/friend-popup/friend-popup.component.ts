import {Component, EventEmitter, Output} from '@angular/core';
import {environment} from '../../../environments/environment';
import {ButtonComponent} from '../../atoms/button/button.component';
import {FriendRequestsPopupComponent} from '../friend-requests-popup/friend-requests-popup.component';
import {NgForOf, NgIf} from '@angular/common';

@Component({
  selector: 'app-friend-popup',
  imports: [
    ButtonComponent,
    FriendRequestsPopupComponent,
    NgIf,
    NgForOf
  ],
  templateUrl: './friend-popup.component.html',
  styleUrl: './friend-popup.component.css'
})
export class FriendPopupComponent {
  friends: Friend[] = [];
  showRequestPopup = false;
  @Output() closePopup = new EventEmitter<void>();

  async ngOnInit() {
    await this.loadFriends();
  }

  async loadFriends() {
    const token = localStorage.getItem('token');
    if (token) {
      const response = await fetch(environment.api_url + "/friends", {
        method: "GET",
        headers: {
          "Content-Type": "application/json",
          'Authorization': `Bearer ${token}`
        }
      });
      if (response.ok) {
        this.friends = await response.json();
      }
    }
  }

  openRequestPopup() {
    this.showRequestPopup = true;
  }

  async closeRequestPopup() {
    this.showRequestPopup = false;
    await this.loadFriends();
  }

  close(){
    console.log("close");
    this.closePopup.emit();
  }
}

type Friend = {
  user_id: string,
  friend_id: string,
   friend_username: string,
}
