import { Game, Direction } from "find-the-relation";

const game = Game.new();

var menuItems = document.getElementsByClassName("menu-item");
var gameViewActive = false;

function navigate(i) {
    console.log(i)
    document.getElementById("rules-menu-item").style.backgroundColor = "#252525";
    for (var j = 0; j < menuItems.length - 1; j++) {
        document.getElementById("level-" + j).style.backgroundColor = "#252525";
    }
    if (i == 0) {
        document.getElementById("landing").style.display = "none";
        document.getElementById("rules").style.display = "";
        document.getElementById("game").style.display = "none";
        document.getElementById("rules-menu-item").style.backgroundColor = "#151515";
        gameViewActive = false;
    } else {
        document.getElementById("landing").style.display = "none";
        document.getElementById("rules").style.display = "none";
        document.getElementById("game").style.display = "";
        document.getElementById("level-header").textContent = document.getElementById("level-" + (i - 1)).innerText;
        document.getElementById("level-" + (i - 1)).style.backgroundColor = "#151515";
        game.change_level(i - 1);
        gameViewActive = true;
        update_game_view();
    }
}

for (var i = 0; i < menuItems.length; i++) {
    const j = i;  // Copy to avoid capturing.
    menuItems[i].onclick = function() { navigate(j) };
};

window.addEventListener("keydown", keydownHandler, false);

function keydownHandler(e) {
    if (!gameViewActive) return;
    var keyCode = e.which;
    if (keyCode == 38 || keyCode == 87) {  // Up arrow and w
        moveNorth();
    } else if (keyCode == 40 || keyCode == 83) {  // Down arrow and s
        moveSouth();
    } else if (keyCode == 39 || keyCode == 68) {  // Down arrow and d
        moveEast();
    } else if (keyCode == 37 || keyCode == 65) {  // Down arrow and a
        moveWest();
    } else if (keyCode == 82) {  // r
        reset();
    }
}
  
function moveNorth() {
    move(Direction.North);
}

function moveSouth() {
    move(Direction.South);
}

function moveEast() {
    move(Direction.East);
}

function moveWest() {
    move(Direction.West);
}

function reset() {
    game.reset();
    update_game_view();
}

function move(direction) {
    game.push(direction);
    update_game_view();
}

document.getElementById("north-button").addEventListener("click", moveNorth);
document.getElementById("south-button").addEventListener("click", moveSouth);
document.getElementById("east-button").addEventListener("click", moveEast);
document.getElementById("west-button").addEventListener("click", moveWest);
document.getElementById("reset-button").addEventListener("click", reset);

function update_game_view() {
    var evaluated = game.evaluated();
    var is_trivial = game.evaluation_is_trivial();
    var qs = game.qs();
    var distances = game.distance();

    document.getElementById("completed").style.display = "none";
    document.getElementById("description").textContent = game.level_description();
    document.getElementById("word").textContent = game.word();
    document.getElementById("matrices").innerHTML = '';
    for (var i = 0; i < evaluated.length/9; i++) {
        var div = document.createElement("div");
        var qtest = document.createTextNode("𝑞 = " + qs[i] + ". Distance from target: " + distances[i]);
        div.appendChild(qtest);
        div.style.marginTop = "20px";
        div.style.backgroundColor = is_trivial[i] && word.textContent != "" ? "#77dd77" : "#333333";
        div.style.color = is_trivial[i] && word.textContent != "" ? "#333333" : "#f5bc41";
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
    if (game.is_solved()) {
        document.getElementById("completed").style.display = "";
        let current_level = game.active_level();
        document.getElementById("level-" + current_level + "-completed").innerHTML = "✅";
        document.getElementById("level-header").textContent = document.getElementById("level-" + current_level).innerText;
    }
}