# Cinematch

Cinematch is a movie recommendation platform that suggests movies based on user preferences.
It is built with a Rust API, Python backend for authentication, an Angular frontend, and PostgreSQL for data storage.

The project is designed to be simple to set up using Docker Compose.
Project Structure

- Rust API: Provides the core functionality, including movie recommendations.
- Python Auth Backend: Handles user authentication.
- Angular Frontend: The user interface for interacting with the application.
- PostgreSQL: Used to store user data and movie-related information.

## Using docker compose

You need to edit the file `docker.env` and at least replace `TMDB_TOKEN` with
your [tmdb token](https://www.themoviedb.org/settings/api).

Then you can do the usual :

```shell
docker compose up
```

Two ports will be open:

- 8080 for the api
- 8000 for the frontend

The frontend will initiate requests to the api with the localhost domain, if you wish to change that, change the
`API_URL` variable that you can find in
`services.front.build.args`.
Note that you need to rebuild the front image with `docker compose build front`.

## How the recommendations work?

The recommendation system use two methods to compute the recommendations.

The first method is to retrieve recommended list of contents that the users liked (with a note > 0.5) from The Movie
Database.
It recurses trough two levels and compute a score based on the note that the user gave to the input content, the
recursion level and the position of the recommendation among others recommendations.

The second method is a bit less straightforward and is inspired from existing algorithms commonly called recommender
systems.
There are multiple categories among these systems, one is particulary interesting: it's collaborative filtering.
It relies on what other users like to automatically determines what can a user like.
You can find more at https://en.wikipedia.org/wiki/Collaborative_filtering
_Fun fact:_ Netflix's system is or was called cinematch.

If we dive in the details of the implementation of the second method, users rating are quantized from a float between 0
and 10 to a bit.
It permits us to create a bit vector that we use to determine the best similarities.
The [Jaccard](https://en.wikipedia.org/wiki/Jaccard_index) distance is used (it the most similar to a cosine distance).

We made the choice to put all the recommendation logic directly at the database level by using pgvector and advanced
postgres functionalities to:

- Have paging support for recommendation
- Minimize considerably the pressure between the api and the db
- Use advanced mechanism to cache recommendations

Thus the recommendation systems is extremely fast.
The only drawbacks are in the ingesting process especially for the first method where escaping rate limits are a things.
Cache mechanism may make the window between user actions and recommendation **relatively slow**: count around 10 minutes
for the recommendation to be provided (not computed) to the user.
