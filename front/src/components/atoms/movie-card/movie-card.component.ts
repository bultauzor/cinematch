import {Component, EventEmitter, Input, Output} from '@angular/core';
import {NgClass} from '@angular/common';
import {HAMMER_GESTURE_CONFIG, HammerGestureConfig} from '@angular/platform-browser';

@Component({
  selector: 'app-movie-card',
  imports: [
    NgClass
  ],
  providers: [
    {
      provide: HAMMER_GESTURE_CONFIG,
      useClass: HammerGestureConfig
    }
  ],
  templateUrl: './movie-card.component.html',
  styleUrl: './movie-card.component.css'
})
export class MovieCardComponent {
  @Input() title: string = "";
  @Input() swipeState: string = '';
  @Input() cardPosition: '' | 'absolute' = '';
  @Input() size: 'small' | 'big' = 'small';
  @Output() swipe = new EventEmitter<string>();


  onSwipe(event: any) {
    if (event.direction === 2) {  // 2 correspond à swipe à gauche
      this.swipe.emit('left');
    } else if (event.direction === 4) {  // 4 correspond à swipe à droite
      this.swipe.emit('right');
    }
  }
}
