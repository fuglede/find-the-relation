import { Game, Direction } from 'find-the-relation';

const $ = (elementId) => document.getElementById(elementId);
const menuItems = document.getElementsByClassName('menu-item');

const game = Game.new();

let gameViewActive = false;

function updateGameView() {
  const entries = game.matrix_entries();
  const isTrivial = game.matrix_is_identity();
  const qs = game.qs();
  const distances = game.distance();

  $('completed').style.display = 'none';
  $('description').textContent = game.level_description();
  const word = game.word();
  $('word').textContent = word;
  $('matrices').innerHTML = '';
  for (let i = 0; i < entries.length / 9; i += 1) {
    const div = document.createElement('div');
    const matrixHeader = document.createTextNode(`𝑞 = ${qs[i]}. Distance from target: ${distances[i]}`);
    div.appendChild(matrixHeader);
    div.style.marginTop = '20px';
    div.style.padding = '10px';
    div.style.backgroundColor = isTrivial[i] && word !== '' ? '#77dd77' : '#333333';
    div.style.color = isTrivial[i] && word !== '' ? '#333333' : '#f5bc41';
    const tbl = document.createElement('table');
    div.appendChild(tbl);
    for (let j = 0; j < 3; j += 1) {
      const tr = tbl.insertRow();
      for (let k = 0; k < 3; k += 1) {
        const td = tr.insertCell();
        td.style.width = '30em';
        td.appendChild(document.createTextNode(entries[9 * i + 3 * j + k]));
      }
    }
    $('matrices').appendChild(div);
  }
  if (game.is_solved()) {
    $('completed').style.display = '';
    const currentLevel = game.active_level();
    $(`level-${currentLevel}-completed`).innerHTML = '✅';
    $('level-header').textContent = $(`level-${currentLevel}`).innerText;
  }
}

function navigate(i) {
  $('rules-menu-item').style.backgroundColor = '#252525';
  for (let j = 0; j < menuItems.length - 1; j += 1) {
    $(`level-${j}`).style.backgroundColor = '#252525';
  }
  if (i === 0) {
    $('landing').style.display = 'none';
    $('rules').style.display = '';
    $('game').style.display = 'none';
    $('rules-menu-item').style.backgroundColor = '#151515';
    gameViewActive = false;
  } else {
    $('landing').style.display = 'none';
    $('rules').style.display = 'none';
    $('game').style.display = '';
    $('level-header').textContent = $(`level-${i - 1}`).innerText;
    $(`level-${i - 1}`).style.backgroundColor = '#151515';
    game.change_level(i - 1);
    gameViewActive = true;
    updateGameView();
  }
}

for (let i = 0; i < menuItems.length; i += 1) {
  const j = i; // Copy to avoid capturing.
  menuItems[i].onclick = () => navigate(j);
}

function move(direction) {
  game.push(direction);
  updateGameView();
}

const moveNorth = () => move(Direction.North);
const moveSouth = () => move(Direction.South);
const moveEast = () => move(Direction.East);
const moveWest = () => move(Direction.West);

function reset() {
  game.reset();
  updateGameView();
}

function keydownHandler(e) {
  if (!gameViewActive) return;
  const keyCode = e.which;
  if (keyCode === 38 || keyCode === 87) { // Up arrow and w
    moveNorth();
  } else if (keyCode === 40 || keyCode === 83) { // Down arrow and s
    moveSouth();
  } else if (keyCode === 39 || keyCode === 68) { // Down arrow and d
    moveEast();
  } else if (keyCode === 37 || keyCode === 65) { // Down arrow and a
    moveWest();
  } else if (keyCode === 82) { // r
    reset();
  }
}

window.addEventListener('keydown', keydownHandler, false);
$('north-button').addEventListener('click', moveNorth);
$('south-button').addEventListener('click', moveSouth);
$('east-button').addEventListener('click', moveEast);
$('west-button').addEventListener('click', moveWest);
$('reset-button').addEventListener('click', reset);
