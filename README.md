# Convert Openapi format to Postman Collection
The original import function in postman is not working as expected, this repo currently support generate postman collection 2.1 from openapi3

## Some feature that postman collection dont have
- group to different folder with tag
- use example value for query instead of `<string>`
- map header value to your postman environment variable

## Usage
cargo run -- --stp -c config.example.json -o testfolder/postmancollection.json testfolder/microservice-swagger.json

cargo run -- --pts -c api-lib-config.example.json -o testfolder/api-lib-swagger.json testfolder/api-lib-postman.v2.1.json

## Note
This might not feature complete, we can slowly make it support more
