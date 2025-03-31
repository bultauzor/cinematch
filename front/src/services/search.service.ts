import {Injectable} from '@angular/core';
import {HttpClient, HttpHeaders} from '@angular/common/http';
import {BehaviorSubject} from 'rxjs';
import {environment} from '../environments/environment';
import {Content} from '../models/api'

@Injectable({
  providedIn: 'root'
})
export class SearchService {
  searchResultSubject = new BehaviorSubject<Content[] | null>(null);
  public searchResult$ = this.searchResultSubject.asObservable();

  constructor(private http: HttpClient) {
  }

  searchMovies(query: string): void {
    if (query === "") {
      this.searchResultSubject.next(null);
      return;
    }

    const apiUrl = environment.api_url + `/search?query=${query}`;
    const token = localStorage.getItem('token');
    const headers = new HttpHeaders({
      'Authorization': `Bearer ${token}`
    });

    this.http.get<Content[]>(apiUrl, {headers}).subscribe(
      (response) => {
        this.searchResultSubject.next(response);
      },
    );
  }
}
