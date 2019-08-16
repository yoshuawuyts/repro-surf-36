var http = require('http')

http.createServer((req, res) => {
  var body = []
  for (let i = 0, j = 100000; i < j; i++) {
    body.push({ message: 'hello world' })
  }
  res.end(JSON.stringify(body))
}).listen(8080, () => console.log('listening on port 8080'))
