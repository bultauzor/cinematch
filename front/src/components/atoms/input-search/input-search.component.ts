import { Component, ElementRef, HostListener, ViewChild } from "@angular/core";
import { HttpClientModule } from "@angular/common/http";
import { FormsModule } from "@angular/forms";
import { SearchService } from "../../../services/search.service";

@Component({
  selector: "app-input-search",
  imports: [FormsModule, HttpClientModule],
  templateUrl: "./input-search.component.html",
  styleUrl: "./input-search.component.css",
})
export class InputSearchComponent {
  query: string = "";

  @ViewChild("search")
  searchElement: ElementRef = {} as ElementRef;

  constructor(private searchService: SearchService) {}

  @HostListener("document:keydown", ["$event"])
  onKeydown(event: KeyboardEvent): void {
    if (event.ctrlKey && event.key === "k") {
      event.preventDefault();
      event.stopPropagation();
      this.searchElement.nativeElement.focus();
    }
  }

  onEnter(): void {
    this.searchService.searchMovies(this.query);
  }
}
