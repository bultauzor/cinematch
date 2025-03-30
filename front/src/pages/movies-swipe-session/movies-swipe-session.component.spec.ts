import { ComponentFixture, TestBed } from '@angular/core/testing';

import { MoviesSwipeSessionComponent } from './movies-swipe-session.component';

describe('MoviesSwipeSessionComponent', () => {
  let component: MoviesSwipeSessionComponent;
  let fixture: ComponentFixture<MoviesSwipeSessionComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [MoviesSwipeSessionComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(MoviesSwipeSessionComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
