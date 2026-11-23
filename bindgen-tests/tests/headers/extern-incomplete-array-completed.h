// An incomplete array type completed by a later redeclaration.
extern int array[];
extern int array[3];

extern int matrix[][2];
extern int matrix[4][2];

extern int tentative[];
int tentative[6];

// A later redeclaration without the bound keeps the completed type.
extern int other[5];
extern int other[];

// Without a completing redeclaration the array stays incomplete.
extern int incomplete[];
