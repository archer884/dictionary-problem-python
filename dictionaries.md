# Dictionaries

Maps, also known as dictionaries (and usually implemented as *hashmaps*) are used to associate a set of keys with a set of values in a one-to-one relationship. Take the following example, where students names are *mapped* to their favorite thing to stick under a desk.

```text
Rupert      => Chewing gum
Sally       => Bubble gum
Solid Snake => Remote-detonated plastic explosives
```

Every *key* (in this case the student's name) is associated with a single *value,* in this case likely a product from either Wrigley's, Ford Gum & Machine, or ACME.

Maps are useful in any case where it is necessary to access a value by a key not known until runtime. Take this sample CSV file containing all the classes taught at the learning annex this fall:

```csv
1,Blake,How to train your dragon
2,Philip,World domination for dummies
3,Blake,How to train your dragon
4,Cori,Underwater basket weaving
5,Cori,World domination for dummies
6,Cori,Underwater basket weaving
7,Blake,Underwater basket weaving
8,Blake,World domination for dummies
9,Cori,World domination for dummies
10,Blake,Underwater basket weaving
11,Philip,Underwater basket weaving
12,Cori,Underwater basket weaving
13,Philip,How to train your dragon
14,Blake,How to train your dragon
15,Blake,How to train your dragon
16,Philip,Underwater basket weaving
17,Philip,Underwater basket weaving
18,Blake,World domination for dummies
19,Philip,Underwater basket weaving
20,Blake,Underwater basket weaving
```

This comma-separated values file exhibits three columns: the first is the number of the class (one through twenty), the second is the name of the instructor, and the last is the name of the class. It would be impossible to know what names appear in the file before you receive the file, so you can't create a variable named `blake` to store a number representing the number of classes Blake taught.

Instead, you would create a single dictionary meant to store the number of classes taught by *all* instructors.

```python3
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
```

For your problem, consider the following CSV file:

```csv
1,Cori,World domination for dummies,1
2,Blake,World domination for dummies,2
3,Cori,How to train your dragon,1
4,Philip,How to train your dragon,3
5,Cori,World domination for dummies,1
6,Blake,How to train your dragon,1
7,Cori,How to train your dragon,2
8,Blake,Underwater basket weaving,1
9,Cori,World domination for dummies,2
10,Philip,Underwater basket weaving,2
11,Blake,How to train your dragon,1
12,Philip,World domination for dummies,1
13,Blake,How to train your dragon,3
14,Blake,World domination for dummies,3
15,Philip,How to train your dragon,3
16,Blake,How to train your dragon,3
17,Philip,Underwater basket weaving,2
18,Cori,How to train your dragon,3
19,Blake,World domination for dummies,3
20,Philip,World domination for dummies,1
```

This file is like the other, with the exception that we have now added an "hours" column to record how long each class took. Given what you now know about dictionaries, how can you use a dictionary to find **the total time spent in each class for each teacher?**
