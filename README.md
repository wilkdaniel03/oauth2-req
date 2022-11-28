### Motivation
While creating SDK for cloud provider that is using OAuth2 as authentication protocol there was no library which I could use to 
construct requests specifically for interaction with the protocol, so I decided to create one.

### Goals
This library is responsible for constructing http requests interacting with OAuth2 protocol.
It does not provide backwards compatibility with previous versions of OAuth protocol (pre 2.0 versions).
It is not responsible for parsing environment variables or deserializing requests.
It should be compatible with every used version of HTTP protocol.
