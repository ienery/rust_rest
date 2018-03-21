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

