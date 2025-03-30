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
  grade?: number; // Optionnel, car il peut être null
}

export interface ContentView {
  content_id: string; // UUID sous forme de string
  content_type: string; // Enum possible si défini côté backend
  title: string;
  overview: string;
  poster?: string; // Optionnel
  release_date?: string; // Stocké sous forme de string (ISO 8601) pour éviter les problèmes de Date
  genres: string[]; // Liste de genres sous forme de tableau de chaînes de caractères
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
  title: string | undefined;
  contentType: ContentType | undefined;
  description: string | undefined;
  poster: string | undefined;
  release_date: Date | undefined;
  // genres: string[] | undefined;
  grade: number | undefined;
  seen: boolean | undefined;
  userRating: number = 4.0;
  sliderValue: number = 8; // Valeur initiale (4 * 2 car chaque pas de 0.5 correspond à 1 sur le slider)
  movieData?: ContentView;
  genres: string[] = ['Action', 'Adventure', 'Crime', 'Drama', 'Sci-Fi'];


  constructor(private route: ActivatedRoute, private router: Router) {}

  ngOnInit() {
    this.route.params.subscribe(async (params) => {
      this.movieID = params['id'];
      console.log('Test ID : ', this.movieID);
      this.contentType = ContentType.Movie


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
          // await this.router.navigate([""]);
        }
      }
    });
  }

  updateRating(): void {
    this.userRating = this.sliderValue / 2;
  }

}



