import { ComponentFixture, TestBed } from '@angular/core/testing';

import { OrSeparatorComponent } from './or-separator.component';

describe('OrSeparatorComponent', () => {
  let component: OrSeparatorComponent;
  let fixture: ComponentFixture<OrSeparatorComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [OrSeparatorComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(OrSeparatorComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
