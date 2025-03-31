import {ContentType} from "./api";

export type RecommendationParametersInput = {
  users_input: string[],
  not_seen_by: string[],
  disable_content_type_filter: boolean,
  content_type: ContentType,
  disable_genre_filter: boolean,
  genres: string[],
}