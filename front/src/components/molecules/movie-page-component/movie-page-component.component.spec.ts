import { ComponentFixture, TestBed } from '@angular/core/testing';

import { MoviePageComponentComponent } from './movie-page-component.component';

describe('MoviePageComponentComponent', () => {
  let component: MoviePageComponentComponent;
  let fixture: ComponentFixture<MoviePageComponentComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [MoviePageComponentComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(MoviePageComponentComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
