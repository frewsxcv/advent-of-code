<?php

$handle = fopen('input', 'r');

$inputArr = [];
while (($line = fgets($handle)) !== false) {
    array_push($inputArr, +$line);
}
fclose($handle);

foreach ($inputArr as $x) {
    foreach ($inputArr as $y) {
        if ($x + $y == 2020) {
            echo "Part 1 answer: ", $x * $y, "\n";
            break 2;
        }
    }
}

foreach ($inputArr as $x) {
    foreach ($inputArr as $y) {
        foreach ($inputArr as $z) {
            if ($x + $y + $z == 2020) {
                echo "Part 2 answer: ", $x * $y * $z, "\n";
                break 3;
            }
        }
    }
}
