import fileinput

def main():
    sum = 0
    max = 0

    with fileinput.input() as fp:
        for line in fp:
            if line.strip():
                sum += int(line)
            else:
                if sum > max:
                    max = sum

                sum = 0
    
    print(max)

if __name__ == "__main__":
    main()