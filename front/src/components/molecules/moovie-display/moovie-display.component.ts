import { CommonModule } from "@angular/common";
import { Component, OnInit } from "@angular/core";
import { SearchService } from "../../../services/search.service";
import { Content } from "../../../models/api";
import { MovieCardComponent } from "../../atoms/movie-card/movie-card.component";

@Component({
  imports: [
    MovieCardComponent,
    CommonModule,
  ],
  selector: "app-moovie-display",
  templateUrl: "./moovie-display.component.html",
  styleUrls: ["./moovie-display.component.css"],
})
export class MoovieDisplayComponent implements OnInit {
  movies: Content[] = [];

  constructor(private searchService: SearchService) {}

  ngOnInit(): void {
    this.searchService.searchResult$.subscribe((result: Content[]) => {
      this.movies = result;
    });
  }
}
