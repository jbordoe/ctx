# ctx

Fast context switching with predefined environment variable groups

## Usage

```
ctx [OPTIONS] -- COMMAND
```

Use`ctx` with any `.env` file containing your environment variables.

Consider having multiple `.env` files for different contexts (staging, docker etc) and using `ctx` to easily switch.

### Examples
> **_NOTE:_**  In these examples we include our command as a string argument to `bash` to ensure the variables are interpolated **after** Rust updates the environment. In other instances you can just pass your command as is.

Run a command using the variables defined in `.env.en.example`
```bash
ctx -f .env.example -- bash -c 'echo "$GREETING, $SUBJECT!"'
```
Should display
```
Hello, World!
```

Now run the same command using the variables defined in `.env.de.example`
```bash
ctx -f .env.de.example -- bash -c 'echo "$GREETING, $SUBJECT!"'
```
Should display
```
Hallo, Welt!
```

Again, using the variables defined in `.env.de.example`, but override the value for `GREETING`
```bash
ctx -f .env.de.example -e GREETING=Moin -- bash -c 'echo "$GREETING, $SUBJECT!"'
```
Should display
```
Moin, Welt!
```

Use `ctx --help` for more information
