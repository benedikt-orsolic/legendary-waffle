/* Zadatak 1 
#include <stdio.h>

int main( void ){
	int w, c;
	
	do{ 
		scanf("%d", w); 
	} while (w <= 0);
	
	
	for (c = 1; c <= w; c++)
		if (c % 2 == 1)
			printf("%d\n", c);
	return 0;
} */










/* Zadatak 2	
#include <stdio.h>

int main(void){
	
	int min = 5, max = 25;
	int n, i, j;
	
	
	
	
	
	
	
	do{
		scanf("%d", &n);
	} while ( n < min || max < n );
	
	
	
	
	
	
	for(i = 0; i<n ; i++ ){
		for( j = 0; j<n; j++ ){
			printf("*");
		}
		printf("\n");
	}
	
	
	
	
	
	
	
	return 0;
}*/










/* Zadatak 3	
#include <stdio.h>
int main (){
	
	int A=0, B=0, C=0, D=0, E=0, F=0;
	char input;
	
	
	
	
	
	
	do {
		scanf("%c", &input);
		
		switch (input) {
			case 'A':
				A++; break;
			case 'B':
				B++; break;
			case 'C':
				C++; break;
			case 'D':
				D++; break;
			case 'E':
				E++; break;
			case 'F':
				F++; break;
		}
	} while (input != '!');
	
	
	
	
	
	
	printf("A=%d\n", A);
	printf("B=%d\n", B);
	printf("C=%d\n", C);
	printf("D=%d\n", D);
	printf("E=%d\n", E);
	printf("F=%d\n", F);
	
	return 0;
}*/










/*	Zadatak 4	
#include <stdio.h>

int main (){
	
	int n;
	
	
	
	
	
	
	
	do {
		scanf("%d", &n);
	} while (n < 0);
	
	
	
	
	
	printf("%d", n%10);
	n/=10;
	
	while ( n != 0 ) {
		printf("__%d", n%10);
		n/=10;
	}
	
	printf("\n");
	
	return 0;
}*/










/* Zadatak 5	*
#include <stdio.h>

int main (){
	
	int parnih = 0;
	int i, t;
	
	for (i = 0; i < 10; i += 1){
		
		scanf("%d", &t);
		parnih += (t%2==0);
	}
	
	printf("%d\n\n", parnih);
	return 0;
}*/










/*	Zadatak 6	
#include <stdio.h>

int main (){
	
	char a, b, c;
	
	scanf("%c%c%c", &a, &b, &c);
	
	if ( a < b && a < c ) {
		printf("%x\t%x", b-a, c-a);
	}
	if ( b < a && b < c ) {
		printf("%x\t%x", a-b, c-b);
	}
	if ( c < b && c < a ) {
		printf("%x\t%x", a-c, b-c);
	}
	
	return 0;
} */




































