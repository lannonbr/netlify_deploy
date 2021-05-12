# netlify-rust

This is going to be a Rust CLI that uses the Netlify API to deploy sites.

To work properly, the `NETLIFY_SITE_ID` and `NETLIFY_AUTH_TOKEN` environment variables need to be set, similar to the official Netlify CLI.

This is still an early-on prototype so there may be bugs about. It does not have the full functionality of the official CLI but it was able to solve my particular use case of running `netlify deploy` in CI environments.
