const WebSocket = require('ws')

const wss = new WebSocket.Server({ port: 9898 })

console.log(`Server opened 9898`);

wss.on('connection', ws => {
  ws.on('message', message => {
    console.log(`${message}`)
  })
  ws.send('Welcome!')
  console.log("CLIENT CONNECTED")
})
