export type AuthToken = {
  token: string;
};

export type ApiError = {
  error: string;
  error_code: string;
};

export type ContentType = "Movie" | "Show"

export type Content = {
    content_id: string,
    provider_id: string,
    updated_at: string,
    content_type: ContentType,
    title: string,
    overview: string,
    poster?: string,
    release_date: string,
    genres: string[],
    backdrop?: string,
    vote_average: number
}
