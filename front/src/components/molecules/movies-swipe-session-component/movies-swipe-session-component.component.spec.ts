import { ComponentFixture, TestBed } from '@angular/core/testing';

import { MoviesSwipeSessionComponentComponent } from './movies-swipe-session-component.component';

describe('MoviesSwipeSessionComponentComponent', () => {
  let component: MoviesSwipeSessionComponentComponent;
  let fixture: ComponentFixture<MoviesSwipeSessionComponentComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [MoviesSwipeSessionComponentComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(MoviesSwipeSessionComponentComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
