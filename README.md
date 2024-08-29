## TL:DR

- `git-oidc` is a Rust crate for validating GitHub OIDC tokens
  - main functions:
    - `fetch_jwks`: fetches JSON Web Key Sets (JWKS)
    - `validate_github_token`: validate GitHub tokens against expected audiences, organizations, and repositories.


## Usuage

- `fetch_jwks` & `validate_github_token` are being used in the `oidc-service-server` to authenticate a github username

