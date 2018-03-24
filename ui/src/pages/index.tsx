import {App} from "../application/App";

const app = new App();
app.renderReact();

console.debug('index page');

let socket = new WebSocket("ws://localhost:3001/echo");

socket.onopen = function() {
    console.debug("Соединение установлено.");
    socket.send("Привет");
};
  
socket.onclose = function(event) {
    if (event.wasClean) {
        console.debug('Соединение закрыто чисто');
    } else {
        console.debug('Обрыв соединения'); // например, "убит" процесс сервера
    }
    console.debug('Код: ' + event.code + ' причина: ' + event.reason);
};

socket.onmessage = function(event) {
    console.debug("Получены данные " + event.data);
};

socket.onerror = function(error) {
    console.debug("Ошибка " + error);
};


console.debug('index pong');

let socket2 = new WebSocket("ws://localhost:3001/pong");

socket2.onopen = function() {
    console.debug("Соединение установлено 2.");
    socket2.send("Привет2");
};
  
socket2.onclose = function(event) {
    if (event.wasClean) {
        console.debug('Соединение закрыто чисто 2');
    } else {
        console.debug('Обрыв соединения'); // например, "убит" процесс сервера
    }
    console.debug('Код 2: ' + event.code + ' причина: ' + event.reason);
};

socket2.onmessage = function(event) {
    console.debug("Получены данные 2" + event.data);
};

socket2.onerror = function(error) {
    console.debug("Ошибка " + error);
};

