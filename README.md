# rust wasm worker hello world

Built using the template at https://github.com/cloudflare/rustwasm-worker-template/ and then complemented with 
the code form this tutorial https://developers.cloudflare.com/workers/tutorials/hello-world-rust which 
can also be found in this repo https://github.com/granjef3/rustwasm-markdown-parser

# to run it locally
`wrangler dev`
then open the web page at the Url echoed to the terminal 

# run tests
Run wasm-bindgen tests using
`wasm-pack test --chrome`

or if you have chromedriver installed (and working!)

`wasm-pack test --headless --chrome`

# publish 
`wrangler publish` then visit the site to see the markdown shown as HTML.