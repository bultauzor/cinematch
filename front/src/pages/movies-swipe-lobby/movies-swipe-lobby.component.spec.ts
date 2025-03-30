import { ComponentFixture, TestBed } from '@angular/core/testing';

import { MoviesSwipeLobbyComponent } from './movies-swipe-lobby.component';

describe('MoviesSwipeLobbyComponent', () => {
  let component: MoviesSwipeLobbyComponent;
  let fixture: ComponentFixture<MoviesSwipeLobbyComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [MoviesSwipeLobbyComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(MoviesSwipeLobbyComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
