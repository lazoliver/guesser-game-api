# Guesser-Game-API

## What is Guesser Game API?

Guesser Game Api é um jogo, com ele é possível cadastrar uma palavra secreta com três dicas, a cada tentativa incorreta o contado de tentativas é agressido de um. Ao atingir os valores pré-definidos em clue1_attempts, clue2_attempts e clue3_attempts, os valores refetentes a clue1, clue2 e clue3 são revelados. Quando o palpite estiver correto, o valor do segredo é revelado.

## Project Structure

### Handlers

The Guesser Game API includes useful handlers for daily needs. Below, we list and provide examples for each of them.

#### Utils

**health_handler**

This handler is used to check the health of the API. Make a GET request to the URL/health. If the API is functioning correctly, a status 200 and a JSON will be returned, as shown below:

```json
{
  "status": "pass"
}
```

**full_health_handler**

This handler is used to check the health of both the API and the database. Make a GET request to URL/health/full. Depending on the situation, it will return a status 200 or an error, accompanied by a JSON. Examples:

If everything is OK:

```json
{
  "status": "pass",
  "uptime": 60,
  "db": true
}
```

In the case of a database error:

```json
{
  "status": "fail",
  "uptime": 60,
  "db": false
}
```

#### Secret

**create_secret_handler**

The `create_secret_handler` is used to create a new secret for our game. To do this, send a POST request to the `URL/secrets`, where the body must be a JSON, as shown below:

Request body:

```json
{
	"secret":  <secret>,
	"clue1":  <clue1>,
	"clue2":  <clue2>,
	"clue3":  <clue3>
}
```

Successful creation response:

```json
{
	"id":  <id>,
	"guessed":  false,
	"guess_attempts":  0
}
```

**get_secret_handler**

The `get_secret_handler` is used to search the database for a secret with the <id> entered. To do this, send a GET request to the `URL/secrets/<id>`, in the event of success (secret guessed or not) or failure (not found), the response will be a json, see the examples below:

Successful get secret (guessed) response:
Note: the clue1, clue2 and clue3 fields can be displayed according to the number of guess_attempts.

```json
{
	"id":  <id>,
	"guessed":  true,
	"guess_attempts":  0,
	"guesser":  <guesser-name>,
	"secret":  <secret>
}
```

Successful get secret (not guessed) response:

```json
{
	"id":  <id>,
	"guessed":  false,
	"guess_attempts":  0
}
```

Secret not found response:

```json
{
  "error": "Secret not found."
}
```

**get_all_secrets_handler**

The `get_all_secrets_handler` returns all the secrets in the database following some predefined criteria, all requests are of the GET type, see the examples below:

GET request for `URL/secrets` or `URL/secrets?guessed=false` will return all unguessed secrets:

```json
{
"secrets": [
		{
			"id":  <id>,
			"guessed":  false,
			"guess_attempts":  0
		},
		{
			"id":  <id>,
			"guessed":  false,
			"guess_attempts":  0
		}
	]
}
```

GET request to `URL/secrets?guessed=true` will return all secrets including those already guessed:

```json
{
	"secrets": [
		{
			"id":  <id>,
			"guessed":  true,
			"guess_attempts":  15,
			"clue1":  <clue1>,
			"clue2":  <clue2>,
			"clue3":  <clue3>,
			"guesser":  <guesser-name>,
			"secret":  <secret>
		},
		{
			"id":  <id>,
			"guessed":  true,
			"guess_attempts":  0,
			"guesser":  <guesser-name>,
			"secret":  <secret>
		},
		{
			"id":  <id>,
			"guessed":  false,
			"guess_attempts":  0
		},
		{
			"id":  <id>,
			"guessed":  false,
			"guess_attempts":  0
		}
	]
}
```

**guess_secret_handler**

The `guesses_secret_handler` is responsible for receiving a POST request to `URL/secrets/<id>`, the body of the request must contain a JSON. If the guess sent in the JSON is incorrect, see the example below:

Request body:

```json
{
	"guess":  <your-guess>,
	"username":  <guesser-name>"
}
```

Answer if the secret has not yet been guessed, but the guess is incorrect:

Note: With each new incorrect guess, the guess_attempts counter is incremented by one.

```json
{
	"id":  <id>,
	"guessed":  false,
	"guess_attempts":  1
}
```

If the guess is correct, this will be the handler's answer:

```json
{
	"id":  <id>,
	"guessed":  true,
	"guess_attempts":  1,
	"guesser":  <guesser-name>,
	"secret":  <guess>
}
```

If the secret has already been guessed, take a look at the example answer:

```json
{
  "error": "Secret already guessed."
}
```

### Storage

#### Secret

**create_secret**

The `create_secret` function is the main function of our API, it receives an object of type NewSecret:

```rust
pub  struct  NewSecret {
	pub secret:  String,
	pub clue1:  String,
	pub clue2:  String,
	pub clue3:  String,
}
```

The value of NewSecret.secret is encrypted using Keccak256 and saved in the hashed_secret variable.

The collection of secrets receives an object of type SecretEntity:

```rust
pub  struct  SecretEntity {
	pub id:  Uuid,
	pub secret:  String,
	pub clue1:  String,
	pub clue2:  String,
	pub clue3:  String,
	pub guess_attempts:  u16,
	pub guesser:  Option<String>,
	pub guessed_secret:  Option<String>,
}
```

To save a new secret in the collection, we need to pass the data provided by NewSecret to SecretEntity:

```rust
let secret =  SecretEntity {
	id:  Uuid::new_v4(),
	secret: hashed_secret,
	clue1: new_secret.clue1,
	clue2: new_secret.clue2,
	clue3: new_secret.clue3,
	guess_attempts:  0,
	guesser:  None,
	guessed_secret:  None,
}
```

Now our secret object is stored in the database and finally the function returns the secret created or an error of the type `AppError::MongoDbError`.

**get_secret_entity**

This function is responsible for retrieving a SecretEntity from the database, for which it receives a `secret_id: Uuid`. The function has a filter for the database, using the `id` field as a parameter. If the search returns a secret, this secret will be returned by the function, otherwise an error of type `AppError::NotFound` will be returned.

**get_all_secrets**

This function is responsible for returning an array of secrets: `Vec<SecretEntity>`, which may or may not contain the secrets already guessed. To do this, the function receives a `with_guessed: bool`, if the value is false all the secrets will be unguessed, if the value is true all the secrets will be a combination of guessed and unguessed.

**guess_secret**

The `guess_secret` function receives three parameters: `secret_id: Uuid`, `guess: String` and `username: String`. With the secret_id, we use the `get_secret_entity` function to return a secret from the database, if the `guesser` field is equal to Some we return `AppError::AlreadyGuessed`.
If `guesser` is equal to None, the function will take the value of `guess` and create a `hashed_guess`, if the value of `hashed_guess` and `secret.secret` are different, it updates the secret in the database with the counter of `guess_attempts`+1. And finally, if `hashed_guess` and `secret.secret` are the same, it updates the secret in the database, but now filling in the `guesser` and `guessed_secret` fields with the values received in `username` and `guess` respectively.

## How to Run This Project

1. To run this application, clone the repository [Guesser-Game-API](https://github.com/lazoliver/guesser-game-api.git).
2. Install Rust by visiting the official [Rust](https://www.rust-lang.org/tools/install) website and following the steps.
   2.1. You need to run the command: `$ rustup override set nightly`, it allows you to run our application in the latest development version.
3. Create a _.env_ file and configure the environment variables. You can use the values from _.env.example_.
4. Run the command `$ cargo build` to install the project's binaries and libraries.
5. Now run the command `$ cargo run`, and the application will execute successfully. Enjoy!

## Environment Variables

In this section, we will detail the environment variables and their roles in our API. These are the necessary environment variables to run the application successfully, and you can see example values in the _.env.example_ file. They are as follows:

- `RELEASE_MODE`: This variable is responsible for defining the execution level of the application, either in development (`dev`) or production (`prod`). Choose the option that best suits your scenario. Opting for `dev` sets the lowest log level to `Debug`, while in `prod`, the lowest log level is set to `Info`.
- `API_PORT`: Specify the port on which the API will receive requests. Feel free to choose according to the most convenient scenario. By default, we set the value to 4000.
- `MONGO_URI`: This variable should point to your preferred MongoDB database, whether local or in the cloud. Remember that this field is of type `String`.
- `TIMEOUT`: Finally, the variable responsible for determining the timeout for requests. By default, this value is 75, but it can be changed according to your organization. This field is of type `Number`.
- `CLUE1_ATTEMPTS`: This variable contains a value of type `Number`, e.g. 5. When the number of attempts reaches this value, the value of the hint is revealed.
- `CLUE2_ATTEMPTS`: This variable contains a value of type `Number`, e.g. 10. When the number of attempts reaches this value, the value of the hint is revealed.
- `CLUE3_ATTEMPTS`: This variable contains a value of type `Number`, e.g. 15. When the number of attempts reaches this value, the value of the hint is revealed.
