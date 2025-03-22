
declare interface Env {
  readonly NODE_ENV: string;
  API_URL: string;
  [key: string]: any;
}

declare interface ImportMeta {
  readonly env: Env;
}
