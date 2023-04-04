# ctx

Fast context switching with predefined environment variable groups

## Usage

`ctx -e DOTENV_FILE COMMAND`

Use`ctx` with any `.env` file containing your environment variables.

Consider having multiple `.env` files for different contexts (staging, docker etc) and using `ctx` to easily switch.

### Example
```
ctx -e .env.example printenv | grep FOO
```
