import { Game } from "find-the-relation";

const easy = Game.easy();
setInterval(() => {
    easy.push(0);
    document.getElementById("boo").textContent = easy.q1_evaluated();
  }, 1000);
/*const playPauseButton = document.getElementById("movebutton");
playPauseButton.addEventListener("click", event => {
    easy.push(0);
    document.getElementById("boo").textContent = easy.q1_evaluated();
  });*/