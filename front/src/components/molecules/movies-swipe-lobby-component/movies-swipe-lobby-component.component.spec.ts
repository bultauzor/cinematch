import { ComponentFixture, TestBed } from '@angular/core/testing';

import { MoviesSwipeLobbyComponentComponent } from './movies-swipe-lobby-component.component';

describe('MoviesSwipeLobbyComponentComponent', () => {
  let component: MoviesSwipeLobbyComponentComponent;
  let fixture: ComponentFixture<MoviesSwipeLobbyComponentComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [MoviesSwipeLobbyComponentComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(MoviesSwipeLobbyComponentComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
