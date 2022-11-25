/**
 * -------------
 * CHEATERS
 * WON'T
 * RECEIVE
 * ANY
 * GIFT
 * THIS
 * YEAR.
 *
 * 👁️👄👁️
 * -------------
 */

let discordEventUrl = "#"; //TODO change this
let errorPrompt = `That's not the right answer. If you're stuck, make sure you're using the full input data; you can ask for hints on the <a class="discord" href=${discordEventUrl} target="_blank"> discord event</a>.`;
let requiredInput = "Uh-oh... we need something to make the answer-o-tron work!";

document.getElementById('submit-1').onclick = function() {
    validatePart1();
};

document.getElementById('submit-2').onclick = function() {
    validatePart2();
};

function validatePart1() {
    valid = '1234'; //TODO change this
    var answer = document.getElementById('answer-1').value;

    if (answer == '') {
        document.getElementById("error-1").style.display = 'block';
        document.getElementById("error-1").innerHTML = requiredInput;
        return;
    }

    if (answer != '1234') {
        document.getElementById("error-1").style.display = 'block';
        document.getElementById("error-1").innerHTML = errorPrompt + `<br><br>(You guessed: <em>${answer}</em>)`;
        return;
    }

    document.getElementById("puzzle-input-1").style.display = 'none';
    document.getElementById("error-1").style.display = 'none';
    document.getElementById('input-group-1').style.display = 'none';

    document.getElementById('success-msg-part-1').style.display = 'block';
    document.getElementById('part-2').style.display = 'block';


    fathom.trackGoal('JVGWB2TP', 0);
}

function validatePart2() {
    valid = "4321"; //TODO change this
    var answer = document.getElementById('answer-2').value;

    if (answer == '') {
        document.getElementById("error-2").style.display = 'block';
        document.getElementById("error-2").innerHTML = requiredInput;
        return;
    }

    if (answer != '4321') {
        document.getElementById("error-2").style.display = 'block';
        document.getElementById("error-2").innerHTML = errorPrompt + `<br><br>(You guessed: <em>${answer}</em>)`
        return;
    }

    document.getElementById("puzzle-input-2").style.display = 'none';
    document.getElementById("error-2").style.display = 'none';
    document.getElementById('input-group-2').style.display = 'none';

    document.getElementById('success-msg-part-2').style.display = 'block';
    document.getElementById("puzzle-input-all").style.display = 'block';

    fathom.trackGoal('FSMQKF95', 0);
}

function elfQuotes() {
    let quotes = [
        "I'm not sure what you're trying to do, but I'm pretty sure it's not going to work.",
        "Feeling stuck? Try asking for hints on the discord event!",
        "Jingle bell, jingle bell, jingle bell rock",
        "Snowin' and blowin' up bushels of fun",
        "Sharing is caring, so share this event with your friends!",
        "You're doing great! Keep it up!",
        "What's the best way to spread Christmas cheer? Singing loud for all to hear!",
        "You're a mean one, Mr. Grinch",
        "Come and shares us your thoughts on the discord event.",
    ]

    document.getElementById("text").style.display = 'block';
    document.getElementById("text").innerHTML = quotes[Math.floor(Math.random() * quotes.length)];
    setTimeout(function(){document.getElementById("text").style.display = 'none'}, 5000);
    setTimeout(elfQuotes, 20000);
};

setTimeout(elfQuotes, 20000);