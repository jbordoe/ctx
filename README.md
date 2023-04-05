# ctx

Fast context switching with predefined environment variable groups

## Usage

```
ctx [OPTIONS] -- COMMAND
```

Use`ctx` with any `.env` file containing your environment variables.

Consider having multiple `.env` files for different contexts (staging, docker etc) and using `ctx` to easily switch.

### Examples
Run a command using the variables defined in `.env.example`
```
ctx -f .env.example -- printenv | grep FOO
```

Run a command using the variables defined in `.env.example`, but override the value for `FOO`
```
ctx -f .env.example -e FOO=OVERRIDEN -- printenv | grep FOO
```

Use `ctx --help` for more information
