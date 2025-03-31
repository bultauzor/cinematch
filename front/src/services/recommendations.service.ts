import {Injectable, OnInit} from '@angular/core';
import {BehaviorSubject} from "rxjs";
import {Content} from "../models/api";
import {HttpClient, HttpHeaders} from "@angular/common/http";
import {FiltersService} from "./filters.service";
import {RecommendationParametersInput} from "../models/filters";
import {environment} from "../environments/environment";

@Injectable({
  providedIn: 'root'
})
export class RecommendationsService implements OnInit {
  recommendationResultSubject = new BehaviorSubject<Content[]>([]);
  public recommendationResult$ = this.recommendationResultSubject.asObservable();

  constructor(private http: HttpClient, private filterService: FiltersService) {
  }

  ngOnInit(): void {
    this.filterService.filter$.subscribe((result: RecommendationParametersInput) => {
      const apiUrl = environment.api_url + `/recommendation`;
      const token = localStorage.getItem('token');
      const headers = new HttpHeaders({
        'Authorization': `Bearer ${token}`
      });

      this.http.post<Content[]>(apiUrl, result, {headers}).subscribe(
        (response) => {
          console.log(response)
          this.recommendationResultSubject.next(response);
        },
      );
    });
  }

}
