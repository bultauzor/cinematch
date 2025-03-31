import {Component, OnInit} from '@angular/core';
import {ActivatedRoute, Router, RouterLink} from '@angular/router';
import {environment} from '../../../environments/environment';
import {FormsModule} from '@angular/forms';
import {ButtonComponent} from '../../atoms/button/button.component';
import {Content} from "../../../models/api";



export interface SeenContent {
  content: Content;
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
  movieID: string | null | undefined;
  content: any;
  userRating: number = 5.0;
  isSeen: boolean = false;
  sliderValue: number = 10;
  genres: string[] = ['Action', 'Adventure', 'Crime', 'Drama', 'Sci-Fi'];


  constructor(private route: ActivatedRoute, private router: Router) {
    const navigation = this.router.getCurrentNavigation();
    this.content = navigation?.extras.state?.['content'];
    this.route.params.subscribe(async (params) => {
      this.movieID = params['id'];
      console.log('Test ID : ', this.movieID);
    });

  }

  async ngOnInit() {

    console.log(this.content)

    if (this.content == undefined) {
      if (this.movieID) {
        try {
          const contentResponse = await fetch(environment.api_url + "/content/" + this.movieID, {
            method: "GET",
            headers: {
              "Content-Type": "application/json",
            }
          });

          if (!contentResponse.ok) {
            throw new Error("Erreur lors de la récupération du film");
          }

          this.content = await contentResponse.json();
          if (this.content) {
            console.log("Données du film :", this.content);
            console.log(`Titre: ${this.content.title}`);
            console.log(`Résumé: ${this.content.overview}`);
            console.log(`Genres: ${this.content.genres.join(", ")}`);
          }

        } catch (error) {
          console.error("Erreur :", error);
          alert("OSKOUR MAUVAIS")
          // await this.router.navigate([""]);
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
          const seenMovie = seenContents.find(content => content.content.content_id === this.movieID);

          if (seenMovie) {

            if (seenMovie) {
              this.isSeen = true;
              if (seenMovie.grade !== undefined) {
                this.sliderValue = seenMovie.grade * 2;
                this.userRating = seenMovie.grade;
              }
            }
            console.log(`Le film a été vu. Note : ${seenMovie.grade ?? "Aucune note"}`);
          } else {
            this.isSeen = false;
            console.log("Le film n'a pas été regardé par l'utilisateur.");
          }

        } catch (error) {
          console.error("Erreur :", error);
          alert("TEST")
        }
      }
    }
  }

  async submitGrade() {
    if (!this.movieID) return;


    const grade = this.userRating;
    const payload = {
      content_id: this.movieID,
      grade: grade
    };

    if(this.isSeen) {


      try {
        const response = await fetch(environment.api_url + "/seen/me", {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify(payload)
        });

        if (!response.ok) {
          throw new Error("Erreur lors de l'enregistrement de la note");
        }

        console.log("Note envoyée avec succès :", grade);
        alert("Note soumise avec succès !");
      } catch (error) {
        console.error("Erreur :", error);
        alert("Erreur lors de l'envoi de la note");
      }
    } else {

      try {
        const response = await fetch(environment.api_url + "/seen/me/" + this.movieID + "/grade", {
          method: "PATCH",
          headers: {
            "Content-Type": "application/json",
          },
        });

        if (!response.ok) {
          throw new Error("Erreur lors de l'enregistrement de la note");
        }

        console.log("Note envoyée avec succès :", grade);
        alert("Note soumise avec succès !");
      } catch (error) {
        console.error("Erreur :", error);
        alert("Erreur lors de l'envoi de la note");
      }

    }
  }


  updateRating(): void {
    this.userRating = this.sliderValue / 2;
  }

}



