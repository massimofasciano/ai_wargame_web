import init, { Game } from "./pkg/ai_wargame_web.js";

// player_next_move and cancel_move use this variable
var player_next_move_first_coord = undefined; 

// player_next_move, enable_auto_reply
// and disable_auto_reply use this variable
var computer_auto_reply = true;

init().then(() => {
    new_game();
});

function new_game() {
    let game = new Game();
    setup_board_onclick(game);
    show_board(game);
    enable_auto_reply();
    options_setup(game);
    show_damage_table(game);
    show_repair_table(game);
    setup_buttons(game);
}

function player_next_move(game, row, col) {
    console.log("Clicked at row:",row," col:",col);
    if (is_game_over(game)) {
        let message = "Game is already finished!";
        show_stats(message);
        console.log(message);
        return;
    }
    if (player_next_move_first_coord == undefined ) {
        player_next_move_first_coord = [row,col];
        document.getElementById("cancel-move").innerHTML=
            "<button>Cancel move from "+
                coord_string(player_next_move_first_coord)
            +"</button>";
        document.getElementById("cancel-move").onclick=cancel_move;
    } else {
        let to = [row,col];
        let from = player_next_move_first_coord;
        cancel_move();
        let result = game.player_play_turn(from[0],from[1],to[0],to[1]);
        if (result != undefined) {
            show_board(game);
            show_stats(result);
            check_winner(game, function() {
                if (computer_auto_reply) {
                    game_iteration_computer(game, false)
                }
            });
        } else {
            let message = "Invalid move!";
            show_winner(message);
            show_stats(message);            
        }
    }
}

function setup_buttons(game) {
    document.getElementById("restart").onclick = new_game;
    document.getElementById("computer-next-move").onclick = function() {
        console.log("computer next move");
        setTimeout(() => game_iteration_computer(game, false), 0);
    }
    document.getElementById("instructions").innerHTML="Click on a source cell and then on a destination cell to perform a move. Same cell = self-destruct.";
    document.getElementById("computer-all-moves").onclick = function() {
        console.log("computer all moves");
        document.getElementById('control-buttons').hidden = true;
        document.getElementById("instructions").innerHTML="Computer is playing in automatic mode.";
        setTimeout(() => game_iteration_computer(game, true), 0);
    }
    document.getElementById('control-buttons').hidden = false;
}

function cancel_move() {
    player_next_move_first_coord = undefined;
    document.getElementById("cancel-move").innerHTML="";
}

function disable_auto_reply() {
    computer_auto_reply = false;
    document.getElementById("auto-reply").innerHTML=
        "<button>Enable auto-reply</button>";
    document.getElementById("auto-reply").onclick=enable_auto_reply;
}

function enable_auto_reply() {
    computer_auto_reply = true;
    document.getElementById("auto-reply").innerHTML=
        "<button>Disable auto-reply</button>";
    document.getElementById("auto-reply").onclick=disable_auto_reply;
}

function check_winner(game, next_step) {
    let win_message = game.has_winner();
    if (win_message == undefined) {
        setTimeout(() => next_step(), 0);
    } else {
        show_winner(win_message);
    }
}

function is_game_over(game) {
    return game.has_winner() != undefined;
}

function setup_board_onclick(game) {
    window.board_onclick = function(row,col) {
        return player_next_move(game, row, col);
    }
}

function show_board(game) {
    let game_html = game.html_board_string("board","window.board_onclick");
    document.getElementById("board").innerHTML=game_html;
}

function show_damage_table(game) {
    let table_html = game.damage_table_string("from / to");
    document.getElementById("damage-table").innerHTML=table_html;
}

function show_repair_table(game) {
    let table_html = game.repair_table_string("from / to");
    document.getElementById("repair-table").innerHTML=table_html;
}

function show_info(game) {
    let game_info = game.info_string();
    document.getElementById("info").innerText=game_info;
}

function show_stats(stats) {
    document.getElementById("stats").innerText=stats;
}

function show_winner(win_message) {
    show_stats(win_message);
    for (let element of document.getElementsByClassName("board_info_moves")) {
        element.innerText=win_message;
    }
}

function game_iteration_computer(game, auto) {
    if (is_game_over(game)) {
        let message = "Game is already finished!";
        show_stats(message);
        console.log(message);
        return;
    }
    let stats = game.computer_play_turn();
    show_stats(stats);
    show_info(game);
    show_board(game);
    check_winner(game, function() {
        if (auto) {
            game_iteration_computer(game, auto);
        }
    });
}

function coord_string(coord) {
    return Game.display_coord(coord[0],coord[1]);
}

function options_setup(game) {
    function set_heuristic() {
        var heuristic = document.querySelector('input[name = heuristic]:checked').value;
        console.log("Heuristic: " + heuristic);
        switch(heuristic) {
            case "simple1": 
                game.set_heuristics_simple1();
                break;
            case "simple2":
                game.set_heuristics_simple2();
                break;
            default:
                game.set_heuristics_default();
          }
        game.set_heuristic
    };
    document.getElementById("heuristics").onclick = set_heuristic;
    set_heuristic();
    function auto_adjust_max_depth() {
        let auto = document.getElementById("auto-depth").checked;
        console.log("Auto-depth: " + auto);
        game.auto_adjust_max_depth(auto);
        if (!auto) {
            set_max_depth();
        }
    }
    document.getElementById("auto-depth").addEventListener('change', auto_adjust_max_depth);
    auto_adjust_max_depth();
    function set_max_depth() {
        let str = document.getElementById("max-depth").value;
        let number = parseInt(str, 10);
        console.log("Max-depth: " + number);
        game.set_max_depth(number);
    }
    document.getElementById("max-depth").addEventListener('change', set_max_depth);
    set_max_depth();
    function set_max_seconds() {
        let str = document.getElementById("max-seconds").value;
        let number = parseInt(str, 10);
        console.log("Max-seconds: " + number);
        game.set_max_seconds(number);
    }
    document.getElementById("max-seconds").addEventListener('change', set_max_seconds);
    set_max_seconds();
}
