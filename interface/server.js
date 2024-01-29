const express = require('express');
const socket_io = require('socket.io');
const http = require('http');
const path = require('path');
const grpc = require('@grpc/grpc-js');
const protoLoader = require('@grpc/proto-loader');

const PORT = 5000;
const PROTO_PATH = __dirname + '/../proto/catalog/v1/catalog.proto';

const app = express();
const http_listener = http.createServer(app);

const io = socket_io(http_listener, {
    cors: {
        origin: 'http://localhost:' + PORT,
        methods: ["GET", "POST"],
        credentials: true,
    }
});

const loaderOptions = {
    keepCase: true,
    longs: String,
    enums: String,
    defaults: true,
    oneofs: true
};

const packageDefinition = protoLoader.loadSync(PROTO_PATH, loaderOptions);
const catalog = grpc.loadPackageDefinition(packageDefinition).proto.catalog.v1;

const grpcServerUrl = '[::1]:50051';
const client = new catalog.CatalogService(grpcServerUrl, grpc.credentials.createInsecure());

function runListItems(socket) {
    function itemsCallback(error, response) {
        if (error) {
            console.log(error);
            return;
        }
        if (response === null) {
            return;
        }
        if (response.items.length === 0) {
            console.log('no items');
            return;
        }
        for (let i = 0; i < response.items.length; i++) {
            const item = response.items[i];
            console.log(i + 1 + '. ' + item.title + ', price: ' +
                item.price + ', count: ' + item.count);
        }
        socket.emit('itemsResponse', response.items);
    }
    console.log('sending list items request..');
    client.listItems({}, itemsCallback);
}

function main() {
    app.get('/', (req, res) => {
        res.sendFile(path.join(__dirname, 'public', 'index.html'));
    });

    app.get('/client.js', (req, res) => {
        res.sendFile(path.join(__dirname, 'client.js'));
    });

    io.on('connection', (socket) => {
        console.log(socket.id + ' connected');
        socket.emit('connection', null);
        runListItems(socket);
        socket.on('search', (query) => {
            console.log(query + ' searched');
        });
    })

    http_listener.listen(PORT, function() {
        console.log('listening on *:' + PORT);
    });
}

if (require.main === module) {
    main();
}
