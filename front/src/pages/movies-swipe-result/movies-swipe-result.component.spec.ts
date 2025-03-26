import { ComponentFixture, TestBed } from '@angular/core/testing';

import { MoviesSwipeResultComponent } from './movies-swipe-result.component';

describe('MoviesSwipeResultComponent', () => {
  let component: MoviesSwipeResultComponent;
  let fixture: ComponentFixture<MoviesSwipeResultComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [MoviesSwipeResultComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(MoviesSwipeResultComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
