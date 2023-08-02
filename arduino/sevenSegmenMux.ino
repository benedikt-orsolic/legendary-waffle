/*******************************************************************************                                
*
*	File name: sevenSegmentMux.ino
*
*	Author: Benedikt Orsolic
*
*	Purpouse: Prikaz dekadskog broja pomocu 7-segmentnog pokazivaca
*
*	Usage: 7-segmentni pokazivac se spoji kako je definirano u poljima:
*          
*          dataBus oznacava izvode 7-segmetnog pokazivaca
*                  indexi od 0->6 redom oznacavaju izvode 
*                  a, b, c, d, e, f, g
*
*          ctrlBus oznacava zajednicki izvod pojedinog decimalnog mjesta
*                  indexi redom oznacavaju znamenke indexima 0->n redom
*                  gdje 0 oznacava znamenku najvece tezine a n jedinicu.
*	
*
*******************************************************************************/










/*------------------------------------------------------------------------------
	
	Secion: Global data structure

------------------------------------------------------------------------------*/
int numMap[10][7] = {	
				{ 1,1,1,1,1,1,0 },	// 0
				{ 0,1,1,0,0,0,0 },	// 1
				{ 1,1,0,1,1,0,1 },	// 2
				{ 1,1,1,1,0,0,1 },	// 3
				{ 0,1,1,0,0,1,1 },	// 4
				{ 1,0,1,1,0,1,1 },	// 5
				{ 1,0,1,1,1,1,1 },	// 6
				{ 1,1,1,0,0,0,0 },	// 7
				{ 1,1,1,1,1,1,1 },	// 8
				{ 1,1,1,0,0,1,1 }	// 9
			};

int dataBus[] = {2,3,4,5,6,7,8};
int ctrlBus[] = {9,10,11,12};
int nDigits = 4;




















/*------------------------------------------------------------------------------
	
	Secion: Main program

------------------------------------------------------------------------------*/






/*   Podesavanje izlaza microupravljaca.   */
void setup(){
	for(int i = 0; i<7; ++i) pinMode( dataBus[i], OUTPUT);
	for(int i = 0; i<4; ++i) pinMode( ctrlBus[i], OUTPUT); 
}










/*   Ispisuje brojeve od 0 do 1000   */
void loop(){
	
	for( int i = 0; i<1000; ++i ) {
		for( int j = 0; j < 20; ++j ){
			printNum( i );
			delay( 1 );
		}
	}
    
}










/*   Prikazuje broj n.   */
void printNum( int n ){
	for( int i = nDigits - 1; i>= 0; --i ){
		writeDigit( ctrlBus[i], n%10 );
		n /= 10;
	}
}// End printNum( n )










/*   Prikazuje znamenku 4ms.   */
void writeDigit( int d, int n ){
	
	for( int i = 0; i<7; ++i) digitalWrite( dataBus[i], numMap[n][i] );
	digitalWrite( d, LOW );
	delay( 4 );
	digitalWrite( d, HIGH );
}

















