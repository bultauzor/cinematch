import { ComponentFixture, TestBed } from '@angular/core/testing';

import { MoovieDisplayComponent } from './moovie-display.component';

describe('MoovieDisplayComponent', () => {
  let component: MoovieDisplayComponent;
  let fixture: ComponentFixture<MoovieDisplayComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [MoovieDisplayComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(MoovieDisplayComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
