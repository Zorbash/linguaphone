#include <string.h>

#define MIN3(a, b, c) ((a) < (b) ? ((a) < (c) ? (a) : (c)) : ((b) < (c) ? (b) : (c)))

unsigned int levenshtein(char *string1, char *string2) {
  unsigned int string1_length, string2_length;

  string1_length = strlen(string1);
  string2_length = strlen(string2);

  unsigned int column[string1_length];
  unsigned int i, j, old, last;

  for (i = 1; i <= string1_length; i++)
    column[i] = i;

  for (j = 1; j <= string2_length; j++) {
    column[0] = j;

    for(i = 1, last = j - 1; i <= string1_length; i++) {
      old = column[i];
      column[i] = MIN3(column[i] + 1, column[i - 1] + 1, last +
          (string1[i - 1] == string2[j - 1] ? 0 : 1));
      last = old;
    }
  }
  return(column[string1_length]);
}
