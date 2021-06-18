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

document.getElementById("north-button").addEventListener("click", moveNorth);
document.getElementById("south-button").addEventListener("click", moveSouth);
document.getElementById("east-button").addEventListener("click", moveEast);
document.getElementById("west-button").addEventListener("click", moveWest);

function update_table() {
    var evaluated = easy.evaluated();
    var is_trivial = easy.evaluation_is_trivial();
    document.getElementById("matrices").innerHTML = '';
    for (var i = 0; i < evaluated.length/9; i++) {
        var div = document.createElement("div");
        div.style.marginTop = "20px";
        div.style.backgroundColor = is_trivial[i] && word.textContent != "" ? "#77dd77" : "#ff6961";
        var tbl = document.createElement("table");
        div.appendChild(tbl);
        for (var j = 0; j < 3; j++) {
            var tr = tbl.insertRow()
            for (var k = 0; k < 3; k++) {
                var td = tr.insertCell();
                td.style.width = "20em";
                td.appendChild(document.createTextNode(evaluated[9*i + 3*j + k]));
            }
        }
        document.getElementById("matrices").appendChild(div);
    }
    document.getElementById("matrices").appendChild(document.createTextNode(easy.det()));
    document.getElementById("matrices").appendChild(document.createElement("br"));
    document.getElementById("matrices").appendChild(document.createTextNode(easy.tr()));

}