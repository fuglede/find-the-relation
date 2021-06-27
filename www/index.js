import { Game, Direction } from "find-the-relation";

const $ = elementId => document.getElementById(elementId);
var menuItems = document.getElementsByClassName("menu-item");

const game = Game.new();

var gameViewActive = false;

function navigate(i) {
    $("rules-menu-item").style.backgroundColor = "#252525";
    for (var j = 0; j < menuItems.length - 1; j++) {
        $("level-" + j).style.backgroundColor = "#252525";
    }
    if (i == 0) {
        $("landing").style.display = "none";
        $("rules").style.display = "";
        $("game").style.display = "none";
        $("rules-menu-item").style.backgroundColor = "#151515";
        gameViewActive = false;
    } else {
        $("landing").style.display = "none";
        $("rules").style.display = "none";
        $("game").style.display = "";
        $("level-header").textContent = $("level-" + (i - 1)).innerText;
        $("level-" + (i - 1)).style.backgroundColor = "#151515";
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

const moveNorth = () => move(Direction.North);
const moveSouth = () => move(Direction.South);
const moveEast = () => move(Direction.East);
const moveWest = () => move(Direction.West);

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

$("north-button").addEventListener("click", moveNorth);
$("south-button").addEventListener("click", moveSouth);
$("east-button").addEventListener("click", moveEast);
$("west-button").addEventListener("click", moveWest);
$("reset-button").addEventListener("click", reset);


function reset() {
    game.reset();
    update_game_view();
}

function move(direction) {
    game.push(direction);
    update_game_view();
}

function update_game_view() {
    var entries = game.matrix_entries();
    var is_trivial = game.matrix_is_identity();
    var qs = game.qs();
    var distances = game.distance();

    $("completed").style.display = "none";
    $("description").textContent = game.level_description();
    $("word").textContent = game.word();
    $("matrices").innerHTML = '';
    for (var i = 0; i < entries.length / 9; i++) {
        var div = document.createElement("div");
        var qtest = document.createTextNode("𝑞 = " + qs[i] + ". Distance from target: " + distances[i]);
        div.appendChild(qtest);
        div.style.marginTop = "20px";
        div.style.padding = "10px";
        div.style.backgroundColor = is_trivial[i] && word.textContent != "" ? "#77dd77" : "#333333";
        div.style.color = is_trivial[i] && word.textContent != "" ? "#333333" : "#f5bc41";
        var tbl = document.createElement("table");
        div.appendChild(tbl);
        for (var j = 0; j < 3; j++) {
            var tr = tbl.insertRow()
            for (var k = 0; k < 3; k++) {
                var td = tr.insertCell();
                td.style.width = "20em";
                td.appendChild(document.createTextNode(entries[9*i + 3*j + k]));
            }
        }
        $("matrices").appendChild(div);
    }
    if (game.is_solved()) {
        $("completed").style.display = "";
        let current_level = game.active_level();
        $("level-" + current_level + "-completed").innerHTML = "✅";
        $("level-header").textContent = $("level-" + current_level).innerText;
    }
}