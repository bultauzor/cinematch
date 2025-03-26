import { ComponentFixture, TestBed } from '@angular/core/testing';

import { MoviesSwipeResultComponentComponent } from './movies-swipe-result-component.component';

describe('MoviesSwipeResultComponentComponent', () => {
  let component: MoviesSwipeResultComponentComponent;
  let fixture: ComponentFixture<MoviesSwipeResultComponentComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [MoviesSwipeResultComponentComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(MoviesSwipeResultComponentComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
