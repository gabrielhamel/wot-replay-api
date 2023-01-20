# Wot replay api

> Work in progress

An api that take a replay URL and returns graphql response containing many replay information.

The main goal of this project is to extract game events for statistics purposes (Charts, ...)

A huge thanks to [Evido](https://github.com/evido) for his replay [parsing crate](https://github.com/evido/wotreplay-parser) and
[Dacite](https://github.com/dacite) for the rust [crate rewrite](https://github.com/dacite/wot-battle-results-parser) 

The listening url http://localhost:8000/graphql

## Usage

Playground available here http://localhost:8000/graphiql

**Query**:

```graphql
{
  replay(url: "http://wotreplays.eu/site/download/6462727") {
    map {
      name
      displayName
    }
    date
    player {
      id
      name
    }
    version {
      executable
      xml
    }
    server {
      name
      regionCode
    }
  }
}
```

**Response:**

```json
{
  "data": {
    "replay": {
      "map": {
        "name": "31_airfield",
        "displayName": "Flugplatz"
      },
      "date": "18.01.2023 13:26:00",
      "player": {
        "id": 579345936,
        "name": "ptrmDE"
      },
      "version": {
        "executable": "1.19.1.0",
        "xml": "World of Tanks v.1.19.1.0 #93"
      },
      "server": {
        "name": "WOT EU1",
        "regionCode": "EU"
      }
    }
  }
}
```

Feel free to contribute if you want more data extracted from the replay.
