import math

def puzzle1(digit_rows):
    digit_count = 0
    for digit_row in digit_rows:
        for digit in digit_row['right']:
            if len(digit) == 2: digit_count += 1 # 1
            if len(digit) == 4: digit_count += 1 # 4
            if len(digit) == 3: digit_count += 1 # 7
            if len(digit) == 7: digit_count += 1 # 8
    return digit_count

def puzzle2(digit_rows):
    digit_sum = 0
    all_letters = [letter for letter in 'abcdefg']
    for digit_row in digit_rows:
        len_five_masks, len_six_masks = set({}), set({})
        for digit in digit_row['left']:
            digit_mask = 0
            for letter in digit: digit_mask |= 2**(ord(letter) - ord('a'))
           
            if len(digit) == 2:   ones_mask = digit_mask #1
            elif len(digit) == 4: fours_mask = digit_mask #4
            elif len(digit) == 3: sevens_mask = digit_mask #7
            elif len(digit) == 5: len_five_masks.add(digit_mask)
            elif len(digit) == 6: len_six_masks.add(digit_mask)
        
        letter_masks = {}
        letter_masks['a'] = sevens_mask & ~ones_mask

        for mask in len_six_masks: # 0, 6, or 9
            if mask & fours_mask == fours_mask: # must be 9
                letter_masks['g'] = mask & ~(fours_mask | letter_masks['a'])
                break

        letter_masks['e'] = (2**7-1) & ~(fours_mask | letter_masks['a'] | letter_masks['g'])
        
        for mask in len_five_masks: # 2, 3, or 5
            if mask & ones_mask == ones_mask: # must be 3
                letter_masks['d'] = mask & ~(ones_mask | letter_masks['a'] | letter_masks['g'])
                break

        for mask in len_six_masks: # 0, 6, or 9
            if mask & fours_mask == fours_mask: continue # must be 9
            if mask & ones_mask == ones_mask: # must be 0
                letter_masks['b'] = mask & ~(ones_mask | letter_masks['a'] | letter_masks['g'] | letter_masks['e'])
        
        for mask in len_five_masks: # 2, 3, or 5
            if mask & ones_mask == ones_mask: continue # must be 3
            if mask & letter_masks['e'] == letter_masks['e']: # must be 2
                letter_masks['c'] = mask & ~(letter_masks['a'] | letter_masks['d'] | letter_masks['e'] | letter_masks['g'])
                break
        letter_masks['f'] = ones_mask & ~letter_masks['c']

        letter_mapping = {}
        for letter, mask in letter_masks.items():
            if (math.log2(mask) % 1) != 0: raise Exception()
            mask_letter = chr(round(math.log2(mask)) + ord('a'))
            letter_mapping[mask_letter] = letter

        full_num = 0
        letters_number = ['abcefg', 'cf', 'acdeg', 'acdfg', 'bcdf', 'abdfg', 'abdefg', 'acf', 'abcdefg', 'abcdfg']
        for digit in digit_row['right']:
            (mapped_letters := [letter_mapping[letter] for letter in digit]).sort()
            full_num = full_num*10 + letters_number.index(''.join(mapped_letters))
        digit_sum += full_num
    return digit_sum


if __name__ == "__main__":
    with open('data/day8.data', 'r') as data:
        data_rows = [line.strip() for line in data.readlines()]
    data_rows = [row.split('|') for row in data_rows]
    digit_rows = [{'left': left.split(), 'right': right.split()} for left, right in data_rows]
    
    print(puzzle1(digit_rows))
    print(puzzle2(digit_rows))