import {
  Component,
  ElementRef,
  inject,
  Input,
  OnInit,
  ViewChild,
} from "@angular/core";
import { Router, RouterLink } from "@angular/router";
import { NgIf, NgOptimizedImage } from "@angular/common";
import { FriendPopupComponent } from "../../molecules/friend-popup/friend-popup.component";
import { environment } from "../../../environments/environment";

@Component({
  selector: "app-avatar",
  imports: [
    RouterLink,
    NgOptimizedImage,
    FriendPopupComponent,
    NgIf,
  ],
  templateUrl: "./avatar.component.html",
  styleUrl: "./avatar.component.css",
})
export class AvatarComponent implements OnInit {
  router = inject(Router);

  @Input()
  image_link: string = "default_avatar.png";
  @Input()
  router_link: string = "";
  @Input()
  size: number = 70;
  menuVisible = false;
  friend_popup: boolean = false;

  @ViewChild("fileInput", { static: false })
  fileInput!: ElementRef;

  ngOnInit(): void {
    this.updateAvartar();
  }

  toggleMenu() {
    this.menuVisible = !this.menuVisible;
  }

  triggerImageUpload() {
    this.fileInput.nativeElement.click();
  }

  handleFileInput(event: any) {
    const file = event.target!.files[0];
    if (file) {
      this.convertToBase64(file);
    }
  }

  convertToBase64(file: File) {
    const reader = new FileReader();
    reader.onload = (e) => {
      const base64Image: string = e.target!.result as string;

      this.uploadImage(base64Image);
    };
    reader.readAsDataURL(file);
  }

  uploadImage(base64Image: string) {
    const data = {
      image: base64Image,
    };

    const token = localStorage.getItem("token");
    fetch(environment.api_url + "/avatar", {
      method: "POST",
      headers: {
        "Authorization": `Bearer ${token}`,
        "Content-Type": "application/json",
      },
      body: JSON.stringify(data),
    }).then((_) => this.updateAvartar());
  }

  updateAvartar() {
    const token = localStorage.getItem("token");
    fetch(environment.api_url + "/avatar", {
      method: "GET",
      headers: {
        "Authorization": `Bearer ${token}`,
        "Content-Type": "application/json",
      },
    }).then((response) => {
      if (response.ok) {
        response.text().then((res) => this.image_link = res);
      }
    });
  }

  toggleFriendPopup() {
    this.friend_popup = !this.friend_popup;
  }

  logOut() {
    this.menuVisible = false;

    localStorage.removeItem("token");
    this.router.navigate(["/"]);
  }
}
