document.getElementById('submit-1').onclick = function() {
    validatePart1();
};

function validatePart1() {
    //get given input
    valid = '1234';
    var answer = document.getElementById('answer-1').value;

    if (answer != '1234') { 
        document.getElementById("error-1").style.display = 'block';
        document.getElementById("error-1").innerHTML = "That's not the right answer. If you're stuck, make sure you're using the full input data; you can ask for hints on the discord event. (You guessed: " + answer + ").";
        return;
    }

    document.getElementById("data-1").style.display = 'none';
    document.getElementById("error-1").style.display = 'none';
    document.getElementById('inputs-1').style.display = 'none';
    document.getElementById('success-part-1').style.display = 'block';
    document.getElementById('part-2').style.display = 'block';
}

document.getElementById('submit-2').onclick = function() {
    validatePart2();
};

function validatePart2() {
    valid = "4321";
    var answer = document.getElementById('answer-2').value;

    if (answer != '4321') {
        document.getElementById("error-2").style.display = 'block';
        document.getElementById("error-2").innerHTML = "That's not the right answer. If you're stuck, make sure you're using the full input data; you can ask for hints on the discord event. (You guessed: " + answer + ").";
        return;
    }

    document.getElementById("data-2").style.display = 'none';
    document.getElementById("error-2").style.display = 'none';
    document.getElementById('inputs-2').style.display = 'none';
    document.getElementById('success-part-2').style.display = 'block';
    document.getElementById("data-3").style.display = 'block';
}