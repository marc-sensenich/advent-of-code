import fileinput

def shape_score(your_throw):
    _shape_scores = {
        "X": 1,
        "Y": 2,
        "Z": 3,
    } 
    
    return _shape_scores.get(your_throw, 0)


def round_score(opponent_throw, your_throw):
    _round_score = shape_score(your_throw)
    outcomes = {
        # rock
        "X": {
            "A": 3,
            "B": 0,
            "C": 6,
        },
        # Paper
        "Y": {
            "A": 6,
            "B": 3,
            "C": 0,
        },
        # Scissors
        "Z": {
            "A": 0,
            "B": 6,
            "C": 3,
        },
    }

    _round_score += outcomes.get(your_throw, {}).get(opponent_throw, 0)

    return _round_score


def main():
    total_score = 0
    with fileinput.input() as fp:
        for line in fp:
            split_line = line.split()
            opponent_throw = split_line[0]
            your_throw = split_line[1]

            score = round_score(opponent_throw, your_throw)

            total_score += score

    print(total_score)

if __name__ == "__main__":
    main()