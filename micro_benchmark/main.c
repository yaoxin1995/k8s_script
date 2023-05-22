#include <stdio.h>
#include <stdint.h>
#include <unistd.h>
#include <time.h>
#include <stdlib.h>



#define ARRAY_LEN (unsigned long)(1024*1024*100)

// #define TEST_TIME (unsigned long)(50000000)


char array[ARRAY_LEN] = {[ 0 ... (1) ] = 'a'} ;

static unsigned long long  NANOS_PER_SEC = 1000000000;



unsigned long long to_ns(struct timespec time) {
   unsigned long long  ns = (unsigned long long)time.tv_sec * NANOS_PER_SEC + (unsigned long long)time.tv_nsec;

   return ns;
}



int get_random() {
    srand(time(NULL));                      // this line is necessary
    int random_number = rand() % ARRAY_LEN;

    return random_number;
}

int main() {
   struct timespec start, end;
   clockid_t clk_id = CLOCK_MONOTONIC;  // CLOCK_REALTIME CLOCK_BOOTTIME CLOCK_PROCESS_CPUTIME_ID



   printf("Hello, World!\n");

   int index = 0;
   int result = clock_gettime(clk_id, &start);
   for(int i = 0; i < ARRAY_LEN; i++) {
      int index = get_random();
      // char a = array[index];
      // printf("%i !\n", index);
      array[index] = 'b';

      // if (index > ARRAY_LEN)
      //    index = index % ARRAY_LEN;

      // printf("a %c\n", a);
   }
   result = clock_gettime(clk_id, &end);

   unsigned long long  start_ns = to_ns(start);
   unsigned long long  end_ns = to_ns(end);

   unsigned long long  duration = end_ns - start_ns;


   printf("duration %ju\n", (uintmax_t)duration);


   // pause ();

   return 0;
}