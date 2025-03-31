import {CommonModule} from "@angular/common";
import {Component, OnInit} from "@angular/core";
import {SearchService} from "../../../services/search.service";
import {Content} from "../../../models/api";
import {MovieCardComponent} from "../../atoms/movie-card/movie-card.component";
import {RecommendationsService} from "../../../services/recommendations.service";
import {Router} from '@angular/router';

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
  moviesSearch: Content[] = [];
  moviesRecommendation: Content[] = [];
  searchMode = false

  constructor(private searchService: SearchService, private recommendationsService: RecommendationsService, private router: Router) {
  }

  goToMovieDetails(content: any) {
    this.router.navigate(['page', content]);
  }

  ngOnInit(): void {
    this.recommendationsService.recommendationResult$.subscribe((result: Content[]) => {
      this.moviesRecommendation = result;
    })
    this.recommendationsService.ngOnInit()
    this.searchService.searchResult$.subscribe((result: Content[] | null) => {
      if (result == null) {
        this.searchMode = false
      } else {
        this.moviesSearch = result;
        this.searchMode = true;
      }
    });
  }
}
