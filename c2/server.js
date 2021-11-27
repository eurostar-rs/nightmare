'use strict';
var http = require('http');
var port = process.env.PORT || 3056;
var fs = require('fs');

const server = http.createServer();
server.on('request', (request, response) => {
    var ip = req.headers['x-forwarded-for'] || req.socket.remoteAddress || null;
    fs.writeFile('ip.txt', `${ip}`)
    console.log("test")
}).listen(port);
