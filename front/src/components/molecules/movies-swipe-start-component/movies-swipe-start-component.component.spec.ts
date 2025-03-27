import { ComponentFixture, TestBed } from '@angular/core/testing';

import { MoviesSwipeStartComponentComponent } from './movies-swipe-start-component.component';

describe('MoviesSwipeConfigComponent', () => {
  let component: MoviesSwipeStartComponentComponent;
  let fixture: ComponentFixture<MoviesSwipeStartComponentComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [MoviesSwipeStartComponentComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(MoviesSwipeStartComponentComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
