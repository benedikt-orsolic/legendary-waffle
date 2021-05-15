import { Injectable } from '@angular/core';
import { Observable, of } from 'rxjs';
import { MessagesService } from './messages.service';

import { Hero } from './hero';
import { HEROS } from './mock-heroes';

@Injectable({
  providedIn: 'root'
})
export class HeroService {

  constructor(private messagesService: MessagesService) { }

  getHeroes(): Observable<Hero[]> {
    const heroes = of(HEROS);
    this.messagesService.add('HeroService: heroes are fetched');
    return heroes;
  }

  getHero(id: number): Observable<Hero> {
    this.messagesService.add(`HeroService: hero is fetched id=${id}`);
    const hero = HEROS.find(h => h.id == id) as Hero;
    return of(hero);
  }
}
