import {Component, EventEmitter, Input, Output} from '@angular/core';
import {NgIf} from '@angular/common';
import {ButtonComponent} from '../../atoms/button/button.component';
import {Router} from '@angular/router';
import {WebSocketService} from '../../../services/websocket.service';
import {environment} from '../../../environments/environment';

@Component({
  selector: 'app-notification-card',
  imports: [
    NgIf,
    ButtonComponent
  ],
  templateUrl: './notification-card.component.html',
  styleUrl: './notification-card.component.css'
})
export class NotificationCardComponent {
  @Input() type: "Session" | "Friend" = "Friend"
  @Input() user_username: string | null = null
  @Input() user_id: string | null = null
  @Input() session_id: string | null = null

  @Output() refreshParent: EventEmitter<void> = new EventEmitter<void>();

  constructor(private router: Router, private webSocketService: WebSocketService) {}

  async join_session(sessionId: string) {
    const token = localStorage.getItem('token');
    if(token != null) {
      this.webSocketService.joinSession(sessionId, token);
      await this.router.navigate(['/movies-swipe/lobby']);
    }
  }

  async accept_invitation(userId: string){
    console.log(userId)
    const token = localStorage.getItem('token');
    const accept_invitations_result = await fetch(environment.api_url + "/invitations/"+userId+"/accept", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        'Authorization': `Bearer ${token}`
      },
    })
    this.refreshParent.emit();
  }

  async refuse_invitation(userId: string){
    const token = localStorage.getItem('token');
    const refuse_invitations_result = await fetch(environment.api_url + "/invitations/"+userId+"/refuse", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        'Authorization': `Bearer ${token}`
      },
    })
    this.refreshParent.emit();
  }
}
