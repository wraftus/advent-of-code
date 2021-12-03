import numpy as np

binary_to_dec = lambda bits : sum([bits[-(idx+1)]*(2**idx) for idx in range(len(bits))])
def puzzle1(binaries):
    gamma = [0 for _ in binaries[0]]
    for binary in binaries:
        for bit_idx in range(len(binary)):
            gamma[bit_idx] += int(binary[bit_idx])
    gamma = [(1 if count >= len(binaries)/2 else 0) for count in gamma]
    eps = [1 - bit for bit in gamma]
    gamma, eps = binary_to_dec(gamma), binary_to_dec(eps)
    return gamma * eps

def puzzle2(binaries):
    bit_idx = 0
    ox_binaries = binaries.copy()
    while len(ox_binaries) > 1:
        mcb = 0
        for binary in ox_binaries:
            mcb += int(binary[bit_idx])
        mcb = (1 if mcb >= len(ox_binaries)/2 else 0)

        ox_binaries = [binary for binary in ox_binaries if int(binary[bit_idx]) == mcb]
        bit_idx += 1
    ox_binary = [int(bit) for bit in ox_binaries[0]]
    ox_rating = binary_to_dec(ox_binary)
    
    bit_idx = 0
    co2_binaries = binaries.copy()
    while len(co2_binaries) > 1:
        lcb = 0
        for binary in co2_binaries:
            lcb += 1 - int(binary[bit_idx])
        lcb = (1 if lcb > len(co2_binaries)/2 else 0)

        co2_binaries = [binary for binary in co2_binaries if int(binary[bit_idx]) == lcb]
        bit_idx += 1
    c02_binary = [int(bit) for bit in co2_binaries[0]]
    co2_rating = binary_to_dec(c02_binary)

    return ox_rating * co2_rating

if __name__ == "__main__":
    with open('data/day3.data', 'r') as data:
        binaries = [line.strip() for line in data.readlines()]

    print(puzzle1(binaries))
    print(puzzle2(binaries))