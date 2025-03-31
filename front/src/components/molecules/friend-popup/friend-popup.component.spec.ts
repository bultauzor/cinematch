import { ComponentFixture, TestBed } from '@angular/core/testing';

import { FriendPopupComponent } from './friend-popup.component';

describe('FriendPopupComponent', () => {
  let component: FriendPopupComponent;
  let fixture: ComponentFixture<FriendPopupComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [FriendPopupComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(FriendPopupComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
