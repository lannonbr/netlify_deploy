# Changelog

## \[0.3.0]

- The File I/O reads & HTTP requests are now running asynchronously on top of Tokio. This reduced a 2,000 file upload to Netlify down to ~1 minute.
  - [238b450](https://github.com/lannonbr/netlify_deploy/commit/238b45065819e1e154b07c407e4cba7d5808acf6) Switching sync tasks to run async on 2021-05-22

## \[0.2.2]

- Trying a final time to fix the workflow
  - [f4bd2e2](https://github.com/lannonbr/netlify_deploy/commit/f4bd2e28068633efc9c9644cf5910d524fbe60ad) final tweak to CI on 2021-05-20

## \[0.2.1]

- Updated version-or-publish workflow to get rid of duplicate release, update the release name, and properly populate release body.
  - [30689f4](https://github.com/lannonbr/netlify_deploy/commit/30689f43306e2a3ad04678eb513349cf6fbbdd97) update workflow on 2021-05-20

## \[0.2.0]

- Added `color-eyre` and `bimap` packages.
  - [1dc4cb2](https://github.com/lannonbr/netlify_deploy/commit/1dc4cb2b4ff9ffca600d2ae299d072104e4183c6) Adding covector on 2021-05-20
- Add flag for deploying to production or not. See #3
  - [1dc4cb2](https://github.com/lannonbr/netlify_deploy/commit/1dc4cb2b4ff9ffca600d2ae299d072104e4183c6) Adding covector on 2021-05-20
- Cleaning up reqwest logic by inserting headers directly into RequestBuilder chain.
  - [1dc4cb2](https://github.com/lannonbr/netlify_deploy/commit/1dc4cb2b4ff9ffca600d2ae299d072104e4183c6) Adding covector on 2021-05-20
