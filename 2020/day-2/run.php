<?php

function getInputLines(): array {
    $handle = fopen('input', 'r');

    $inputLines = [];
    while (($line = fgets($handle)) !== false) {
        array_push($inputLines, $line);
    }
    fclose($handle);

    return $inputLines;
}

$total = 0;
foreach (getInputLines() as $line) {
    $tokens = explode(" ", $line);
    $lowerBound = +explode("-", $tokens[0])[0];
    $upperBound = +explode("-", $tokens[0])[1];
    $char = rtrim($tokens[1], ":");
    $password = rtrim($tokens[2], "\n");
    $charCount = substr_count($password, $char);
    if ($charCount >= $lowerBound && $charCount <= $upperBound) {
        $total += 1;
    }
}
echo "Part 1: ", $total, "\n";

$total = 0;
foreach (getInputLines() as $line) {
    $tokens = explode(" ", $line);
    $pos1 = +explode("-", $tokens[0])[0];
    $pos2 = +explode("-", $tokens[0])[1];
    $char = rtrim($tokens[1], ":");
    $password = rtrim($tokens[2], "\n");
    if ($password[$pos1 - 1] == $char xor $password[$pos2 - 1] == $char) {
        $total += 1;
    }
}
echo "Part 2: ", $total, "\n";
