import init, { Game } from "./pkg/ai_wargame_web.js";

init().then(() => {
    new_game();
});

function new_game() {
    let game = new Game();
    game.auto_reply = true;
    game.first_coord = undefined;
    game.aborted = false;
    setup_board_onclick(game);
    show_board(game);
    enable_auto_reply(game);
    options_setup(game);
    show_damage_table(game);
    show_repair_table(game);
    setup_buttons(game);
    show_result("");
    show_info(undefined);
}

function restart_game(game) {
    console.log("restarting game...");
    game.aborted = true;
    new_game();
}

function player_next_move(game, row, col) {
    console.log("Clicked at row:",row," col:",col);
    if (is_game_over(game)) {
        let message = "Game is already finished!";
        show_result(message);
        console.log(message);
        return;
    }
    if (game.first_coord == undefined ) {
        game.first_coord = [row,col];
        document.getElementById("cancel-move").innerHTML=
            "<button>Cancel move from "+
                coord_string(game.first_coord)
            +"</button>";
        document.getElementById("cancel-move").onclick = function() {
            cancel_move(game);
        };
    } else {
        let to = [row,col];
        let from = game.first_coord;
        cancel_move(game);
        let result = game.player_play_turn(from[0],from[1],to[0],to[1]);
        if (result != undefined) {
            post_move(game,from,to);
            show_board(game);
            show_result(result);
            highlight_move(from, to);
            check_winner(game, function() {
                if (game.auto_reply) {
                    game_iteration_computer(game, undefined)
                }
            });
        } else {
            let message = "Invalid move!";
            show_winner(message);
            show_result(message);            
        }
    }
}

function setup_buttons(game) {
    document.getElementById("restart").onclick = function() {
        restart_game(game);
    }
    document.getElementById("computer-next-move").onclick = function() {
        console.log("computer next move");
        setTimeout(() => game_iteration_computer(game, undefined), 0);
    }
    document.getElementById("instructions").innerHTML="Click on a source cell and then on a destination cell to perform a move. Same cell = self-destruct.";
    document.getElementById("computer-all-moves").onclick = function() {
        console.log("computer all moves");
        document.getElementById('control-buttons').hidden = true;
        document.getElementById("instructions").innerHTML="Computer is playing in automatic mode.";
        setTimeout(() => game_iteration_computer(game, game_iteration_computer), 0);
    }
    document.getElementById('control-buttons').hidden = false;
    document.getElementById("broker-next-move").onclick = function() {
        console.log("broker next move");
        document.getElementById("broker-next-move").disabled = true;
        setTimeout(() => broker_next_move(game), 0);
    }
}

function cancel_move(game) {
    game.first_coord = undefined;
    document.getElementById("cancel-move").innerHTML="";
}

function disable_auto_reply(game) {
    game.auto_reply = false;
    document.getElementById("auto-reply").innerHTML=
        "<button>Enable auto-reply</button>";
    document.getElementById("auto-reply").onclick = function() {
        enable_auto_reply(game);
    };
}

function enable_auto_reply(game) {
    game.auto_reply = true;
    document.getElementById("auto-reply").innerHTML=
        "<button>Disable auto-reply</button>";
    document.getElementById("auto-reply").onclick = function() {
        disable_auto_reply(game);
    }
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
    let game_html = game.html_board_string("board","board-table","window.board_onclick");
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
    let info = "";
    if (game != undefined) {
        info = game.info_string();
    }
    document.getElementById("info").innerText=info;
}

function show_result(result) {
    document.getElementById("result").innerText=result;
}

function show_winner(win_message) {
    show_result(win_message);
    for (let element of document.getElementsByClassName("board_info_moves")) {
        element.innerText=win_message;
    }
}

function game_iteration_computer(game, next_step) {
    if (is_game_over(game)) {
        let message = "Game is already finished!";
        show_result(message);
        console.log(message);
        return;
    }
    let result = game.computer_play_turn();
    console.log("Move result from computer: ", result);
    if (game.aborted) {
        console.log("aborting game...")
        return;
    }
    show_result(result.string);
    show_info(game);
    show_board(game);
    if (result.coords != undefined) {
        let from = [result.coords.from.row, result.coords.from.col];
        let to = [result.coords.to.row, result.coords.to.col];
        post_move(game, from, to);
        highlight_move(from, to);
    }
    check_winner(game, function() {
        if (next_step != undefined) {
            next_step(game, next_step);
        }
    });
}

function highlight_move(from, to) {
    document.getElementById("board-table-"+from[0]+"-"+from[1]).style.backgroundColor = "#e2dcd4";
    document.getElementById("board-table-"+to[0]+"-"+to[1]).style.backgroundColor = "#e9e7e2";
}

function coord_string(coord) {
    return Game.display_coord(coord[0],coord[1]);
}

function options_setup(game) {
    function set_heuristic() {
        var heuristic = document.querySelector('input[name = heuristic]:checked').value;
        console.log("Heuristic: " + heuristic);
        switch(heuristic) {
            case "e1": 
                game.set_heuristics_e1();
                break;
            case "e2":
                game.set_heuristics_e2();
                break;
            default:
                game.set_heuristics_e3e4();
        }
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
    function set_max_moves() {
        let str = document.getElementById("max-moves").value;
        let number = parseInt(str, 10);
        console.log("Max-moves: " + number);
        game.set_max_moves(number);
    }
    document.getElementById("max-moves").addEventListener('change', set_max_moves);
    set_max_moves();
    function rand_traversal() {
        let random = document.getElementById("rand-traversal").checked;
        console.log("Random traversal: " + random);
        game.set_rand_traversal(random);
    }
    document.getElementById("rand-traversal").addEventListener('change', rand_traversal);
    rand_traversal();
    function alpha_beta() {
        let ab = document.getElementById("alpha-beta").checked;
        console.log("Alpha-Beta: " + ab);
        game.set_alpha_beta(ab);
    }
    document.getElementById("alpha-beta").addEventListener('change', alpha_beta);
    alpha_beta();
}

function post_move(game, from, to) {
    let data = {
        from: { row: from[0], col: from[1] },
        to: { row: to[0], col: to[1] },
        turn: game.moves_played(),
    };
    let broker_url = document.getElementById("broker-url").value;
    if (broker_url.length > 0) {
        let broker_username = document.getElementById("broker-username").value;
        let broker_password = document.getElementById("broker-password").value;
        console.log("Broker URL: ",broker_url);
        console.log("Broker Username: ", broker_username);
        // console.log("Broker Password: ", broker_password);
        post_json_data(broker_url, data, broker_username, broker_password);
    }
}

function post_json_data(url, data, username, password) {
    var xhr = new XMLHttpRequest();
    xhr.open("POST", url, true);
    xhr.setRequestHeader("Content-Type", "application/json");
    if (username.length > 0) {
        xhr.setRequestHeader("Authorization", "Basic " + btoa(username+":"+password));
    }
    xhr.onreadystatechange = function () {
        if (xhr.readyState === 4 && xhr.status === 200) {
            var json_reply = JSON.parse(xhr.responseText);
            console.log("Got reply from ", url, ": ",json_reply);
        }
    };
    var data_json = JSON.stringify(data);
    xhr.send(data_json);
    console.log("Sent to ", url, ": ",data_json);
}

function get_move(game, next_step) {
    let broker_url = document.getElementById("broker-url").value;
    if (broker_url.length > 0) {
        let broker_username = document.getElementById("broker-username").value;
        let broker_password = document.getElementById("broker-password").value;
        console.log("Broker URL: ",broker_url);
        console.log("Broker Username: ", broker_username);
        // console.log("Broker Password: ", broker_password);
        get_json_data(game, broker_url, next_step, broker_username, broker_password);
    }
}

function get_json_data(game, url, next_step, username, password, retries=10) {
    var next_turn = game.moves_played() + 1;
    var xhr = new XMLHttpRequest();
    xhr.open("GET", url, true);
    xhr.setRequestHeader("Content-Type", "application/json");
    if (username.length > 0) {
        xhr.setRequestHeader("Authorization", "Basic " + btoa(username+":"+password));
    }
    xhr.onreadystatechange = function () {
        if (xhr.readyState === 4 && xhr.status === 200) {
            var json_reply = JSON.parse(xhr.responseText);
            console.log("Got reply from ", url, ": ",json_reply);
            var data = json_reply.data;
            if (data != undefined && data.turn == next_turn) {
                next_step(
                    [data.from.row,data.from.col], 
                    [data.to.row, data.to.col]
                );
            } else {
                if (game.aborted) {
                    console.log("aborting game...")
                    return;
                }
                if (retries > 0) {
                    console.log("Broker not ready or invalid data: retry in 500ms");
                    setTimeout(() => get_json_data(game, url, next_step, username, password, retries-1), 500);
                } else {
                    console.log("Aborting on broker after too many retries");
                    document.getElementById("broker-next-move").disabled = false;
                }
            }
        }
    };
    xhr.send();
    console.log("Requested data from ", url);
}

function broker_next_move(game) {
    if (is_game_over(game)) {
        let message = "Game is already finished!";
        show_result(message);
        console.log(message);
        return;
    }
    get_move(game,function(from,to) {
        let result = game.player_play_turn(from[0],from[1],to[0],to[1]);
        if (result != undefined) {
            show_board(game);
            show_result(result);
            highlight_move(from, to);
            check_winner(game, function() {
                if (game.auto_reply) {
                    game_iteration_computer(game, undefined)
                }
                document.getElementById("broker-next-move").disabled = false;
            });
        } else {
            let message = "Invalid move!";
            show_winner(message);
            show_result(message);            
        }
    });
}
