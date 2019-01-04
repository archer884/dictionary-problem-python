classes_by_teacher = {}

with open('classes.csv') as input:
    for line in input.readlines():
        parts = line.split(',')
        teacher = parts[1]
        if teacher not in classes_by_teacher:
            classes_by_teacher[teacher] = 1
        else:
            classes_by_teacher[teacher] += 1

for teacher, classes in classes_by_teacher.items():
    print(f'{teacher}: {classes}')
        