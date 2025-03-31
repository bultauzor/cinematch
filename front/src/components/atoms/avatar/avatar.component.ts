import { Component, ElementRef, inject, Input, ViewChild } from "@angular/core";
import { Router, RouterLink } from "@angular/router";
import { NgIf, NgOptimizedImage } from "@angular/common";
import { environment } from "../../../environments/environment";

type ButtonOption = "logout";

@Component({
  selector: "app-avatar",
  imports: [
    RouterLink,
    NgOptimizedImage,
    NgIf,
  ],
  templateUrl: "./avatar.component.html",
  styleUrl: "./avatar.component.css",
})
export class AvatarComponent {
  router = inject(Router);

  @Input()
  image_link: string = "default_avatar.png";
  @Input()
  router_link: string = "";
  @Input()
  size: number = 70;
  menuVisible = false;

  @ViewChild("fileInput", { static: false })
  fileInput!: ElementRef;

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
    }).then(_ => this.updateAvartar());
  }

  updateAvartar() {
    const token = localStorage.getItem("token");
    fetch(environment.api_url + "/avatar", {
      method: "GET",
      headers: {
        "Authorization": `Bearer ${token}`,
        "Content-Type": "application/json",
      },
    }).then(response => {
      if (response.ok) {
        response.text().then(res => this.image_link = res)
      }
    });
  }

  onButtonClick(option: ButtonOption) {
    this.menuVisible = false;

    switch (option) {
      case "logout":
        localStorage.removeItem("token");
        this.router.navigate(["/"]);
    }
  }
}
