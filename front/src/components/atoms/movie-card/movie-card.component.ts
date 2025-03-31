import {Component, EventEmitter, Input, Output} from '@angular/core';
import {NgClass} from '@angular/common';
import {HAMMER_GESTURE_CONFIG, HammerGestureConfig} from '@angular/platform-browser';
import {Content} from '../../../models/api';
import {Router} from '@angular/router';

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
  @Input() posterUrl: string = "https://www.themoviedb.org/t/p/original/8tDyUTNsV5ZiXtBJYU0bFbSYxEj.jpg";
  @Input() title: string = "";
  @Input() rating: string = "5";
  @Input() swipeState: string = '';
  @Input() cardPosition: '' | 'absolute' = '';
  @Input() size: 'small' | 'big' = 'small';
  @Output() swipe = new EventEmitter<string>();
  @Input() content?: Content;

  constructor(private router: Router) {}

  goToContentPage(){
    this.router.navigate(['/page', this.content?.content_id], {
      state: { contentview: this.content }
    });
  }


  onSwipe(event: any) {
    if (event.direction === 2) {  // 2 correspond à swipe à gauche
      this.swipe.emit('left');
    } else if (event.direction === 4) {  // 4 correspond à swipe à droite
      this.swipe.emit('right');
    }
  }
}
