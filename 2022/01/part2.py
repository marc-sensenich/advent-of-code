import fileinput

def main():
    calories = 0
    calorie_tracking = []

    with fileinput.input() as fp:
        for line in fp:
            if line.strip():
                calories += int(line)
            else:
                calorie_tracking.append(calories)
                calories = 0
    
    print(sum(sorted(calorie_tracking, reverse=True)[0:3]))

if __name__ == "__main__":
    main()