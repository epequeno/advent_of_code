with open("input.txt") as f:
    data = f.read().splitlines()

def part_two(data):
    oxy = filter(data)
    co2 = filter(data, is_oxy=False)
    print(int(oxy, 2) * int(co2, 2))
    
        

def filter(data, is_oxy=True):
    candidates = data
    ix = 0
    while len(candidates) > 1:
        group = [c[ix] for c in candidates]
        if group.count('0') > group.count('1'):
            target = '0' if is_oxy else '1'
        elif group.count('1') > group.count('0'):
            target = '1' if is_oxy else '0'
        else:
            target = '1' if is_oxy else '0'
        candidates = [c for c in candidates if c[ix] == target]
        ix += 1
        if ix > len(data[0]):
            break
    return candidates[0]

part_two(data)