import { ComponentFixture, TestBed } from '@angular/core/testing';

import { FriendRequestsPopupComponent } from './friend-requests-popup.component';

describe('FriendRequestsPopupComponent', () => {
  let component: FriendRequestsPopupComponent;
  let fixture: ComponentFixture<FriendRequestsPopupComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [FriendRequestsPopupComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(FriendRequestsPopupComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
