#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

const char demo_file[] = "./demo-input.txt";
const char real_file[] = "./input.txt";
const bool use_real_data = true;

#define MAX_LINE_LENGTH 1024

void free_file_memory(char **lines, int line_count) {
  for (int i = 0; i < line_count; i++) {
    free(lines[i]);
  }
  free(lines);
}

char **read_file_by_lines(const char *filename, int *line_count) {
  FILE *file = fopen(filename, "r");
  if (file == NULL) {
    perror("Error opening file");
    return NULL;
  }

  char **lines = NULL;
  char temp_buffer[MAX_LINE_LENGTH];
  *line_count = 0;

  while (fgets(temp_buffer, MAX_LINE_LENGTH, file) != NULL) {
    // Remove newline character if present
    temp_buffer[strcspn(temp_buffer, "\n")] = 0;

    // Allocate memory for the new line and copy it
    char *line = malloc(strlen(temp_buffer) + 1);
    if (line == NULL) {
      perror("Memory allocation failed");
      free(line);
      free_file_memory(lines, *line_count);
      fclose(file);
      return NULL;
    }

    strcpy(line, temp_buffer);

    // Reallocate the array of lines to accommodate the new line
    char **new_lines = realloc(lines, (*line_count + 1) * sizeof(char *));
    if (new_lines == NULL) {
      perror("Memory reallocation failed");
      free(line);
      free_file_memory(lines, *line_count);
      fclose(file);
      return NULL;
    }
    lines = new_lines;

    lines[*line_count] = line;
    (*line_count)++;
  }

  fclose(file);
  return lines;
}

int main() {

  int line_count = 0;
  char **lines =
      read_file_by_lines(use_real_data ? real_file : demo_file, &line_count);

  if (lines == NULL) {
    return -1;
  }

  int sum = 0;

  for (int i = 0; i < line_count; i++) {
    printf("Line: %s\n", lines[i]);

    // Size of line
    int line_length = strlen(lines[i]);
    // Numbers buffer (same as line length)
    char numbers[line_length];

    for (int i = 0; i < line_length; i++) {
      numbers[i] = 0;
    }

    int numbers_index = 0;

    for (int j = 0; j < line_length; j++) {

      char current_char = lines[i][j];

      if (!('0' <= current_char && current_char <= '9')) {
        continue;
      }

      numbers[numbers_index] = lines[i][j];
      numbers_index++;
    }

    free(lines[i]);

    // Sum the first (multiple of 10) and last digit (multiple of 1)
    sum += (numbers[0] - '0') * 10 + (numbers[numbers_index - 1] - '0');
  }

  free(lines);
  printf("Sum: %d\n", sum);

  return 0;
}
