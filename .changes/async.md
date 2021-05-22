---
"netlify_deploy": minor
---

The File I/O reads & HTTP requests are now running asynchronously on top of Tokio. This reduced a 2,000 file upload to Netlify down to ~1 minute.
