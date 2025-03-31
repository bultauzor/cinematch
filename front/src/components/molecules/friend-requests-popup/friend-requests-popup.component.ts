import {Component, EventEmitter, Output} from '@angular/core';
import {FormsModule} from '@angular/forms';
import {ButtonComponent} from '../../atoms/button/button.component';
import {NgIf} from '@angular/common';
import {environment} from '../../../environments/environment';

@Component({
  selector: 'app-friend-requests-popup',
  imports: [
    FormsModule,
    ButtonComponent,
    NgIf
  ],
  templateUrl: './friend-requests-popup.component.html',
  styleUrl: './friend-requests-popup.component.css'
})
export class FriendRequestsPopupComponent {
  username: string = '';

  @Output() closePopup = new EventEmitter<void>();
  error = false;

  async sendFriendRequest() {
    if (this.username) {
      const token = localStorage.getItem('token');
      if(token != null) {
        const response = await fetch(environment.api_url + "/friends", {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
            'Authorization': `Bearer ${token}`
          },
          body: JSON.stringify(this.username),
        })
        if(response.ok){
          this.closePopup.emit();
        } else {
          this.error = true;
        }
      }
    }
  }

  close() {
    this.closePopup.emit();
  }
}
