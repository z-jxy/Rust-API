let roomListDiv = document.getElementById('room-list');
let messagesDiv = document.getElementById('messages');
let newMessageForm = document.getElementById('new-message');
let newRoomForm = document.getElementById('new-room');
let statusDiv = document.getElementById('status');

let usersListDiv = document.getElementById('users-list');
let usersTemplate = document.getElementById('users');

let roomTemplate = document.getElementById('room');
let messageTemplate = document.getElementById('message');

let messageField = newMessageForm.querySelector("#message");
let usernameField = newMessageForm.querySelector("#username");
let roomNameField = newRoomForm.querySelector("#name");


let connectedUsers = document.querySelector(".users-counter");

let members = [];
let global_members = [];

var STATE = {
  room: "C2-Channel",
  rooms: {},
  connected: false,
  users: {},
}

// Generate a color from a "hash" of a string. Thanks, internet.
function hashColor(str) {
  let hash = 0;
  for (var i = 0; i < str.length; i++) {
    hash = str.charCodeAt(i) + ((hash << 5) - hash);
    hash = hash & hash;
  }

  return `hsl(${hash % 360}, 100%, 70%)`;
}

// Add a new room `name` and change to it. Returns `true` if the room didn't
// already exist and false otherwise.
function addRoom(name) {
  if (STATE[name]) {
    changeRoom(name);
    return false;
  }

  var node = roomTemplate.content.cloneNode(true);
  var room = node.querySelector(".room");
  room.addEventListener("click", () => changeRoom(name));
  room.textContent = name;
  room.dataset.name = name;
  roomListDiv.appendChild(node);

  STATE[name] = [];
  changeRoom(name);
  return true;
}

// Change the current room to `name`, restoring its messages.
function changeRoom(name) {
  if (STATE.room == name) return;

  var newRoom = roomListDiv.querySelector(`.room[data-name='${name}']`);
  var oldRoom = roomListDiv.querySelector(`.room[data-name='${STATE.room}']`);
  if (!newRoom || !oldRoom) return;

  STATE.room = name;
  oldRoom.classList.remove("active");
  newRoom.classList.add("active");

  messagesDiv.querySelectorAll(".message").forEach((msg) => {
    messagesDiv.removeChild(msg)
  });

  STATE[name].forEach((data) => addMessage(name, data.username, data.message))
}

// Add `message` from `username` to `room`. If `push`, then actually store the
// message. If the current room is `room`, render the message.
function addMessage(room, username, message, users, push = false) {

  if (push) {
    STATE[room].push({ username, message });
    members.push({ username });
    STATE[users].push({ username });
  }

  if (STATE.room == room) {
    var node = messageTemplate.content.cloneNode(true);
    node.querySelector(".message .username").textContent = username;
    node.querySelector(".message .username").style.color = hashColor(username);
    node.querySelector(".message .text").textContent = message;
    messagesDiv.appendChild(node);
  }
}

// Subscribe to the event source at `uri` with exponential backoff reconnect.
function subscribe(uri) {
  var retryTime = 1;

  function connect(uri) {
    const events = new EventSource(uri);

    events.addEventListener("message", (ev) => {
      console.log("raw data", JSON.stringify(ev.data));
      console.log("decoded data", JSON.stringify(JSON.parse(ev.data)));
      const msg = JSON.parse(ev.data);
      if (!"message" in msg || !"room" in msg || !"username" in msg) return;
      addMessage(msg.room, msg.username, msg.message, true);
    });

    events.addEventListener("open", () => {
        let _members = 0; 
        _members += 1;

        console.log(`got a new member!`);
        console.log({ _members });
        //for (let i = 0; i < members.length; i++) {
        //    _members += members[i] + "<br>";
        //}
        if (members.length !== 0) {
            try {
                addMessage("C2-Channel", "Hall Monitor", `Current clients connected to chat: ${members.length}`);
            }
            catch {
                console.log(`we got an issue ${Error}`);
            };
            retryTime = 1;
        }
        


      console.log(`users state: ${STATE[users]}`);
      setConnectedStatus(true);
      console.log(`connected to event stream at ${uri}`);
      members.push(1);
      console.log(`members on event stream: ${members}`);
      retryTime = 1;
    });

    events.addEventListener("check-members", () => {
        let _members = "";
        
        for (let i = 0; i < members.length; i++) {
            _members += members[i] + "<br>";
        }

        document.getElementById("_members").innerHTML = text;
        addMessage(msg.room, "Hall Monitor", _members);
        console.log(`inc'd _members ${_members}`);
        retryTime = 1;
      });

    events.addEventListener('member_join', member => {
        members.push(member);
    });

    events.addEventListener('member_leave', ({username}) => {
        const index = members.findIndex(member => member.id ===  username);
        members.splice(index, 1);
        console.log(`spliced index ${members.entries}`);
    });


    events.addEventListener("error", () => {
      setConnectedStatus(false);
      events.close();

      let timeout = retryTime;
      retryTime = Math.min(64, retryTime * 2);
      console.log(`connection lost. attempting to reconnect in ${timeout}s`);
      setTimeout(() => connect(uri), (() => timeout * 1000)());
    });
  }

  connect(uri);
}

// Set the connection status: `true` for connected, `false` for disconnected.
function setConnectedStatus(status) {
  STATE.connected = status;
  statusDiv.className = (status) ? "connected" : "reconnecting";
}

// Let's go! Initialize the world.
function init() {
  // Initialize some rooms.

  addRoom("C2-Channel");
  addRoom("Private-Comms");
  changeRoom("C2-Channel");
  addMessage("C2-Channel", "Admin", "Hey! Open another browser tab, send a message.", true);

  addMessage("C2-Channel", "Hall Monitor", `Currently connect clients: ${ JSON.stringify( conClients) }`, true);
  addMessage("Private-Comms", "Admin", "You can use this room for groups of 2 or 3", true);

  // Set up the form handler.
  newMessageForm.addEventListener("submit", (e) => {
    e.preventDefault();

    const room = STATE.room;
    const message = messageField.value;
    const username = usernameField.value || "guest";

    if (!message || !username) return;

    if (STATE.connected) {
      fetch("/api/message", {
        method: "POST",
        body: new URLSearchParams({ room, username, message }),
      }).then((response) => {
        if (response.ok) messageField.value = "";
      });
    }
  })

  // Set up the new room handler.
  newRoomForm.addEventListener("submit", (e) => {
    e.preventDefault();

    const room = roomNameField.value;
    if (!room) return;

    roomNameField.value = "";
    if (!addRoom(room)) return;

    addMessage(room, "Rocket", `Look, your own "${room}" room! Nice.`, true);
  })

  // Subscribe to server-sent events.
  subscribe("/api/events");
}

init();
