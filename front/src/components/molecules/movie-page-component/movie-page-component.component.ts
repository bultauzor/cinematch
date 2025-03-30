import {Component, OnInit} from '@angular/core';
import {ActivatedRoute, Router, RouterLink} from '@angular/router';
import {environment} from '../../../environments/environment';
import {FormsModule} from '@angular/forms';
import {ButtonComponent} from '../../atoms/button/button.component';

enum ContentType {
  Movie = "Movie",
  Show = "TV Show"
}

export interface SeenContent {
  content: ContentView;
  grade?: number;
}





export interface ContentView {
  content_id: string;
  content_type: ContentType;
  title: string;
  overview: string;
  poster?: string;
  release_date?: string;
  genres: string[];
  grade?: number;
}


@Component({
  selector: 'app-movie-page-component',
  imports: [
    RouterLink,
    FormsModule,
    ButtonComponent
  ],
  templateUrl: './movie-page-component.component.html',
  standalone: true,
  styleUrl: './movie-page-component.component.css'
})
export class MoviePageComponentComponent implements OnInit {
  movieID: string | undefined;
  movieData?: ContentView;
  grade: number | undefined;
  seen: boolean | undefined;
  userRating: number = 5.0;
  sliderValue: number = 10;
  genres: string[] = ['Action', 'Adventure', 'Crime', 'Drama', 'Sci-Fi'];


  constructor(private route: ActivatedRoute, private router: Router) {}

  ngOnInit() {
    this.route.params.subscribe(async (params) => {
      this.movieID = params['id'];
      console.log('Test ID : ', this.movieID);


      if (this.movieID) {
        try {
          const contentResponse = await fetch(environment.api_url + "/movie/" + this.movieID, {
            method: "GET",
            headers: {
              "Content-Type": "application/json",
            }
          });

          if (!contentResponse.ok) {
            throw new Error("Erreur lors de la récupération du film");
          }

          this.movieData = await contentResponse.json();
          if(this.movieData) {
            console.log("Données du film :", this.movieData);
            console.log(`Titre: ${this.movieData.title}`);
            console.log(`Résumé: ${this.movieData.overview}`);
            console.log(`Genres: ${this.movieData.genres.join(", ")}`);
          }

          try {
            const seenResponse = await fetch(environment.api_url + "/seen/me/" + this.movieID, {
              method: "GET",
              headers: {
                "Content-Type": "application/json",
              }
            });

            if (!seenResponse.ok) {
              throw new Error("Erreur lors de la récupération du film");
            }

            const seenContents: SeenContent[] = await seenResponse.json();

            // Vérifier si le film est dans la liste et récupérer sa note
            const seenMovie = seenContents.find(content => content.content.content_id === this.movieID);

            if (seenMovie) {
              console.log(`Le film a été vu. Note : ${seenMovie.grade ?? "Aucune note"}`);
            } else {
              console.log("Le film n'a pas été regardé par l'utilisateur.");
            }

          } catch (error) {
            console.error("Erreur :", error);
          }


        } catch (error) {
          console.error("Erreur :", error);
          alert("OSKOUR MAUVAIS")
          // await this.router.navigate([""]);
        }
      }
    });
  }

  updateRating(): void {
    this.userRating = this.sliderValue / 2;
  }

}



