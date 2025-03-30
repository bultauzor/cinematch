import { ComponentFixture, TestBed } from '@angular/core/testing';

import { MoviesSwipeStartComponent } from './movies-swipe-start.component';

describe('MoviesSwipeStartComponent', () => {
  let component: MoviesSwipeStartComponent;
  let fixture: ComponentFixture<MoviesSwipeStartComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [MoviesSwipeStartComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(MoviesSwipeStartComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
