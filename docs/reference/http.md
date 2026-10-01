# HTTP API

The clients are thin wrappers over this API. Use it directly from a language
without a client, or to see exactly what a client sends. Every tool is
`POST /v1/tools/{name}` with a JSON body, authorised with a bearer token
obtained by exchanging your API key (see [Authentication](../getting-started/authentication.md)).

The specification is also available as <a href="../../openapi.json">`openapi.json`</a>.

<div id="redoc-container"></div>
<script src="https://cdn.jsdelivr.net/npm/redoc@2.5.4/bundles/redoc.standalone.js"></script>
<script>
  Redoc.init(new URL("../../openapi.json", window.location.href).href,
             { hideDownloadButton: false, scrollYOffset: 64 },
             document.getElementById("redoc-container"));
</script>
