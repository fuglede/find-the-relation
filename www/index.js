import { Game, Direction } from "find-the-relation";

const easy = Game.easy();
const word = document.getElementById("word");

window.addEventListener("keydown", keydownHandler, false);

function keydownHandler(e) {
    var keyCode = e.which;
    if (keyCode == 38) {
        moveNorth();
    } else if (keyCode == 40) {
        moveSouth();
    } else if (keyCode == 39) {
        moveEast();
    } else if (keyCode == 37) {
        moveWest();
    }
}

function moveNorth() {
    move(Direction.North, "N", "S");
}

function moveSouth() {
    move(Direction.South, "S", "N");
}

function moveEast() {
    move(Direction.East, "E", "W");
}

function moveWest() {
    move(Direction.West, "W", "E");
}

function move(direction, symbol, opposite) {
    easy.push(direction);
    if (word.textContent.slice(-1) != opposite) {
        word.textContent += symbol;
    } else {
        word.textContent = word.textContent.substring(0, word.textContent.length - 1);
    }
    update_table();
}

const northButton = document.getElementById("north-button");
northButton.addEventListener("click", moveNorth);
const southButton = document.getElementById("south-button");
southButton.addEventListener("click", moveSouth);
const eastButton = document.getElementById("east-button");
eastButton.addEventListener("click", moveEast);
const westButton = document.getElementById("west-button");
westButton.addEventListener("click", moveWest);

function update_table() {
    var el = "";
    var evaluated = easy.evaluated();
    var is_trivial = easy.evaluation_is_trivial();
    for (var i = 0; i < 3; i++) {
        var color = is_trivial[i] && word.textContent != "" ? "green" : "red";
        el += "<div style=\"margin-top: 20px; background-color:" + color +"\"><table>"
        for (var j = 0; j < 3; j++) {
            el += "<tr>"
            for (var k = 0; k < 3; k++) {
                el += "<td>" + evaluated[9*i + 3*j + k] + "</td>";
            }
            el += "</tr>";
        }
        el += "</table></div>";
    }
    document.getElementById("boo").innerHTML = el
}