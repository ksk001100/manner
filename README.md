# Manner app

## Development setup

```bash
$ docker-compose build
$ docker-compose up
```

## Backend development

### Add migration
```bash
$ docker-compose run backend diesel migration generate <migration name>
```

### Run migration
```bash
$ docker-compose run backend diesel migration run
```

### Frontend development

### Add npm package
```bash
$ docker-compose run frontend yarn add <npm package>
```
