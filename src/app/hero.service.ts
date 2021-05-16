import { Injectable } from '@angular/core';
import { Observable, of } from 'rxjs';
import { catchError, map, tap } from 'rxjs/operators';
import { MessagesService } from './messages.service';

import { Hero } from './hero';
import { HEROS } from './mock-heroes';
import { HttpClient, HttpHeaders } from '@angular/common/http';

@Injectable({
  providedIn: 'root'
})
export class HeroService {

  private heroesUrl = 'api/heroes';  // URL to web api

  constructor(
    private http: HttpClient,
    private messagesService: MessagesService) { }

  /** GET heroes from the server */
  getHeroes(): Observable<Hero[]> {
    return this.http.get<Hero[]>(this.heroesUrl).pipe(
      tap( _ => this.log('fetch heroes')),
      catchError(this.handleError<Hero[]>('getHeroes', []))
    );;
  }

  getHero(id: number): Observable<Hero> {
    const heroUrl: string = `${this.heroesUrl}/${id}`;
    return this.http.get<Hero>(heroUrl).pipe(
      tap( _ => this.log(`fetched hero id=${id}`),
      catchError(this.handleError<Hero>(`getHero id=${id}`))
    ));

    this.messagesService.add(`HeroService: hero is fetched id=${id}`);
    const hero = HEROS.find(h => h.id == id) as Hero;
    return of(hero);
  }

  /** Log a HeroService message with the MessageService */
  private log(message: string) {
    this.messagesService.add(`HeroService: ${message}`);
  }

    /**
   * Handle Http operation that failed.
   * Let the app continue.
   * @param operation - name of the operation that failed
   * @param result - optional value to return as the observable result
   */
  private handleError<T>(operation = 'operation', result?: T) {
    return (error: any): Observable<T> => {

      // TODO: send the error to remote logging infrastructure
      console.error(error); // log to console instead

      // TODO: better job of transforming error for user consumption
      this.log(`${operation} failed: ${error.message}`);

      // Let the app keep running by returning an empty result.
      return of(result as T);
    };
  }
  
}
