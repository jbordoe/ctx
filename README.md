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

#### Using env files
You can point to any file containing environment variable definitions:
```bash
FOO=BAR
export NAME=ALICE
# comments will be ignored
# AGE=99
```

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

#### Using `.ctx.yml`
You can keep all your variables in a `.ctx.yml` file. The file should contain multiple named `contexts`, each with a list of `variables` which have a `name` and a `value`:

```yaml
contexts:
  en:
    variables:
      - name: GREETING
        value: Hello
      - name: SUBJECT
        value: World

  it:
    variables:
      - name: GREETING
        value: Ciao
      - name: SUBJECT
        value: Mondo
...
```
When you pass in a context, ctx will automatically pull in the variables from `.ctx.yml`:
```bash
ctx -c it -- bash -c 'echo "$GREETING, $SUBJECT!"'
```
Should display
```
Ciao, Mondo!
```
You can still use overrides with `-e`:
```bash
ctx -c it -e SUBJECT=ragazzi -- bash -c 'echo "$GREETING, $SUBJECT!"'
```
Should display
```
Ciao, ragazzi!
```

Use `ctx --help` for more information
