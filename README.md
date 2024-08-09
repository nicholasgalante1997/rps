# Rust Pod Server (rps)

*Action Items*

When a request (http) hits our server, the following needs to occur

1. Check if the request is for a special API route
   1. This could be Auth based
   2. This could be a health check
2. Check if the request is for a static resource path
   1. Check if the requested path is in our pod
   2. Check the content type / accept headers to determine response format
   3. Check the access policy of the resource
   4. Check the authorization of the request
   5. Respond with the resource in the requested format

## Utilities

For **container resources**, Rust utility module to convert the contents of a directory to the following format

- RDF (ttl, jsonld)
- JSON
- Determine if container resource is root