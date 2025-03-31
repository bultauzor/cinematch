import {Injectable} from '@angular/core';
import {ContentType} from "../models/api";
import {RecommendationParametersInput} from "../models/filters";
import {BehaviorSubject} from "rxjs";

@Injectable({
    providedIn: 'root'
})
export class FiltersService {
    private filter: RecommendationParametersInput = {
        users_input: [],
        not_seen_by: [],
        disable_content_type_filter: true,
        content_type: "Movie",
        disable_genre_filter: true,
        genres: []
    }
    filterSubject = new BehaviorSubject<RecommendationParametersInput>(this.filter);
    public filter$ = this.filterSubject.asObservable();

    constructor() {
    }

    forceUpdate() {
        this.filterSubject.next(this.filter)
    }

    setUsersInput(input: string[]) {
        this.filter.users_input = input
        this.filterSubject.next(this.filter)
    }

    setNotSeenBy(input: string[]) {
        this.filter.not_seen_by = input
        this.filterSubject.next(this.filter)
    }

    setContentType(input?: ContentType) {
        if (input == null) {
            this.filter.disable_content_type_filter = true
        } else {
            this.filter.disable_content_type_filter = false
            this.filter.content_type = input
        }
        this.filterSubject.next(this.filter)
    }

    setGenres(input: string[]) {
        if (input == null) {
            this.filter.disable_genre_filter = true
            this.filter.genres = []
        } else {
            this.filter.disable_genre_filter = false
            this.filter.genres = input
        }
        this.filterSubject.next(this.filter)
    }
}
