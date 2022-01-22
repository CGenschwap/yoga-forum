# You Only Get An API

[You Only Get an API](https://youonlygetanapi.com/public/index.html) (YOGA) is a self-selecting community of builders, and is an API-only forum. This repository holds all of the source-code for the forum backend.

## Development

1. Use `$ nix-shell` to load the necessary development dependencies via Nix.
2. Build and run the dev server using `$ just run-dev`
3. Set up the initial entities with `$ ./scripts/setup.sh`
4. Run integration tests using `$ just test`

You can run `just -l` to list out common commands that are wrapped up with Just.

## Architecture

### High Level

The high-level architecture is about as simple as it gets. We are currently using fly.io for hosting, tls-termination, rev-proxy and static file hosting. The request is then passed to our YOGA backend (this repository) which handles communication with SQLite which holds the data. Currently there is not separate caching layer, and all caching is internal to the YOGA backend.

```
                  ________________          ______________         ________
[request] -----> / Proxy/TLS/etc. \ -----> / YOGA Backend \ ----> | SQLite |
          <----- \____(fly.io)____/ <----- \______________/ <---- |________|
```

### Code

The repository holds a few different crates, and will likely expand to more as is deemed reasonable.

```
(git clone)
|-- yoga-forum/     // Main codebase for the backend
|                   // See yoga-forum/README.md
|                   // for more details on code-structure
|
|-- yoga-actor/     // Actor-based testing for random
|                   // stress-testing of the system
|
|-- templatize-me/  // Super simple binary for generating
|                   // HTML for static elements of the site
|
|-- templates/      // Templates processed by `templatize-me`
|
|-- public/         // Static files, either generated or just
|                   // present
|
`-- *               // Misc. other files
```


## Data Model

The data model behind the YOGA API should hopefully be as expected. At the core is a "user" who has the ability to create "stories" and "comments" on those stories. A comment belongs to a given story, and may have a "parent" comment which also belongs to that story. Finally, we separate the password table from the user table.

| Relationship       | Relationship Type |
|:-------------------|:------------------|
|Users <-> Stories   | One-to-Many       |
|Users <-> Comments  | One-to-Many       |
|Stories <-> Comments| One-to-Many       |
|Users <-> Passwords | One-to-One        |

```
 [ Stories ] --> [ Users ] 
      ^       /      |
      |      /       |
      |     /        v
[ Comments ]    [ Passwords ]
```

## Testing Philosophy

We attempt to keep the testing as minimal yet comprehensive as possible. The general philosophy towards testing is that writing mocks is error-prone and time-consuming, and excessive unit tests just make it harder to change things. Given this, we have primarily integration tests.

The integration tests are meant to be run with a live server _already running_ in a separate process. Additionally there is no need to clean out your database first, as the tests don't make assumptions of a clean database. There are plenty of trade-offs with this testing design philosophy, but for now this is what we're sticking with.

- Integration tests are primarily located in the `yoga-forum/tests/` directory. The goal here is to capture edge-cases as well-defined test-scenarios.
- There are some unit tests scattered throughout the codebase where they make sense.
- There are additionally "actor" tests which "simulate" a user of the API which are run via the `yoga-actor` crate. This is a bit of a WIP and only simulates a handful of actions. The goal here is to uncover edge-cases we don't have in the integration test suite.

Running the tests is straightforward. `just test` will run all tests  and `just test-one {test}` will run a single test. Make sure you have `just run-dev` or `just run` running in a separate terminal! Running the actor testing is done with `just actor-test`, and this will run indefinitely so `Ctrl + c` to cancel out.
