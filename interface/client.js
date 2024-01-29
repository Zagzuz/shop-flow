const PORT = 5000;

const socket = io(':' + PORT, {
	cors: {
        origin: 'http://localhost:' + PORT,
    	methods: ["GET", "POST"],
    	credentials: true,
    }
});

socket.on('itemsResponse', (items) => {
	let list = document.getElementById('itemsList');
	list.innerHTML = "";
	items.forEach((item) => {
        let li = document.createElement("li");
        li.innerText = item.title + ', price: ' + item.price.toFixed(2) + ', count: ' + item.count;
        list.appendChild(li);
    });
});

function clickSearchButton() {
	const query = document.getElementById('searchField').value;
	if (typeof query === "string" && query.length !== 0) {
		socket.emit('search', query);
	}
}
